use utils::timer::Timer;

mod area;
mod houses;
mod mining;
mod nice;
mod lights;
mod circuit;
mod strings;
mod distances;
mod look_and_say;
mod passwords;
mod numbers;
mod dinner_table;
mod reindeer_olympics;
mod science_for_hungry_people;
mod aunt_sue;
mod no_such_thing_as_too_much;
mod gif_for_your_yard;
mod medicine_for_rudolph;
mod infinite_elves_and_houses;
mod rpg_simulator_20xx;
mod wizard_simulator_20xx;
mod turing_lock;
mod hangs_in_the_balance;
mod let_it_snow;

fn main() {
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        25
    };
    match day {
        2 => area::run(),
        3 => houses::run(),
        4 => mining::run(),
        5 => nice::run(),
        6 => lights::run(),
        7 => circuit::run(),
        8 => strings::run(),
        9 => distances::run(),
        10 => look_and_say::run(),
        11 => passwords::run(),
        12 => numbers::run(),
        13 => dinner_table::run(),
        14 => reindeer_olympics::run(),
        15 => science_for_hungry_people::run(),
        16 => aunt_sue::run(),
        17 => no_such_thing_as_too_much::run(),
        18 => gif_for_your_yard::run(),
        19 => medicine_for_rudolph::run(),
        20 => infinite_elves_and_houses::run(),
        21 => rpg_simulator_20xx::run(),
        22 => wizard_simulator_20xx::run(),
        23 => turing_lock::run(),
        24 => hangs_in_the_balance::run(),
        25 => let_it_snow::run(),
        _ => panic!("day {} not found", day)
    }
}
