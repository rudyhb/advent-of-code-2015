use std::collections::{BTreeMap, BTreeSet, HashSet};

pub fn run() {
    let input = get_input();
//     let input = "\
// 123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i";

    let var = "a";

    let inputs = input.split('\n');

    let circuit_box = CircuitBox::with_instructions(inputs.map(|i| i.into()));

    let results = circuit_box.calculate();
    for result in results.iter() {
        println!("[{}] {}", result.0, result.1);
    }

    println!("output for {} is {}", var, results.get(var).unwrap())
}

struct CircuitBox<'a> {
    instructions: Vec<Instruction<'a>>,
}

fn get_values<'a>(instruction: &Instruction, state: &BTreeMap<&'a str, u16>) -> Vec<u16> {
    instruction.values.iter().map(|name| *state.get(name).unwrap())
        .chain(instruction.num_values.iter().map(|i| *i))
        .collect()
}

impl<'a> CircuitBox<'a> {
    pub fn with_instructions<T>(instructions: T) -> Self
        where T: Iterator<Item=Instruction<'a>>
    {
        Self {
            instructions: instructions.collect()
        }
    }

    fn modify_state<'b, 'c>(state: &'c mut BTreeMap<&'b str, u16>, instruction: &'c Instruction<'b>)
        where 'b : 'c
    {
        match instruction.action {
            Action::Assign => {
                state.insert(instruction.output, get_values(instruction, state)[0]);
            }
            Action::And => {
                let values = get_values(instruction, state);
                state.insert(instruction.output, values[0] & values[1]);
            }
            Action::Or => {
                let values = get_values(instruction, state);
                state.insert(instruction.output, values[0] | values[1]);
            }
            Action::Not => {
                let values = get_values(instruction, state);
                state.insert(instruction.output, !values[0]);
            }
            Action::LShift => {
                let values = get_values(instruction, state);
                let mut x = values[0];
                x = x << values[1];
                state.insert(instruction.output, x);
            }
            Action::RShift => {
                let values = get_values(instruction, state);
                let mut x = values[0];
                x = x >> values[1];
                state.insert(instruction.output, x);
            }
        }
    }

    pub fn calculate(&self) -> BTreeMap<&'a str, u16> {
        let mut state: BTreeMap<&'a str, u16> = Default::default();
        let mut instructions: Vec<&Instruction> = self.instructions.iter().collect();

        while instructions.len() > 0 {
            let mut found = BTreeSet::new();
            instructions = instructions.into_iter().filter(|instruction| {
                // if instruction.output == "ab" {
                //     let instruction = instruction.clone();
                //     let state = state.clone();
                //     let z = state.get("z");
                //     let aa = state.get("aa");
                //     println!("z");
                // }
                if instruction.dependencies().iter().all(|&dependency| state.contains_key(dependency)) {
                    Self::modify_state(&mut state, &instruction);
                    found.insert(instruction.output);

                    false
                } else {
                    true
                }
            }).collect();

            println!("found variables {:?}", found);
            if found.is_empty() {
                let mut set = HashSet::new();
                for i in instructions.iter().flat_map(|i| i.values.to_vec()) {
                    set.insert(i);
                }
                println!("missing variables {:?}", set);
                if !set.is_empty() {
                    println!("unresolved dependencies: {:?}", set);
                    return state;
                }
            }
        }

        state
    }
}

struct Instruction<'a> {
    action: Action,
    values: Vec<&'a str>,
    num_values: Vec<u16>,
    output: &'a str,
}

enum Action {
    Assign,
    And,
    Or,
    Not,
    LShift,
    RShift,
}

impl<'a> Instruction<'a> {
    pub fn dependencies(&self) -> &Vec<&'a str> {
        &self.values
    }
}

fn parse_tuple<'a>(s1: &'a str, s2: &'a str) -> (Vec<&'a str>, Vec<u16>) {
    let value1 = s1.parse();
    let value2 = s2.parse();
    let values: Vec<&str>;
    let num_values: Vec<u16>;

    match (value1, value2) {
        (Ok(x), Ok(y)) => {
            num_values = vec![x, y];
            values = vec![];
        }
        (Ok(x), _) => {
            num_values = vec![x];
            values = vec![s2];
        }
        (_, Ok(y)) => {
            num_values = vec![y];
            values = vec![s1];
        }
        _ => {
            num_values = vec![];
            values = vec![s1, s2];
        }
    }
    (values, num_values)
}

fn parse_single(s1: &str) -> (Vec<&str>, Vec<u16>) {
    let value1 = s1.parse();
    let values: Vec<&str>;
    let num_values: Vec<u16>;

    match value1 {
        Ok(x) => {
            num_values = vec![x];
            values = vec![];
        }
        _ => {
            num_values = vec![];
            values = vec![s1];
        }
    }
    (values, num_values)
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(s: &'a str) -> Self {
        let parts: Vec<&str> = s.split("->").map(|s| s.trim()).collect();
        let output = parts[1];
        let parts: Vec<&str> = parts[0].split(" ").map(|s| s.trim()).collect();
        let action: Action;
        let values: Vec<&str>;
        let num_values: Vec<u16>;

        match parts.len() {
            1 => {
                action = Action::Assign;
                let (values0, num_values0) = parse_single(parts[0]);
                values = values0;
                num_values = num_values0;
            }
            2 => {
                action = Action::Not;
                let (values0, num_values0) = parse_single(parts[1]);
                values = values0;
                num_values = num_values0;
            }
            3 => {
                match parts[1] {
                    "AND" => {
                        action = Action::And;
                        let (values0, num_values0) = parse_tuple(parts[0], parts[2]);
                        values = values0;
                        num_values = num_values0;
                    }
                    "OR" => {
                        action = Action::Or;
                        let (values0, num_values0) = parse_tuple(parts[0], parts[2]);
                        values = values0;
                        num_values = num_values0;
                    }
                    "RSHIFT" => {
                        action = Action::RShift;
                        let (values0, num_values0) = parse_single(parts[0]);
                        if !values0.is_empty() {
                            num_values = vec![parts[2].parse().unwrap()];
                            values = values0;
                        } else {
                            values = vec![];
                            num_values = vec![num_values0[0], parts[2].parse().unwrap()];
                        }
                    }
                    "LSHIFT" => {
                        action = Action::LShift;
                        let (values0, num_values0) = parse_single(parts[0]);
                        if !values0.is_empty() {
                            values = values0;
                            num_values = vec![parts[2].parse().unwrap()];
                        } else {
                            values = vec![];
                            num_values = vec![num_values0[0], parts[2].parse().unwrap()];
                        }
                    }
                    _ => panic!("invalid instruction")
                }
            }
            _ => panic!("invalid number of parts")
        }

        Self {
            action,
            values,
            output,
            num_values,
        }
    }
}

fn get_input() -> &'static str {
    "\
NOT dq -> dr
kg OR kf -> kh
ep OR eo -> eq
3176 -> b
NOT gs -> gt
dd OR do -> dp
eg AND ei -> ej
y AND ae -> ag
jx AND jz -> ka
lf RSHIFT 2 -> lg
z AND aa -> ac
dy AND ej -> el
bj OR bi -> bk
kk RSHIFT 3 -> km
NOT cn -> co
gn AND gp -> gq
cq AND cs -> ct
eo LSHIFT 15 -> es
lg OR lm -> ln
dy OR ej -> ek
NOT di -> dj
1 AND fi -> fj
kf LSHIFT 15 -> kj
NOT jy -> jz
NOT ft -> fu
fs AND fu -> fv
NOT hr -> hs
ck OR cl -> cm
jp RSHIFT 5 -> js
iv OR jb -> jc
is OR it -> iu
ld OR le -> lf
NOT fc -> fd
NOT dm -> dn
bn OR by -> bz
aj AND al -> am
cd LSHIFT 15 -> ch
jp AND ka -> kc
ci OR ct -> cu
gv AND gx -> gy
de AND dk -> dm
x RSHIFT 5 -> aa
et RSHIFT 2 -> eu
x RSHIFT 1 -> aq
ia OR ig -> ih
bk LSHIFT 1 -> ce
y OR ae -> af
NOT ca -> cb
e AND f -> h
ia AND ig -> ii
ck AND cl -> cn
NOT jh -> ji
z OR aa -> ab
1 AND en -> eo
ib AND ic -> ie
NOT eh -> ei
iy AND ja -> jb
NOT bb -> bc
ha OR gz -> hb
1 AND cx -> cy
NOT ax -> ay
ev OR ew -> ex
bn RSHIFT 2 -> bo
er OR es -> et
eu OR fa -> fb
jp OR ka -> kb
ea AND eb -> ed
k AND m -> n
et RSHIFT 3 -> ev
et RSHIFT 5 -> ew
hz RSHIFT 1 -> is
ki OR kj -> kk
NOT h -> i
lv LSHIFT 15 -> lz
as RSHIFT 1 -> bl
hu LSHIFT 15 -> hy
iw AND ix -> iz
lf RSHIFT 1 -> ly
fp OR fv -> fw
1 AND am -> an
ap LSHIFT 1 -> bj
u LSHIFT 1 -> ao
b RSHIFT 5 -> f
jq AND jw -> jy
iu RSHIFT 3 -> iw
ih AND ij -> ik
NOT iz -> ja
de OR dk -> dl
iu OR jf -> jg
as AND bd -> bf
b RSHIFT 3 -> e
jq OR jw -> jx
iv AND jb -> jd
cg OR ch -> ci
iu AND jf -> jh
lx -> a
1 AND cc -> cd
ly OR lz -> ma
NOT el -> em
1 AND bh -> bi
fb AND fd -> fe
lf OR lq -> lr
bn RSHIFT 3 -> bp
bn AND by -> ca
af AND ah -> ai
cf LSHIFT 1 -> cz
dw OR dx -> dy
gj AND gu -> gw
jg AND ji -> jj
jr OR js -> jt
bl OR bm -> bn
gj RSHIFT 2 -> gk
cj OR cp -> cq
gj OR gu -> gv
b OR n -> o
o AND q -> r
bi LSHIFT 15 -> bm
dy RSHIFT 1 -> er
cu AND cw -> cx
iw OR ix -> iy
hc OR hd -> he
0 -> c
db OR dc -> dd
kk RSHIFT 2 -> kl
eq LSHIFT 1 -> fk
dz OR ef -> eg
NOT ed -> ee
lw OR lv -> lx
fw AND fy -> fz
dz AND ef -> eh
jp RSHIFT 3 -> jr
lg AND lm -> lo
ci RSHIFT 2 -> cj
be AND bg -> bh
lc LSHIFT 1 -> lw
hm AND ho -> hp
jr AND js -> ju
1 AND io -> ip
cm AND co -> cp
ib OR ic -> id
NOT bf -> bg
fo RSHIFT 5 -> fr
ip LSHIFT 15 -> it
jt AND jv -> jw
jc AND je -> jf
du OR dt -> dv
NOT fx -> fy
aw AND ay -> az
ge LSHIFT 15 -> gi
NOT ak -> al
fm OR fn -> fo
ff AND fh -> fi
ci RSHIFT 5 -> cl
cz OR cy -> da
NOT ey -> ez
NOT ju -> jv
NOT ls -> lt
kk AND kv -> kx
NOT ii -> ij
kl AND kr -> kt
jk LSHIFT 15 -> jo
e OR f -> g
NOT bs -> bt
hi AND hk -> hl
hz OR ik -> il
ek AND em -> en
ao OR an -> ap
dv LSHIFT 1 -> ep
an LSHIFT 15 -> ar
fo RSHIFT 1 -> gh
NOT im -> in
kk RSHIFT 1 -> ld
hw LSHIFT 1 -> iq
ec AND ee -> ef
hb LSHIFT 1 -> hv
kb AND kd -> ke
x AND ai -> ak
dd AND do -> dq
aq OR ar -> as
iq OR ip -> ir
dl AND dn -> do
iu RSHIFT 5 -> ix
as OR bd -> be
NOT go -> gp
fk OR fj -> fl
jm LSHIFT 1 -> kg
NOT cv -> cw
dp AND dr -> ds
dt LSHIFT 15 -> dx
et RSHIFT 1 -> fm
dy RSHIFT 3 -> ea
fp AND fv -> fx
NOT p -> q
dd RSHIFT 2 -> de
eu AND fa -> fc
ba AND bc -> bd
dh AND dj -> dk
lr AND lt -> lu
he RSHIFT 1 -> hx
ex AND ez -> fa
df OR dg -> dh
fj LSHIFT 15 -> fn
NOT kx -> ky
gk OR gq -> gr
dy RSHIFT 2 -> dz
gh OR gi -> gj
lj AND ll -> lm
x OR ai -> aj
bz AND cb -> cc
1 AND lu -> lv
as RSHIFT 3 -> au
ce OR cd -> cf
il AND in -> io
dd RSHIFT 1 -> dw
NOT lo -> lp
c LSHIFT 1 -> t
dd RSHIFT 3 -> df
dd RSHIFT 5 -> dg
lh AND li -> lk
lf RSHIFT 5 -> li
dy RSHIFT 5 -> eb
NOT kt -> ku
at OR az -> ba
x RSHIFT 3 -> z
NOT lk -> ll
lb OR la -> lc
1 AND r -> s
lh OR li -> lj
ln AND lp -> lq
kk RSHIFT 5 -> kn
ea OR eb -> ec
ci AND ct -> cv
b RSHIFT 2 -> d
jp RSHIFT 1 -> ki
NOT cr -> cs
NOT jd -> je
jp RSHIFT 2 -> jq
jn OR jo -> jp
lf RSHIFT 3 -> lh
1 AND ds -> dt
lf AND lq -> ls
la LSHIFT 15 -> le
NOT fg -> fh
at AND az -> bb
au AND av -> ax
kw AND ky -> kz
v OR w -> x
kk OR kv -> kw
ks AND ku -> kv
kh LSHIFT 1 -> lb
1 AND kz -> la
NOT kc -> kd
x RSHIFT 2 -> y
et OR fe -> ff
et AND fe -> fg
NOT ac -> ad
jl OR jk -> jm
1 AND jj -> jk
bn RSHIFT 1 -> cg
NOT kp -> kq
ci RSHIFT 3 -> ck
ev AND ew -> ey
1 AND ke -> kf
cj AND cp -> cr
ir LSHIFT 1 -> jl
NOT gw -> gx
as RSHIFT 2 -> at
iu RSHIFT 1 -> jn
cy LSHIFT 15 -> dc
hg OR hh -> hi
ci RSHIFT 1 -> db
au OR av -> aw
km AND kn -> kp
gj RSHIFT 1 -> hc
iu RSHIFT 2 -> iv
ab AND ad -> ae
da LSHIFT 1 -> du
NOT bw -> bx
km OR kn -> ko
ko AND kq -> kr
bv AND bx -> by
kl OR kr -> ks
1 AND ht -> hu
df AND dg -> di
NOT ag -> ah
d OR j -> k
d AND j -> l
b AND n -> p
gf OR ge -> gg
gg LSHIFT 1 -> ha
bn RSHIFT 5 -> bq
bo OR bu -> bv
1 AND gy -> gz
s LSHIFT 15 -> w
NOT ie -> if
as RSHIFT 5 -> av
bo AND bu -> bw
hz AND ik -> im
bp AND bq -> bs
b RSHIFT 1 -> v
NOT l -> m
bp OR bq -> br
g AND i -> j
br AND bt -> bu
t OR s -> u
hz RSHIFT 5 -> ic
gk AND gq -> gs
fl LSHIFT 1 -> gf
he RSHIFT 3 -> hg
gz LSHIFT 15 -> hd
hf OR hl -> hm
1 AND gd -> ge
fo OR fz -> ga
id AND if -> ig
fo AND fz -> gb
gr AND gt -> gu
he OR hp -> hq
fq AND fr -> ft
ga AND gc -> gd
fo RSHIFT 2 -> fp
gl OR gm -> gn
hg AND hh -> hj
NOT hn -> ho
gl AND gm -> go
he RSHIFT 5 -> hh
NOT gb -> gc
hq AND hs -> ht
hz RSHIFT 3 -> ib
hz RSHIFT 2 -> ia
fq OR fr -> fs
hx OR hy -> hz
he AND hp -> hr
gj RSHIFT 5 -> gm
hf AND hl -> hn
hv OR hu -> hw
NOT hj -> hk
gj RSHIFT 3 -> gl
fo RSHIFT 3 -> fq
he RSHIFT 2 -> hf"
}