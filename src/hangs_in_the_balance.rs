use std::collections::HashSet;
use std::fmt::{Debug};
use utils::timer::Timer;

pub(crate) fn run() {
    let _input = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    let _input = _get_input();

    let input: Vec<usize> = _input.split('\n').map(|s| s.parse().unwrap()).collect();

    println!("sum: {}", input.iter().sum::<usize>());
    let ideal = get_ideal_configuration(&input, false);
    println!("Ideal configuration:");
    for i in 0..3 {
        println!("Group {} (QE {}): {:?}", i + 1, ideal[i].quantum_entanglement(), ideal[i]);
    }
    println!("QE for group 1: {}", ideal[0].quantum_entanglement());
}

fn get_possible_groups_2_3(weights: &[usize], reserved_indexes: &HashSet<usize>, target_weight: usize) -> Option<(PackageGroup, PackageGroup)> {
    let allowed_indexes: Vec<_> = (0..weights.len()).filter(|i| !reserved_indexes.contains(i)).collect();
    println!("starting get possible group 2 with {} indexes left", allowed_indexes.len());
    let max = (1u32 << allowed_indexes.len()) - 1;
    for input in 1u32..max {
        let (sum2, sum3) = allowed_indexes.iter().enumerate().fold((0usize, 0usize), |(sum2, sum3), (i, weight_index)| {
            let val = weights[*weight_index];
            if input & (1 << i) == 0 {
                (sum2 + val, sum3)
            } else {
                (sum2, sum3 + val)
            }
        });
        if sum2 == target_weight && sum3 == target_weight {
            let group2: PackageGroup = allowed_indexes.iter().enumerate().filter(|(i, _)| input & (1 << *i) == 0).map(|(_, index)| weights[*index]).collect();
            let group3: PackageGroup = allowed_indexes.iter().enumerate().filter(|(i, _)| input & (1 << *i) != 0).map(|(_, index)| weights[*index]).collect();
            return Some((group2, group3));
        }
    }

    None
}

fn get_possible_groups_2_3_4(weights: &[usize], reserved_indexes: &HashSet<usize>, target_weight: usize) -> Option<(PackageGroup, PackageGroup, PackageGroup)> {
    let allowed_indexes: Vec<_> = (0..weights.len()).filter(|i| !reserved_indexes.contains(i)).collect();
    println!("starting get possible group 2 with {} indexes left", allowed_indexes.len());
    let mut input = vec![0u8; allowed_indexes.len()];
    let add_1 = |input: &mut Vec<u8>| -> bool {
        for i in (0..input.len()).rev() {
            if input[i] == 2 {
                input[i] = 0;
            } else {
                input[i] += 1;
                return true;
            }
        }
        false
    };
    while add_1(&mut input) {
        let (sum2, sum3, sum4) = allowed_indexes.iter().enumerate().fold((0usize, 0usize, 0usize), |(sum2, sum3, sum4), (i, weight_index)| {
            let val = weights[*weight_index];
            match input[i] {
                0 => (sum2 + val, sum3, sum4),
                1 => (sum2, sum3 + val, sum4),
                2 => (sum2, sum3, sum4 + val),
                _ => panic!("out of bounds")
            }
        });
        if sum2 == target_weight && sum3 == target_weight && sum4 == target_weight {
            let mut group2 = PackageGroup::default();
            let mut group3 = group2.clone();
            let mut group4 = group2.clone();
            for (i, index) in allowed_indexes.into_iter().enumerate() {
                let val = weights[index];
                match input[i] {
                    0 => {
                        group2.values.push(val);
                    }
                    1 => {
                        group3.values.push(val);
                    }
                    2 => {
                        group4.values.push(val);
                    }
                    _ => panic!("out of bounds")
                }
            }
            return Some((group2, group3, group4));
        }
    }

    None
}

fn filter_weights<'a>(weights: &'a [usize], bits: u32) -> impl Iterator<Item=usize> + 'a {
    filter_weights_indexed(weights, bits)
        .map(|(_, weight)| {
            *weight
        })
}

fn filter_weights_indexed<'a>(weights: &'a [usize], bits: u32) -> impl Iterator<Item=(usize, &'a usize)> + 'a {
    weights.iter().enumerate().filter(move |(weight_index, _)| {
        bits & (1 << *weight_index) != 0
    })
}

fn get_combinations<'a>(total_bits: usize, set_bits: usize) -> Vec<u32> {
    let mut results: HashSet<(Vec<usize>, u32)> = (0..total_bits)
        .map(|i| {
            (vec![i], 1 << i)
        })
        .collect();
    for _ in 1..set_bits {
        let mut next_results: HashSet<(Vec<usize>, u32)> = HashSet::new();
        for result in results.into_iter() {
            next_results.extend((0..total_bits)
                .filter(|i| !result.0.contains(i))
                .map(|i| {
                    let mut vec = result.0.clone();
                    vec.push(i);
                    vec.sort();
                    (vec, result.1 | 1 << i)
                })
            );
        }
        results = next_results;
    }
    results.into_iter().map(|r| r.1)
        .collect()
}

fn get_ideal_configuration(weights: &[usize], include_trunk: bool) -> Vec<PackageGroup> {
    let target_weight = weights.iter().sum::<usize>() / if include_trunk { 4 } else { 3 };
    for group_size in 1..weights.len() {
        println!("trying group of size {}", group_size);
        let mut groups: Vec<_> = {
            let _timer = Timer::start(|elapsed| println!("got group of size {} (elapsed {} ms.)", group_size, elapsed.as_millis()));
            let mut c = get_combinations(weights.len(), group_size);
            c.sort();
            get_combinations(weights.len(), group_size)
                .into_iter()
                .filter(|&i| {
                    filter_weights(weights, i)
                        .sum::<usize>() == target_weight
                }).collect()
        };
        if !groups.is_empty() {
            println!("got {} possible group1s with len={}", groups.len(), group_size);
            groups.sort_by(|a, b| {
                filter_weights(weights, *a).product::<usize>().cmp(&filter_weights(weights, *b).product::<usize>())
            });
            for group in groups {
                let used_indexes: HashSet<usize> = filter_weights_indexed(weights, group).map(|(index, _)| index).collect();
                let group: PackageGroup = filter_weights(weights, group).collect();
                if include_trunk {
                    if let Some((group2, group3, group4)) = get_possible_groups_2_3_4(weights, &used_indexes, target_weight) {
                        return vec![group, group2, group3, group4];
                    }
                } else {
                    if let Some((group2, group3)) = get_possible_groups_2_3(weights, &used_indexes, target_weight) {
                        return vec![group, group2, group3];
                    }
                }
            }
        }
    }
    panic!("no matches found");
}

#[derive(Debug, Clone)]
struct PackageGroup {
    values: Vec<usize>,
}

impl PackageGroup {
    pub(crate) fn quantum_entanglement(&self) -> usize {
        self.values.iter().product()
    }
}

impl FromIterator<usize> for PackageGroup {
    fn from_iter<T: IntoIterator<Item=usize>>(iter: T) -> Self {
        Self {
            values: iter.into_iter().collect(),
        }
    }
}

impl Default for PackageGroup {
    fn default() -> Self {
        Self {
            values: vec![],
        }
    }
}

fn _get_input() -> &'static str {
    "1
2
3
7
11
13
17
19
23
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
101
103
107
109
113"
}