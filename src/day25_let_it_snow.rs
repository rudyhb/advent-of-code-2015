use std::str::FromStr;

pub(crate) fn run() {
    // let _input = "20151125";
    let _input = _get_input();
    // let target = Coord::new(5, 5);
    let target = Coord::new(2980, 3074);

    let mut manual: Manual = _input.parse().unwrap();

    loop {
        if let Some(code) = manual.get(&target) {
            println!("found code: {}", code);
            break;
        }
        manual.next().expect("error going next");
    }
}

#[derive(Debug)]
struct Error(&'static str);

struct Manual {
    rows: Vec<Vec<u64>>,
    current: Coord,
}

impl Manual {
    pub(crate) fn next(&mut self) -> Result<(), Error> {
        let previous = self.get(&self.current).ok_or(Error("cannot get previous"))?;
        self.go_to_next();

        let next_value = (previous * 252533) % 33554393;

        if let Some(current) = self.get(&self.current) {
            assert_eq!(next_value, current);
            return Ok(());
        }
        let current = self.get_mut(self.current.clone());
        *current = next_value;
        Ok(())
    }
    pub(crate) fn get(&self, coord: &Coord) -> Option<u64> {
        self.rows.iter().nth(coord.row).map(|row| row.iter().nth(coord.col)).flatten().copied()
    }
    fn go_to_next(&mut self) {
        if self.current.row == 0 {
            self.current = Coord {
                row: self.current.col + 1,
                col: 0,
            };
        } else {
            self.current.row -= 1;
            self.current.col += 1;
        }
    }
    fn get_mut(&mut self, coord: Coord) -> &mut u64 {
        while self.rows.len() <= coord.row {
            self.rows.push(Default::default())
        }
        let row = &mut self.rows[coord.row];
        while row.len() <= coord.col {
            row.push(Default::default())
        }
        &mut row[coord.col]
    }
    pub(crate) fn _print(&self) {
        println!("     |{}", (1..=self.rows[0].len()).map(|i| format!("{:>8}", i)).collect::<Vec<String>>().join(" "));
        println!("-----+{}", (1..=self.rows[0].len()).map(|_| "--------+").collect::<String>());
        for i in 0..self.rows.len() {
            println!("{:>4} |{}", i + 1, self.rows[i].iter().map(|j| format!("{:>8}", *j)).collect::<Vec<String>>().join(" "));
        }
    }
}

impl FromStr for Manual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: s.split('\n').map(|line| line.split_whitespace().map(|w| w.parse().unwrap()).collect()).collect(),
            current: Coord::new(0, 0),
        })
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn _get_input() -> &'static str {
    "20151125  18749137  17289845  30943339  10071777  33511524
 31916031  21629792  16929656   7726640  15514188   4041754
 16080970   8057251   1601130   7981243  11661866  16474243
 24592653  32451966  21345942   9380097  10600672  31527494
 77061  17552253  28094349   6899651   9250759  31663883
 33071741   6796745  25397450  24659492   1534922  27995004"
}

trait Flatten<T> {
    fn flatten(self) -> Option<T>;
}

impl<T> Flatten<T> for Option<Option<T>> {
    fn flatten(self) -> Option<T> {
        match self {
            None => None,
            Some(val) => val
        }
    }
}
