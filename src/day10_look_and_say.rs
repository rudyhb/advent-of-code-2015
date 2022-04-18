pub fn run() {
    let mut input = "1113122113".into();
    println!("input: {}", input);

    let n = 40;

    for _ in 0..n {
        input = play(input);
        println!("{}", input);
    }
    println!("len: {}", input.len());
}

fn play(input: String) -> String {
    let mut result: Vec<char> = Default::default();
    let mut last_char = input.chars().next().unwrap();
    let mut count = 1u32;
    let mut push = |last_char: char, count: u32| {
        for c in count.to_string().chars() {
            result.push(c);
        }
        result.push(last_char);
    };
    for c in input.chars().skip(1) {
        if c == last_char {
            count += 1;
        } else {
            push(last_char, count);
            count = 1;
            last_char = c;
        }
    }
    push(last_char, count);

    result.iter().collect()
}