use std::str::FromStr;

type TNum = i32;

pub(crate) fn run() {
    let _input = "Hit Points: 109
Damage: 8
Armor: 2";

    let weapons = parse_items("Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0");
    let armor = parse_items("Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5");
    let rings = parse_items("Damage+1    25     1       0
Damage+2    50     2       0
Damage+3   100     3       0
Defense+1   20     0       1
Defense+2   40     0       2
Defense+3   80     0       3");

    let boss: Fighter = _input.parse().unwrap();

    let mut player = Fighter::new(100);

    let min_cost = minimize_cost(&player, &boss, &weapons, &armor, &rings, true);

    player.equip_items(&min_cost.iter().collect::<Vec<_>>());
    fight(player, boss, true);

    println!("optimized items: {:?}", min_cost);
    println!("min cost: {}", min_cost.iter().map(|i| i.cost).sum::<TNum>());

    let boss: Fighter = _input.parse().unwrap();
    let mut player = Fighter::new(100);

    let max_cost = minimize_cost(&player, &boss, &weapons, &armor, &rings, false);

    player.equip_items(&max_cost.iter().collect::<Vec<_>>());
    fight(player, boss, true);

    println!("reverse optimized items: {:?}", max_cost);
    println!("max cost: {}", max_cost.iter().map(|i| i.cost).sum::<TNum>());
}

fn minimize_cost(player: &Fighter, boss: &Fighter, weapons: &[Item], armor: &[Item], rings: &[Item], minimize_cost: bool) -> Vec<Item> {
    let possible_equipment = (0..weapons.len())
        .flat_map(|weapon_index| {
            let mut armors: Vec<Option<usize>> = (0..armor.len()).map(|i| Some(i)).collect();
            armors.push(None);
            armors.into_iter()
                .flat_map(move |armor_index| {
                    let mut ringses: Vec<Vec<usize>> = (0..rings.len()).map(|i| vec![i]).collect();
                    ringses.push(Vec::new());
                    for x in 0..rings.len() - 1 {
                        for y in x + 1..rings.len() {
                            ringses.push(vec![x, y]);
                        }
                    }
                    ringses.into_iter().map(move |ring_index| {
                        let mut equipment: Vec<&Item> = vec![&weapons[weapon_index]];
                        if let Some(armor_index) = armor_index {
                            equipment.push(&armor[armor_index]);
                        }
                        for &ring_index in ring_index.iter() {
                            equipment.push(&rings[ring_index]);
                        }
                        equipment
                    })
                })
        });
    if minimize_cost {
        possible_equipment
            .filter(|e| {
                let mut player = player.clone();
                let boss = boss.clone();
                player.equip_items(e);
                fight(player, boss, false)
            })
            .min_by(|a, b| a.iter().map(|i| i.cost).sum::<TNum>().cmp(&b.iter().map(|i| i.cost).sum::<TNum>())).unwrap().into_iter().cloned().collect()
    } else {
        possible_equipment
            .filter(|e| {
                let mut player = player.clone();
                let boss = boss.clone();
                player.equip_items(e);
                !fight(player, boss, false)
            })
            .min_by(|b, a| a.iter().map(|i| i.cost).sum::<TNum>().cmp(&b.iter().map(|i| i.cost).sum::<TNum>())).unwrap().into_iter().cloned().collect()
    }
}

fn fight(mut player: Fighter, mut boss: Fighter, print: bool) -> bool {
    if print {
        println!("player deals {} damage", player.damage - boss.armor);
        println!("boss deals {} damage", boss.damage - player.armor);
    }

    loop {
        if boss.hit_by(&player) {
            if print {
                println!("player wins!");
            }
            return true;
        }
        if print {
            println!("player hits boss down to {}", boss.hit_points);
        }
        if player.hit_by(&boss) {
            if print {
                println!("boss wins!");
            }
            return false;
        }
        if print {
            println!("boss hits player down to {}", player.hit_points);
        }
    }
}

fn parse_items(s: &'static str) -> Vec<Item> {
    s.split('\n')
        .map(|line| {
            let mut words = line.split_whitespace();
            let name = words.next().unwrap();
            let cost: TNum = words.next().unwrap().parse().unwrap();
            let damage: TNum = words.next().unwrap().parse().unwrap();
            let armor: TNum = words.next().unwrap().parse().unwrap();
            Item {
                _name: name,
                cost,
                damage,
                armor,
            }
        })
        .collect()
}

#[derive(Clone)]
struct Fighter {
    hit_points: TNum,
    damage: TNum,
    armor: TNum,
}

impl Fighter {
    pub(crate) fn new(hit_points: TNum) -> Self {
        Self {
            hit_points,
            damage: 0,
            armor: 0,
        }
    }
    pub(crate) fn hit_by(&mut self, other: &Self) -> bool {
        self.hit_points -= (other.damage - self.armor).max(1);
        self.hit_points <= 0
    }
    pub(crate) fn equip_items(&mut self, items: &[&Item]) {
        for item in items {
            self.damage += item.damage;
            self.armor += item.armor;
        }
    }
}

impl FromStr for Fighter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let hit_points: TNum = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let damage: TNum = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let armor: TNum = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        Ok(Self {
            hit_points,
            damage,
            armor,
        })
    }
}

#[derive(Debug, Clone)]
struct Item {
    _name: &'static str,
    cost: TNum,
    damage: TNum,
    armor: TNum,
}