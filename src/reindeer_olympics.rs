
pub(crate) fn run() {
    let _input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds";
    let _input = _get_input();

    let mut reindeer: Vec<Reindeer> = _input.split('\n').map(|line| line.into()).collect();

    for _ in 0..2503 {
        for r in reindeer.iter_mut() {
            r.advance();
        }
        let lead = reindeer.iter().map(|r| r.current_distance).max().unwrap();
        for r in reindeer.iter_mut().filter(|r| r.current_distance == lead) {
            r.award_point();
        }
    }
    reindeer.sort_by(|a, b| b.current_distance.cmp(&a.current_distance));
    let best = &reindeer[0];
    println!("{} got farthest at {} km", best.name, best.current_distance);

    reindeer.sort_by(|a, b| b.points.cmp(&a.points));
    let best = &reindeer[0];
    println!("{} got most points at {}", best.name, best.points);
}

struct Reindeer<'a> {
    name: &'a str,
    stats: ReindeerStats,
    current_second: u32,
    current_distance: u32,
    points: u32
}

impl<'a> Reindeer<'a> {
    pub(crate) fn advance(&mut self) {
        if self.current_second < self.stats.sprint_period {
            self.current_distance += self.stats.speed;
        }
        self.current_second = (self.current_second + 1) % (self.stats.sprint_period + self.stats.rest_period);
    }
    pub(crate) fn award_point(&mut self) {
        self.points += 1;
    }
}

impl<'a> From<&'a str> for Reindeer<'a> {
    fn from(s: &'a str) -> Self {
        let name = s.split_whitespace().next().unwrap();
        let mut words = s.split_whitespace().filter(|w| w.chars().all(|c| c.is_numeric()));
        let speed: u32 = words.next().unwrap().parse().unwrap();
        let sprint_period: u32 = words.next().unwrap().parse().unwrap();
        let rest_period: u32 = words.next().unwrap().parse().unwrap();
        Self {
            name,
            points: 0,
            stats: ReindeerStats {
                speed,
                sprint_period,
                rest_period
            },
            current_second: 0,
            current_distance: 0
        }
    }
}

struct ReindeerStats {
    speed: u32,
    sprint_period: u32,
    rest_period: u32,
}

fn _get_input() -> &'static str {
    "Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.
Rudolph can fly 3 km/s for 15 seconds, but then must rest for 28 seconds.
Donner can fly 19 km/s for 9 seconds, but then must rest for 164 seconds.
Blitzen can fly 19 km/s for 9 seconds, but then must rest for 158 seconds.
Comet can fly 13 km/s for 7 seconds, but then must rest for 82 seconds.
Cupid can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
Dasher can fly 14 km/s for 3 seconds, but then must rest for 38 seconds.
Dancer can fly 3 km/s for 16 seconds, but then must rest for 37 seconds.
Prancer can fly 25 km/s for 6 seconds, but then must rest for 143 seconds."
}