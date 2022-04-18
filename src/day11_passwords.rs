use std::collections::HashSet;

const CHAR_A: u8 = 'a' as u8;
const CHAR_Z: u8 = 'z' as u8;

pub fn run() {
    let mut password = "hxbxxyzz".as_bytes().to_vec();
    println!("input: {}", to_string(&password));
    loop {
        next_password(&mut password);
        // println!("next: {}", to_string(&password));
        if is_valid(&password) {
            break;
        }
    }
    println!("next password: {}", to_string(&password));
}

fn to_string(word: &Vec<u8>) -> String {
    word.iter().map(|&c| c as char).collect()
}

fn next_letter(letter: u8) -> u8 {
    if letter == CHAR_Z {
        CHAR_A
    } else {
        letter + 1
    }
}

fn next_password(password: &mut Vec<u8>) {
    for i in (0..password.len()).rev() {
        password[i] = next_letter(password[i]);
        if password[i] != CHAR_A {
            break;
        }
    }
}

const INVALID_CHARS: [u8; 3] = ['i' as u8, 'o' as u8, 'l' as u8];

fn is_valid(password: &Vec<u8>) -> bool {
    let mut c2 = *password.iter().nth(0).unwrap();
    let mut c1 = *password.iter().nth(1).unwrap();
    let mut has_increasing_straight = false;
    let mut pairs: HashSet<u8> = Default::default();
    if INVALID_CHARS.iter().any(|&c| c == c1 || c == c2) {
        return false;
    }
    if c1 == c2 {
        pairs.insert(c1);
    }
    for &c in password.iter().skip(2) {
        if INVALID_CHARS.iter().any(|&i| i == c) {
            return false;
        }
        if !has_increasing_straight {
            if c1 == c2 + 1 && c == c1 + 1 {
                has_increasing_straight = true;
            }
        }

        if c == c1 {
            pairs.insert(c);
        }

        c2 = c1;
        c1 = c;
    }

    has_increasing_straight && pairs.len() > 1
}