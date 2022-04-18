pub(crate) fn run() {
    let _input = "20\n15\n10\n5\n5";
    let _target_sum = 25u32;
    let _input = _get_input();
    let _target_sum = 150u32;

    let containers: Vec<u32> = _input.split('\n').map(|v| v.parse().unwrap()).collect();

    let combinations = get_combinations(&containers[..], _target_sum);
    println!("{}", combinations.iter().map(|c| c.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")).collect::<Vec<String>>().join("\n"));
    println!("there are {} combinations", combinations.len());

    let min = combinations.iter().map(|c| c.len()).min().unwrap();
    let min_containers = combinations.into_iter().filter(|c| c.len() == min).collect::<Vec<_>>();
    println!("there are {} combinations with the min of {}", min_containers.len(), min);
}

fn get_combinations(containers: &[u32], target_sum: u32) -> Vec<Vec<u32>> {
    containers.into_iter().enumerate().filter_map(|(i, &first)| {
        if first > target_sum {
            None
        } else if first == target_sum {
            Some(vec![vec![first]])
        } else {
            Some(get_combinations(&containers[i + 1..], target_sum - first).into_iter().map(|mut next| {
                next.push(first);
                next
            }).collect::<Vec<_>>())
        }
    }).flat_map(|v| v).collect::<Vec<_>>()
}

fn _get_input() -> &'static str {
    "50
44
11
49
42
46
18
32
26
40
21
7
18
43
10
47
36
24
22
40"
}
