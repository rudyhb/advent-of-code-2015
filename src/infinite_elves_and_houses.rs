use std::collections::HashSet;

pub(crate) fn run() {
    let _input = "34000000";
    // let _input = "34000";
    // let _input = "1800";

    let input: u64 = _input.parse().unwrap();

    // for house in 1.. {
    //     let presents = presents_for_house(house, 10, None);
    //     println!("House {} got {:?} presents.", house, presents);
    //     if presents.unwrap() >= input {
    //         println!("lowest house to get at least {} presents is: {}", input, house);
    //         break;
    //     }
    // }
    for house in 1.. {
        let presents = presents_for_house(house, 11, Some(50));
        println!("House {} got {:?} presents.", house, presents);
        if presents.unwrap() >= input {
            println!("lowest house to get at least {} presents is: {}", input, house);
            break;
        }
    }
}

fn presents_for_house(house_number: u64, multiplier: u64, max_houses_for_elf: Option<u64>) -> Option<u64> {
    if house_number == 1 || house_number == 2 {
        return Some(multiplier + 20 * (house_number - 1));
    }
    // println!("getting factors of {}", house_number);
    let factors: Vec<u64> = prime_factors(house_number);
    // println!("factors: {:?}", factors);
    let mut numbers: HashSet<u64> = Default::default();
    numbers.insert(1);
    let mut combinations = vec![0u64; factors.len()];
    add_1(&mut combinations[..]);
    loop {
        numbers.insert(combinations.iter().enumerate().filter(|(_, val)| **val == 1).map(|(index, _)| factors[index]).product::<u64>());
        if combinations.iter().all(|&n| n == 1) {
            break;
        }
        add_1(&mut combinations);
    }
    if let Some(max_houses_for_elf) = max_houses_for_elf {
        numbers.retain(|&i| {
            i * max_houses_for_elf >= house_number
        });
    }
    println!("numbers: {:?}", numbers);
    Some(multiplier * (numbers.into_iter().sum::<u64>()))
}

fn add_1(v: &mut [u64]) {
    for i in (0..v.len()).rev() {
        if v[i] == 0 {
            v[i] += 1;
            break;
        } else {
            v[i] = 0;
        }
    }
}

fn factor_v2(mut n: u64, limit: u32) -> Option<Vec<u64>> {
    if n < 6 {
        match n {
            1 => return Some(vec![1]),
            2 => return Some(vec![2]),
            3 => return Some(vec![3]),
            4 => return Some(vec![2, 2]),
            5 => return Some(vec![5]),
            _ => {}
        }
    }
    let mut factors: Vec<u64> = Vec::new();
    'outer: loop {
        if n == 1 {
            return Some(factors);
        }

        let mut a = 2u64;
        for i in 2..=limit {
            if n % a == 0 {
                factors.push(a);
                n /= a;
                continue 'outer;
            }
            a = {
                if let Some(a) = a.checked_pow(i) {
                    a % n
                } else {
                    factors.push(n);
                    return Some(factors);
                }
            };
            let g = gcd(a - 1, n);
            if g > 1 {
                factors.push(g);
                n /= g;
                continue 'outer;
            }
        }
        break;
    }

    None
}

fn prime_factors(n: u64) -> Vec<u64> {
    let factors = factor(n);
    factors.into_iter().flat_map(|f| {
        factor_v2(f, 20).unwrap()
    }).collect()
}

fn factor(mut n: u64) -> Vec<u64> {
    if n < 6 {
        match n {
            1 => return vec![1],
            2 => return vec![2],
            3 => return vec![3],
            4 => return vec![2, 2],
            5 => return vec![5],
            _ => {}
        }
    }
    let mut factors = Vec::new();

    loop {
        if n == 1 {
            break;
        }
        let g = |x: u64| {
            x.checked_pow(2).map(|x| (x + 1) % n)
        };
        let mut d = 1u64;
        let mut x = 2u64;
        let mut y = 2u64;

        while d == 1 {
            x = if let Some(x) = g(x) {
                x
            } else {
                factors.push(n);
                return factors;
            };
            y = if let Some(Some(y)) = g(y).map(|gy| g(gy)) {
                y
            } else {
                factors.push(n);
                return factors;
            };
            d = gcd(x.abs_diff(y), n)
        }

        factors.push(d);
        n /= d;
    }

    factors
}


fn gcd<T: std::ops::Rem<Output=T> + std::cmp::PartialOrd + Default + Copy>(mut a: T, mut b: T) -> T {
    while (a % b) > T::default() {
        let r = a % b;
        a = b;
        b = r;
    }
    return b;
}