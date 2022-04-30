use log::*;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use enum_dispatch::enum_dispatch;

pub(crate) fn run() {
    let _input = "Hit Points: 51
Damage: 9";
    let _player = Player::new(50, 500);
    let hard_mode = true;

    let player = _player.clone();
    let boss: Boss = _input.parse().unwrap();

    let game = Game::new(player, boss, hard_mode);
    let spells = game.minimize_mana_used();

    Game::replay(&spells, _player.clone(), _input.parse().unwrap(), hard_mode);
    println!("optimal mana use is {}: {:?}", spells.iter().map(|s| s.get_cost()).sum::<i32>(), spells);
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Game {
    player: Player,
    boss: Boss,
    hard_mode: bool,
    spells_used: Vec<Spell>,
}

impl Game {
    pub(crate) fn new(player: Player, boss: Boss, hard_mode: bool) -> Self {
        Self {
            player,
            boss,
            hard_mode,
            spells_used: Default::default(),
        }
    }
    pub(crate) fn replay(spells: &[Spell], player: Player, boss: Boss, hard_mode: bool) {
        let mut game = Game::new(player, boss, hard_mode);
        for &spell in spells {
            game.play_full_turn(spell);
        }
        let winner = game.get_winner().unwrap();
        debug!("{} wins!", winner.fighter_type());
    }
    pub(crate) fn minimize_mana_used(&self) -> Vec<Spell> {
        let mut games: Vec<Self> = vec![self.clone()];
        let mut finished_games: Vec<Self> = Default::default();

        while !games.is_empty() {
            let mut continued_games = Vec::new();
            for game in games {
                for next_game in game.get_next_possible_games() {
                    if next_game.is_finished() {
                        finished_games.push(next_game);
                    } else {
                        continued_games.push(next_game);
                    }
                }
            }
            games = continued_games;
            debug!("{}/{} games finished", finished_games.len(), finished_games.len() + games.len());
        }

        let optimum = finished_games.into_iter()
            .filter(|game| game.player.hit_points > 0)
            .min_by(|a, b| {
                a.get_total_spells_cost().cmp(&b.get_total_spells_cost())
            }).unwrap();

        optimum.spells_used
    }
    fn get_next_possible_games<'a>(&'a self) -> impl Iterator<Item=Self> + 'a {
        self.player.possible_spells()
            .map(|spell| {
                let mut game = self.clone();
                game.play_full_turn(spell);
                game
            })
    }
    fn get_total_spells_cost(&self) -> i32 {
        self.spells_used.iter().map(|spell| spell.get_cost()).sum()
    }
    fn play_full_turn(&mut self, spell: Spell) {
        self.spells_used.push(spell);
        //player
        debug!("\n-- Player turn --");
        debug!("- {:?}", self.player);
        debug!("- {:?}", self.boss);
        if self.hard_mode {
            if self.player.receive_damage(1) {
                return;
            }
        }
        if self.player.start_turn(&mut self.boss) {
            return;
        }
        if self.player.cast_spell(&mut self.boss, spell) {
            return;
        }

        //boss
        debug!("\n-- Boss turn --");
        debug!("- {:?}", self.player);
        debug!("- {:?}", self.boss);
        if self.player.start_turn(&mut self.boss) {
            return;
        }
        if self.player.receive_damage(self.boss.damage) {
            return;
        }
    }
    fn is_finished(&self) -> bool {
        self.player.hit_points <= 0 || self.boss.hit_points <= 0
    }
    fn get_winner(&self) -> Option<FighterEnum> {
        if self.player.hit_points <= 0 {
            Some(self.boss.clone().into())
        } else if self.boss.hit_points <= 0 {
            Some(self.player.clone().into())
        } else {
            None
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub(crate) fn get_cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Player {
    hit_points: i32,
    mana: i32,
    shield_turns: usize,
    shield_active: bool,
    poison_turns: usize,
    recharge_turns: usize,
}

impl Fighter for Player {
    fn get_hit_points(&mut self) -> &mut i32 {
        &mut self.hit_points
    }

    fn get_armor(&self) -> i32 {
        if self.shield_active {
            7
        } else {
            0
        }
    }

    fn fighter_type(&self) -> &'static str {
        "player"
    }
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player has {} hp, shield {}, {} mana", self.hit_points, self.shield_turns > 0, self.mana)
    }
}

impl Player {
    pub(crate) fn new(hit_points: i32, mana: i32) -> Self {
        Self {
            hit_points,
            mana,
            shield_turns: 0,
            shield_active: false,
            poison_turns: 0,
            recharge_turns: 0,
        }
    }
    pub(crate) fn start_turn(&mut self, boss: &mut impl Fighter) -> bool {
        self.shield_active = if self.shield_turns > 0 {
            self.shield_turns -= 1;
            debug!("Shield's timer is now {}.", self.shield_turns);
            true
        } else {
            false
        };
        if self.recharge_turns > 0 {
            self.recharge_turns -= 1;
            debug!("Recharge provides 101 mana; its timer is now {}.", self.recharge_turns);
            self.mana += 101;
        }
        if self.poison_turns > 0 {
            self.poison_turns -= 1;
            debug!("Poison deals 3 damage; its timer is now {}.", self.poison_turns);
            if boss.receive_damage(3) {
                return true;
            }
        }

        false
    }
    pub(crate) fn possible_spells<'a>(&'a self) -> impl Iterator<Item=Spell> + 'a {
        [Spell::MagicMissile,
            Spell::Drain,
            Spell::Shield,
            Spell::Poison,
            Spell::Recharge]
            .into_iter().filter(|spell| {
            if spell.get_cost() > self.mana {
                return false;
            }
            match spell {
                Spell::Shield => {
                    if self.shield_turns > 1 {
                        return false;
                    }
                }
                Spell::Poison => {
                    if self.poison_turns > 1 {
                        return false;
                    }
                }
                Spell::Recharge => {
                    if self.recharge_turns > 1 {
                        return false;
                    }
                }
                _ => {}
            }

            true
        })
    }
    pub(crate) fn cast_spell(&mut self, boss: &mut impl Fighter, spell: Spell) -> bool {
        debug!("Player casts {:?}.", spell);
        self.mana -= spell.get_cost();
        match spell {
            Spell::MagicMissile => {
                boss.receive_damage(4)
            }
            Spell::Drain => {
                self.heal(2);
                boss.receive_damage(2)
            }
            Spell::Shield => {
                self.shield_turns = 6;
                false
            }
            Spell::Poison => {
                self.poison_turns = 6;
                false
            }
            Spell::Recharge => {
                self.recharge_turns = 5;
                false
            }
        }
    }
}

#[enum_dispatch(FighterEnum)]
trait Fighter {
    fn get_hit_points(&mut self) -> &mut i32;
    fn get_armor(&self) -> i32;
    fn receive_damage(&mut self, mut damage: i32) -> bool {
        let armor = self.get_armor();
        damage -= armor;
        damage = damage.max(1);
        debug!("{} is inflicted {} damage.", self.fighter_type(), damage);
        let hp = self.get_hit_points();
        *hp -= damage;
        *hp <= 0
    }
    fn heal(&mut self, hit_points: i32) {
        debug!("{} is healed {} hp.", self.fighter_type(), hit_points);
        let hp = self.get_hit_points();
        *hp += hit_points;
    }
    fn fighter_type(&self) -> &'static str;
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Boss {
    hit_points: i32,
    damage: i32,
}

impl Debug for Boss {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boss has {} hp", self.hit_points)
    }
}

impl FromStr for Boss {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let hit_points: i32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let damage: i32 = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        Ok(Self {
            hit_points,
            damage,
        })
    }
}

impl Fighter for Boss {
    fn get_hit_points(&mut self) -> &mut i32 {
        &mut self.hit_points
    }

    fn get_armor(&self) -> i32 {
        0
    }

    fn fighter_type(&self) -> &'static str {
        "boss"
    }
}

#[enum_dispatch]
enum FighterEnum {
    Player,
    Boss,
}