use std::collections::HashMap;

pub fn run() {

//     let input = "\
// London to Dublin = 464
// London to Belfast = 518
// Dublin to Belfast = 141";
    let input = get_input();
    let distances = Distances::build(input);

    let mut data: Vec<usize> = (0..distances.length()).map(|i| i).collect();
    let min_dist = permutator::Permutation::permutation(&mut data).map(|p| {
        distances.get_total_distance(&p)
    }).min().unwrap();

    println!("min distance is {}", min_dist);
}

struct Distances {
    values: HashMap<(usize, usize), u32>,
    names: Vec<&'static str>
}

fn get_index_or_add<'a, 'b>(vec: &'b mut Vec<&'a str>, item: &'a str) -> usize
where 'a: 'b
{
    let i = vec.iter().position(|&name| name == item);
    match i {
        Some(i) => i,
        None => {
            vec.push(item);
            vec.len() - 1
        }
    }
}

impl Distances {
    pub fn get_total_distance(&self, permutation: &Vec<usize>) -> u32 {
        let mut last: usize = *permutation.first().unwrap();
        permutation.iter().skip(1).map(|&i| {
            let d = self.values.get(&(last, i)).or(self.values.get(&(i, last))).unwrap();
            last = i;
            d
        }).sum()
    }
    pub fn length(&self) -> usize {
        self.names.len()
    }
    pub fn build(input: &'static str) -> Self {
        let mut names: Vec<&'static str> = vec![];
        let mut values: HashMap<(usize, usize), u32> = Default::default();
        for line in input.split('\n') {
            let mut parts = line.split(' ');
            let name_a = parts.next().unwrap();
            parts.next();
            let name_b = parts.next().unwrap();
            parts.next();
            let distance: u32 = parts.next().unwrap().parse().unwrap();

            let a = get_index_or_add(&mut names, name_a);
            let b = get_index_or_add(&mut names, name_b);

            values.insert((a, b), distance);
        }

        Self {
            names: names,
            values
        }
    }
}

fn get_input() -> &'static str {
    "\
AlphaCentauri to Snowdin = 66
AlphaCentauri to Tambi = 28
AlphaCentauri to Faerun = 60
AlphaCentauri to Norrath = 34
AlphaCentauri to Straylight = 34
AlphaCentauri to Tristram = 3
AlphaCentauri to Arbre = 108
Snowdin to Tambi = 22
Snowdin to Faerun = 12
Snowdin to Norrath = 91
Snowdin to Straylight = 121
Snowdin to Tristram = 111
Snowdin to Arbre = 71
Tambi to Faerun = 39
Tambi to Norrath = 113
Tambi to Straylight = 130
Tambi to Tristram = 35
Tambi to Arbre = 40
Faerun to Norrath = 63
Faerun to Straylight = 21
Faerun to Tristram = 57
Faerun to Arbre = 83
Norrath to Straylight = 9
Norrath to Tristram = 50
Norrath to Arbre = 60
Straylight to Tristram = 27
Straylight to Arbre = 81
Tristram to Arbre = 90"
}