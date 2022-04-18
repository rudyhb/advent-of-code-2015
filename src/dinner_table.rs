use std::collections::{BTreeSet, HashMap};

pub fn run() {
//     let input = "Alice would gain 54 happiness units by sitting next to Bob.
// Alice would lose 79 happiness units by sitting next to Carol.
// Alice would lose 2 happiness units by sitting next to David.
// Bob would gain 83 happiness units by sitting next to Alice.
// Bob would lose 7 happiness units by sitting next to Carol.
// Bob would lose 63 happiness units by sitting next to David.
// Carol would lose 62 happiness units by sitting next to Alice.
// Carol would gain 60 happiness units by sitting next to Bob.
// Carol would gain 55 happiness units by sitting next to David.
// David would gain 46 happiness units by sitting next to Alice.
// David would lose 7 happiness units by sitting next to Bob.
// David would gain 41 happiness units by sitting next to Carol.";
    let input = get_input();

    let mut table: Table = input.into();
    table.inject_apathetic_person("Me");

    println!("optimal happiness is {}", table.optimal_happiness());
}

struct Table<'a> {
    happiness: HashMap<(&'a str, &'a str), i32>,
    people: BTreeSet<&'a str>
}

impl<'a> From<&'a str> for Table<'a> {
    fn from(s: &'a str) -> Self {
        let mut happiness: HashMap<(&'a str, &'a str), i32> = Default::default();
        let mut people: BTreeSet<&'a str> = Default::default();

        for line in s.split('\n') {
            let mut parts = line.split(' ');
            let person_a = parts.next().unwrap();
            parts.next();
            let sign = match parts.next().unwrap() {
                "gain" => 1,
                "lose" => -1,
                _ => panic!("should be gain/lose")
            };
            let amount: i32 = parts.next().unwrap().parse().unwrap();
            let person_b = parts.last().unwrap().trim_end_matches('.');

            people.insert(person_a);
            happiness.insert((person_a, person_b), sign * amount);
        }

        Self {
            happiness,
            people
        }
    }
}

impl<'a> Table<'a> {

    pub fn inject_apathetic_person(&mut self, name: &'a str) {
        let people: Vec<&'a str> = self.people.iter().copied().collect();

        self.people.insert(name);
        for person in people {
            self.happiness.insert((person, name), 0);
            self.happiness.insert((name, person), 0);
        }
    }

    fn get_happiness(&self, arrangement: &Vec<usize>) -> i32 {
        let people: Vec<&'a str> = self.people.iter().copied().collect();
        (0..arrangement.len())
            .map(|i| {
                let pair;
                if i == 0 {
                    pair = (arrangement[arrangement.len() - 1], arrangement[0]);
                } else {
                    pair = (arrangement[i - 1], arrangement[i]);
                }
                let person_a = people[pair.0];
                let person_b = people[pair.1];
                let happiness_a = *self.happiness.get(&(person_a, person_b)).unwrap();
                let happiness_b = *self.happiness.get(&(person_b, person_a)).unwrap();
                let happiness = happiness_a + happiness_b;
                // println!("happiness between {} and {} is {} + {} = {}", person_a, person_b, happiness_a, happiness_b, happiness);
                happiness
            })
            .sum()
    }

    fn _get_names(&self, arrangement: &Vec<usize>) -> String {
        let people: Vec<&'a str> = self.people.iter().copied().collect();
        arrangement.iter().map(|&i| people[i]).collect()
    }

    pub fn optimal_happiness(&self) -> i32 {
        let mut data: Vec<usize> = (0..self.people.len()).map(|i| i).collect();
        permutator::Permutation::permutation(&mut data).map(|p| {
            let happiness = Self::get_happiness(self, &p);
            println!("happiness {} for {} {:?}", happiness, Self::_get_names(self, &p), p);
            happiness
        }).max().unwrap()
    }
}

fn get_input() -> &'static str {
    "Alice would gain 2 happiness units by sitting next to Bob.
Alice would gain 26 happiness units by sitting next to Carol.
Alice would lose 82 happiness units by sitting next to David.
Alice would lose 75 happiness units by sitting next to Eric.
Alice would gain 42 happiness units by sitting next to Frank.
Alice would gain 38 happiness units by sitting next to George.
Alice would gain 39 happiness units by sitting next to Mallory.
Bob would gain 40 happiness units by sitting next to Alice.
Bob would lose 61 happiness units by sitting next to Carol.
Bob would lose 15 happiness units by sitting next to David.
Bob would gain 63 happiness units by sitting next to Eric.
Bob would gain 41 happiness units by sitting next to Frank.
Bob would gain 30 happiness units by sitting next to George.
Bob would gain 87 happiness units by sitting next to Mallory.
Carol would lose 35 happiness units by sitting next to Alice.
Carol would lose 99 happiness units by sitting next to Bob.
Carol would lose 51 happiness units by sitting next to David.
Carol would gain 95 happiness units by sitting next to Eric.
Carol would gain 90 happiness units by sitting next to Frank.
Carol would lose 16 happiness units by sitting next to George.
Carol would gain 94 happiness units by sitting next to Mallory.
David would gain 36 happiness units by sitting next to Alice.
David would lose 18 happiness units by sitting next to Bob.
David would lose 65 happiness units by sitting next to Carol.
David would lose 18 happiness units by sitting next to Eric.
David would lose 22 happiness units by sitting next to Frank.
David would gain 2 happiness units by sitting next to George.
David would gain 42 happiness units by sitting next to Mallory.
Eric would lose 65 happiness units by sitting next to Alice.
Eric would gain 24 happiness units by sitting next to Bob.
Eric would gain 100 happiness units by sitting next to Carol.
Eric would gain 51 happiness units by sitting next to David.
Eric would gain 21 happiness units by sitting next to Frank.
Eric would gain 55 happiness units by sitting next to George.
Eric would lose 44 happiness units by sitting next to Mallory.
Frank would lose 48 happiness units by sitting next to Alice.
Frank would gain 91 happiness units by sitting next to Bob.
Frank would gain 8 happiness units by sitting next to Carol.
Frank would lose 66 happiness units by sitting next to David.
Frank would gain 97 happiness units by sitting next to Eric.
Frank would lose 9 happiness units by sitting next to George.
Frank would lose 92 happiness units by sitting next to Mallory.
George would lose 44 happiness units by sitting next to Alice.
George would lose 25 happiness units by sitting next to Bob.
George would gain 17 happiness units by sitting next to Carol.
George would gain 92 happiness units by sitting next to David.
George would lose 92 happiness units by sitting next to Eric.
George would gain 18 happiness units by sitting next to Frank.
George would gain 97 happiness units by sitting next to Mallory.
Mallory would gain 92 happiness units by sitting next to Alice.
Mallory would lose 96 happiness units by sitting next to Bob.
Mallory would lose 51 happiness units by sitting next to Carol.
Mallory would lose 81 happiness units by sitting next to David.
Mallory would gain 31 happiness units by sitting next to Eric.
Mallory would lose 73 happiness units by sitting next to Frank.
Mallory would lose 89 happiness units by sitting next to George."
}