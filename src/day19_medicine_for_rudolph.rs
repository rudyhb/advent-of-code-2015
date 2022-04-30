use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use utils::a_star::*;
use lazy_static::lazy_static;
use rand::Rng;

pub(crate) fn run() {
    let _input = "H => HO
H => OH
O => HH
e => H
e => O

HOH";
    let _input = "H => HO
H => OH
O => HH
e => H
e => O

HOHOHO";
    let _input = _get_input();

    let input: InputStruct = _input.into();
    let final_formulas: Vec<_> = input.formulas.iter().filter(|f| f.from.0 == "e").cloned().collect();
    let mut formulas = input.formulas;
    formulas.retain(|f| f.from.0 != "e");
    formulas.sort_by(|a, b| a.to.0.len().cmp(&b.to.0.len()));
    let start = input.start;

    let next = start.next_possible_steps(&formulas);
    println!("there are {} distinct molecules", next.len());
    // for m in next {
    //     println!("{}", m.0.into_iter().map(|m| format!("'{}'", m.0)).collect::<String>());
    // }


    formulas.extend(final_formulas);
    // let target = start;
    // let start = Compound::electron();
    let target = Compound::electron();
    let start = start;

    println!("target molecule: {:?}", target);
    // let result = a_star_search(start, &target, |compound| get_successors_rev(compound, &formulas), distance_function_rev, None).expect("no solution found");

    let result = cheat(start, &target, &formulas, 0);

    // println!("{:?}", result);
    println!("best path is length {}", result.len() - 1);
}

fn cheat(start: Compound, target: &Compound, formulas: &[Expression], times: usize) -> Vec<String> {
    if times > 10_000 {
        panic!("no solution found");
    }
    println!("starting round {}", times);
    let mut rng = rand::thread_rng();
    let mut result: Vec<String> = Default::default();
    let mut formulas_vec: Vec<_> = formulas.into_iter()
        .map(|e| (e, 0usize))
        .collect();
    let mut shuffle = |formulas: &mut Vec<(&Expression, usize)>| {
        for (_, r) in formulas.iter_mut() {
            *r = rng.gen();
        }
        formulas.sort_by(|(_, a), (_, b)| a.cmp(b));
    };

    let target_str = target.to_string();
    let mut current = start.to_string();
    result.push(current.clone());
    for i in 0..10_000_000 {
        shuffle(&mut formulas_vec);
        if i > 0 && i % 100_000 == 0 {
            println!("i={}, current={}", i, current);
        }
        if current == target_str {
            return result;
        }
        let mut any_changes = false;
        for (expression, _) in formulas_vec.iter() {
            let to = expression.to.to_string();
            if current.contains(&to) {
                any_changes = true;
                let count = current.matches(&to).count();
                current = current.replace(&to, expression.from.0);
                for _ in 0..count {
                    result.push(current.clone());
                }
            }
        }
        if !any_changes {
            return cheat(start, target, formulas, times + 1);
        }
    }

    cheat(start, target, formulas, times + 1)
}

impl AStarNode for Compound {}

static PRODUCTS: [&'static str; 40] = [
    "ThF",
    "ThRnFAr",
    "BCa",
    "TiB",
    "TiRnFAr",
    "CaCa",
    "PB",
    "PRnFAr",
    "SiRnFYFAr",
    "SiRnMgAr",
    "SiTh",
    "CaF",
    "PMg",
    "SiAl",
    "CRnAlAr",
    "CRnFYFYFAr",
    "CRnFYMgAr",
    "CRnMgYFAr",
    "HCa",
    "NRnFYFAr",
    "NRnMgAr",
    "NTh",
    "OB",
    "ORnFAr",
    "BF",
    "TiMg",
    "CRnFAr",
    "HSi",
    "CRnFYFAr",
    "CRnMgAr",
    "HP",
    "NRnFAr",
    "OTi",
    "CaP",
    "PTi",
    "SiRnFAr",
    "CaSi",
    "ThCa",
    "BP",
    "TiTi",
];

lazy_static! {
#[allow(unused)]
        static ref CA_FORMULA: Expression = Expression {
            from: Element("Ca"),
            to: Compound(vec![Element("Ca"), Element("Ca")])
        };
}

#[allow(unused)]
fn try_get_successors_rev_optimized(compound: &Compound) -> Option<Compound> {
    compound.0.windows(2)
        .enumerate()
        .filter(|(_, item)| item[0].0 == "Ca" && item[1].0 == "Ca")
        .map(|(i, _)| i)
        .next()
        .map(|index| {
            let formula = &CA_FORMULA;
            let mut next = compound.clone();
            next.apply_reverse_at_single(index, formula);
            next
        })
}

#[allow(unused)]
fn get_successors_rev(compound: &Compound, formulas: &[Expression]) -> Vec<Successor<Compound>> {
    // if let Some(next) = try_get_successors_rev_optimized(compound) {
    //     return vec![Successor::new(next, 1)];
    // }
    formulas.iter().flat_map(|formula| {
        (0..compound.0.len())
            .filter(|&i| {
                if i + formula.to.0.len() > compound.0.len() {
                    false
                } else {
                    compound.0[i..i + formula.to.0.len()] == formula.to.0[..]
                }
            })
            .map(|index| {
                let mut next = compound.clone();
                next.apply_reverse_at_single(index, formula);
                Successor::new(next, 1)
            })
    }).collect()
}

#[allow(unused)]
fn distance_function_rev(node_details: CurrentNodeDetails<Compound>) -> i32 {
    let CurrentNodeDetails {
        current_node: left,
        target_node: _right,
        cost_to_move_to_current: _to_current
    } = node_details;
    let mut distance = if left.0.len() == 1 {
        if *left == Compound::electron() {
            0
        } else {
            1
        }
    } else {
        left.0.len() as i32
    };
    // ((_to_current + distance.pow(2)) as f64).sqrt().floor() as i32

    let mut s: String = left.0.iter().map(|e| e.0).collect();
    for &product in PRODUCTS.iter() {
        s = s.replace(product, "");
    }
    let tot_len = |c: &Compound| c.0.iter().map(|i| i.0.len()).sum::<usize>();
    distance *= 5;
    distance += (tot_len(left) as i32 - s.len() as i32);
    // for &product in PRODUCTS.iter() {
    //     non_matches += s.matches(product).count() as i32;
    // }

    distance
}

#[allow(unused)]
fn distance_function(node_details: CurrentNodeDetails<Compound>) -> i32 {
    let CurrentNodeDetails {
        current_node: left,
        target_node: right,
        cost_to_move_to_current: _to_current
    } = node_details;
    let mut non_matches = (left.0.len() as i32 - right.0.len() as i32).abs();
    // non_matches *= 2;
    // non_matches += (0..left.0.len().min(right.0.len()))
    //     .filter(|&i| {
    //         left.0[i] != right.0[i]
    //     })
    //     .count() as i32;
    // non_matches *= 2;
    // let equal_windows_count = right.0.windows(3).filter(|&right| {
    //     left.0.windows(3).any(|left| {
    //         left == right
    //     })
    // }).count() as i32;
    // non_matches += (right.0.len() as i32 - 2) - equal_windows_count;
    non_matches
}

#[allow(unused)]
fn get_successors(compound: &Compound, formulas: &[Expression]) -> Vec<Successor<Compound>> {
    compound.0.iter().enumerate().flat_map(|(index, element)| {
        formulas.iter().filter(|formula| {
            formula.from == *element
        })
            .map(move |formula| {
                let mut next = compound.clone();
                next.apply_at_single(index, formula);
                Successor::new(next, 1)
            })
    }).collect()
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Element(&'static str);

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Compound(Vec<Element>);

impl Debug for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.to_string(), self.0.len())
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct FormulatedCompound(Vec<FormulaStep>);

#[derive(Clone, Hash, Eq, PartialEq)]
struct FormulaStep {
    apply_at_index: usize,
    expression_index: usize,
}

impl Display for Compound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|c| c.0).collect::<String>())
    }
}

#[allow(unused)]
impl Compound {
    pub(crate) fn next_possible_steps(&self, formulas: &[Expression]) -> HashSet<Self> {
        let mut result: HashSet<Self> = Default::default();
        for (index, element) in self.0.iter().enumerate() {
            for formula in formulas {
                if formula.from == *element {
                    result.insert(self.with_replaced(index, &formula.to));
                }
            }
        }

        result
    }
    pub(crate) fn electron() -> Self {
        Self {
            0: vec![Element("e")]
        }
    }
    fn with_replaced(&self, at_index: usize, compound: &Self) -> Self {
        let mut result: Vec<Element> = Vec::with_capacity(self.0.len() + compound.0.len() - 1);
        result.extend((0..at_index).into_iter().map(|i| self.0[i].clone()));
        result.extend(compound.0.clone());
        result.extend((at_index + 1..self.0.len()).into_iter().map(|i| self.0[i].clone()));

        Self(result)
    }
    fn with_reverse_replaced(&self, at_index: usize, length: usize, element: Element) -> Self {
        let mut result: Vec<Element> = Vec::with_capacity(self.0.len() - (length - 1));
        result.extend((0..at_index).into_iter().map(|i| self.0[i].clone()));
        result.push(element);
        result.extend((at_index + length..self.0.len()).into_iter().map(|i| self.0[i].clone()));

        Self(result)
    }
    pub(crate) fn grow_towards(self, from_index: usize, target: &Self, formulas: &[Expression], current_age: usize, max_repeat: usize) -> HashMap<Self, usize> {
        if self.0.len() <= from_index || target.0.len() <= from_index {
            Default::default()
        } else {
            let self_element = &self.0[from_index];
            let target_element = &target.0[from_index];
            let mut results: HashMap<Self, usize> = HashMap::new();
            for next in self_element.grow_towards(target_element, formulas, Default::default(), max_repeat) {
                let age = current_age + next.len();
                let mut item = self.clone();
                item.apply_at(from_index, &next);
                // if item.0.len() > target.0.len() || self.0.len() - from_index > 25 {
                //     continue;
                // }
                results.insert(item, age);
            }
            if max_repeat > 0 {
                let items: Vec<(Compound, usize)> = results.clone().into_iter().collect();
                for (item, current_age) in items {
                    results.extend(item.grow_towards(from_index, target, formulas, current_age, max_repeat - 1))
                }
            }
            results
        }
    }

    fn apply_at(&mut self, index: usize, actions: &[&Expression]) {
        for action in actions {
            *self = self.with_replaced(index, &action.to);
        }
    }

    pub(crate) fn apply_at_single(&mut self, index: usize, action: &Expression) {
        *self = self.with_replaced(index, &action.to);
    }
    pub(crate) fn apply_reverse_at_single(&mut self, index: usize, action: &Expression) {
        *self = self.with_reverse_replaced(index, action.to.0.len(), action.from.clone());
    }
}

#[allow(unused)]
impl Element {
    pub(crate) fn grow_towards<'a>(&'a self, current_target: &'a Element, formulas: &'a [Expression], history: Vec<&'a Expression>, max_repeat: usize) -> Vec<Vec<&'a Expression>> {
        let mut results: Vec<Vec<&'a Expression>> = formulas.iter().filter(|f| {
            f.from == *self && if max_repeat == 0 {
                !history.contains(f)
            } else {
                history.iter().filter(|h| *h == f).count() <= max_repeat
            }
        })
            .flat_map(|formula| {
                let mut current_history = history.clone();
                current_history.push(formula);
                let next_element = &formula.to.0[0];
                if current_target == next_element {
                    vec![current_history]
                } else {
                    next_element.grow_towards(current_target, formulas, current_history, max_repeat)
                }
            }).collect();

        if self == current_target {
            results.push(Vec::new());
        }

        results
    }
}

#[allow(unused)]
impl FormulatedCompound {
    pub(crate) fn resolve(&self, formulas: &[Expression]) -> Compound {
        let mut compound = Compound::electron();
        for action in self.0.iter() {
            compound.apply_at_single(action.apply_at_index, &formulas[action.expression_index]);
        }

        compound
    }
    pub(crate) fn next_possible_steps(&self, formulas: &[Expression]) -> HashSet<Compound> {
        let mut result: HashSet<Compound> = Default::default();
        let compound = self.resolve(formulas);
        for (index, element) in compound.0.iter().enumerate() {
            for formula in formulas {
                if formula.from == *element {
                    result.insert(compound.with_replaced(index, &formula.to));
                }
            }
        }

        result
    }
    pub(crate) fn grow_towards(self, from_index: usize, target: &Compound, formulas: &[Expression], max_repeat: usize) -> HashSet<Self> {
        let compound = self.resolve(formulas);
        if compound.0.len() <= from_index || target.0.len() <= from_index {
            Default::default()
        } else {
            let self_element = &compound.0[from_index];
            let target_element = &target.0[from_index];
            let mut results: HashSet<Self> = Default::default();
            for next in self_element.grow_towards(target_element, formulas, Default::default(), max_repeat) {
                let mut item = self.clone();
                item.0.extend(next.into_iter().map(|action| formulas.iter().position(|i| i == action).unwrap())
                    .map(|expression_index| FormulaStep {
                        apply_at_index: from_index,
                        expression_index,
                    }));
                results.insert(item);
            }
            if max_repeat > 0 {
                for item in results.clone().into_iter() {
                    results.extend(item.grow_towards(from_index, target, formulas, max_repeat - 1))
                }
            }
            results
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Expression {
    from: Element,
    to: Compound,
}

impl From<&'static str> for Expression {
    fn from(s: &'static str) -> Self {
        let mut parts = s.split(" => ");
        let from = Element(parts.next().unwrap().trim());
        let to: Compound = parts.next().unwrap().into();
        Self {
            from,
            to,
        }
    }
}

impl From<&'static str> for Compound {
    fn from(s: &'static str) -> Self {
        let s = s.trim();
        let mut i = 0;
        let mut elements: Vec<Element> = Vec::new();
        for c in s.chars() {
            if c.is_lowercase() && c != 'e' {
                *elements.last_mut().unwrap() = Element(&s[i - 1..=i]);
            } else {
                elements.push(Element(&s[i..=i]));
            }
            i += 1;
        }

        Self(elements)
    }
}

struct InputStruct {
    formulas: Vec<Expression>,
    start: Compound,
}

impl From<&'static str> for InputStruct {
    fn from(s: &'static str) -> Self {
        let mut lines = s.split('\n');
        let mut formulas: Vec<Expression> = Default::default();
        loop {
            let line = lines.next().unwrap();
            if line.trim().is_empty() {
                break;
            }
            formulas.push(line.into());
        }
        let start: Compound = lines.next().unwrap().into();
        Self {
            formulas,
            start,
        }
    }
}

fn _get_input() -> &'static str {
    "Al => ThF
Al => ThRnFAr
B => BCa
B => TiB
B => TiRnFAr
Ca => CaCa
Ca => PB
Ca => PRnFAr
Ca => SiRnFYFAr
Ca => SiRnMgAr
Ca => SiTh
F => CaF
F => PMg
F => SiAl
H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
H => HCa
H => NRnFYFAr
H => NRnMgAr
H => NTh
H => OB
H => ORnFAr
Mg => BF
Mg => TiMg
N => CRnFAr
N => HSi
O => CRnFYFAr
O => CRnMgAr
O => HP
O => NRnFAr
O => OTi
P => CaP
P => PTi
P => SiRnFAr
Si => CaSi
Th => ThCa
Ti => BP
Ti => TiTi
e => HF
e => NAl
e => OMg

CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr"
}