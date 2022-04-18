pub fn find_first_nonce(input: &str) -> u32 {
    for i in 0.. {
        let hash = md5_hash(input, i);
        if hash.starts_with("000000") {
            return i;
        }
    }
    0
}

fn md5_hash(input: &str, i: u32) -> String {
    let s = format!("{}{}", input, i);
    let digest = md5::compute(s);
    format!("{:x}", digest)
}

pub fn run() {
    let n = find_first_nonce("iwrupvqb");
    println!("found at {}", n);
}