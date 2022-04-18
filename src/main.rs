use utils::timer::Timer;

mod day1_not_quite_lisp;
mod day2_area;
mod day3_houses;
mod day4_mining;
mod day5_nice;
mod day6_lights;
mod day7_circuit;
mod day8_strings;
mod day9_distances;
mod day10_look_and_say;
mod day11_passwords;
mod day12_numbers;
mod day13_dinner_table;
mod day14_reindeer_olympics;
mod day15_science_for_hungry_people;
mod day16_aunt_sue;
mod day17_no_such_thing_as_too_much;
mod day18_gif_for_your_yard;
mod day19_medicine_for_rudolph;
mod day20_infinite_elves_and_houses;
mod day21_rpg_simulator_20xx;
mod day22_wizard_simulator_20xx;
mod day23_turing_lock;
mod day24_hangs_in_the_balance;
mod day25_let_it_snow;

fn main() {
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        25
    };
    println!("running day {}\n", day);
    match day {
        1 => day1_not_quite_lisp::run(),
        2 => day2_area::run(),
        3 => day3_houses::run(),
        4 => day4_mining::run(),
        5 => day5_nice::run(),
        6 => day6_lights::run(),
        7 => day7_circuit::run(),
        8 => day8_strings::run(),
        9 => day9_distances::run(),
        10 => day10_look_and_say::run(),
        11 => day11_passwords::run(),
        12 => day12_numbers::run(),
        13 => day13_dinner_table::run(),
        14 => day14_reindeer_olympics::run(),
        15 => day15_science_for_hungry_people::run(),
        16 => day16_aunt_sue::run(),
        17 => day17_no_such_thing_as_too_much::run(),
        18 => day18_gif_for_your_yard::run(),
        19 => day19_medicine_for_rudolph::run(),
        20 => day20_infinite_elves_and_houses::run(),
        21 => day21_rpg_simulator_20xx::run(),
        22 => day22_wizard_simulator_20xx::run(),
        23 => day23_turing_lock::run(),
        24 => day24_hangs_in_the_balance::run(),
        25 => day25_let_it_snow::run(),
        _ => panic!("day {} not found", day)
    }
}
