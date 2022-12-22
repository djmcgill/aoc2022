use fxhash::FxHashMap as HashMap;
use std::{
    str::{from_utf8, FromStr},
    time::Instant,
};

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
    Div,
    Sub,
}
#[derive(Clone, Copy, Debug)]
enum Job<T> {
    Const(isize),
    Op(bool, Op, T, T),
    Human,
}
type Key = u32;
type P1Job = Job<Key>;
type P2Job = Job<Child>;

#[derive(Clone, Copy, Debug)]
enum Child {
    Monkey(Key),
    Const(isize),
}

const ROOT: u32 = u32::from_le_bytes([b'r', b'o', b'o', b't']);
const HUMN: u32 = u32::from_le_bytes([b'h', b'u', b'm', b'n']);

const INPUT: &str = REAL;
// const INPUT: &str = TEST;

fn main() {
    let start_time = Instant::now();
    let mut jobs = parse_p1();

    // P1:
    // take the dynamic programming approach, so you go from the root down and add to a queue all
    // unevaluated nodes, checking if both their children are const and working out their value if so
    // eventually, all that's left is the const root node
    //
    let mut job_queue = Vec::new();
    job_queue.push(ROOT);
    while let Some(job_id) = job_queue.pop() {
        let old_status = {
            let job = jobs.get_mut(&job_id).unwrap();
            if let Job::Op(pending, _, _, _) = job {
                *pending = true;
            };
            job
        };
        match *old_status {
            Job::Op(_, op, arg_1, arg_2) => {
                let old_arg_1 = jobs.get(&arg_1).unwrap();
                let old_arg_2 = jobs.get(&arg_2).unwrap();

                // if they're both const, replace us with the value
                if let (Job::Const(arg_1_value), Job::Const(arg_2_value)) = (old_arg_1, old_arg_2) {
                    let new_value = do_op(op, *arg_1_value, *arg_2_value);
                    let job = jobs.get_mut(&job_id).unwrap();
                    *job = Job::Const(new_value);
                } else {
                    // if they're const or already in the queue, no need to evaluate
                    let arg_1_is_new_op = if let Job::Op(arg1_pending, _, _, _) = old_arg_1 {
                        !arg1_pending
                    } else {
                        false
                    };
                    let arg_2_is_new_op = if let Job::Op(arg2_pending, _, _, _) = old_arg_2 {
                        !arg2_pending
                    } else {
                        false
                    };
                    job_queue.push(job_id);
                    if arg_1_is_new_op {
                        job_queue.push(arg_1)
                    };
                    if arg_2_is_new_op {
                        job_queue.push(arg_2)
                    };
                }
            }
            _ => {}
        }
    }
    let p1 = if let Job::Const(v) = jobs[&ROOT] {
        v
    } else {
        unreachable!()
    };
    let p1_time = Instant::now();

    // p2:
    // for each const in the hashmap, just bubble up until we only have a const on one side and a tree containing
    // a human on the other then we solve the resulting equation

    let mut jobs = parse_p2();
    let mut consts = vec![];
    let mut dep_map: HashMap<Key, Vec<Key>> = HashMap::default();

    for (k, v) in &jobs {
        match v {
            Job::Const(_) => consts.push(*k),
            Job::Op(_, _, Child::Monkey(child_1_id), Child::Monkey(child_2_id)) => {
                dep_map
                    .entry(*child_1_id)
                    .and_modify(|v| v.push(*k))
                    .or_insert_with(|| vec![*k]);
                dep_map
                    .entry(*child_2_id)
                    .and_modify(|v| v.push(*k))
                    .or_insert_with(|| vec![*k]);
            }
            _ => {}
        }
    }

    while let Some(first_const_id) = consts.pop() {
        let first_const: isize = if let Some(Job::Const(v)) = jobs.remove(&first_const_id) {
            v
        } else {
            unreachable!()
        };
        for k in dep_map.get(&first_const_id).unwrap() {
            let v = jobs.get_mut(k).unwrap();
            match v {
                // two consts, even if one indirect, get folded
                Job::Op(_, op, Child::Const(arg1_const), Child::Monkey(monkey_id))
                    if *monkey_id == first_const_id =>
                {
                    consts.push(*k);
                    *v = Job::Const(do_op(*op, *arg1_const, first_const));
                }
                Job::Op(_, op, Child::Monkey(monkey_id), Child::Const(arg2_const))
                    if *monkey_id == first_const_id =>
                {
                    consts.push(*k);
                    *v = Job::Const(do_op(*op, first_const, *arg2_const));
                }
                Job::Op(_, op, Child::Monkey(monkey_1_id), Child::Monkey(monkey_2_id))
                    if *monkey_1_id == first_const_id && *monkey_2_id == first_const_id =>
                {
                    consts.push(*k);
                    *v = Job::Const(do_op(*op, first_const, first_const));
                }

                // bubble up const to parents
                Job::Op(p, op, Child::Monkey(monkey_id), arg2) if *monkey_id == first_const_id => {
                    *v = Job::Op(*p, *op, Child::Const(first_const), *arg2);
                }
                Job::Op(p, op, arg1, Child::Monkey(monkey_id)) if *monkey_id == first_const_id => {
                    *v = Job::Op(*p, *op, *arg1, Child::Const(first_const));
                }
                _ => {}
            }
        }
    }
    // okay we removed all the consts and are just left with standard linear equations
    let (mut x, mut child_id) = match jobs[&ROOT] {
        Job::Op(_, _, Child::Const(x), Child::Monkey(child_id))
        | Job::Op(_, _, Child::Monkey(child_id), Child::Const(x)) => (x, child_id),
        _ => unreachable!(),
    };
    while child_id != HUMN {
        match jobs[&child_id] {
            // x = y+c  =>  x-y =  c
            // x = y-c  =>  x-y = -c  => y-x = c
            // x = y*c  =>  x/y =  c
            // x = y/c  =>  c*x =  y  => c = y/x
            Job::Op(_, op, Child::Const(y), Child::Monkey(new_child_id)) => {
                child_id = new_child_id;
                match op {
                    Op::Add => x -= y,
                    Op::Sub => x = y - x,
                    Op::Mul => x /= y,
                    Op::Div => x = y / x,
                };
            }
            // x = c+y  =>  x-y = c
            // x = c-y  =>  x+y = c
            // x = c*y  =>  x/y = c
            // x = c/y  =>  x*y = c
            Job::Op(_, op, Child::Monkey(new_child_id), Child::Const(y)) => {
                child_id = new_child_id;
                match op {
                    Op::Add => x -= y,
                    Op::Sub => x += y,
                    Op::Mul => x /= y,
                    Op::Div => x *= y,
                };
            }
            _ => unreachable!(),
        }
    }
    let p2 = x;
    let p2_time = Instant::now();
    println!("p1: {} {:?}", p1, p1_time - start_time);
    println!("p2: {} {:?}", p2, p2_time - p1_time);
}

fn parse_id(line: &[u8]) -> Key {
    u32::from_le_bytes([line[0], line[1], line[2], line[3]])
}
fn parse_op(op: u8) -> Op {
    match op {
        b'+' => Op::Add,
        b'-' => Op::Sub,
        b'*' => Op::Mul,
        b'/' => Op::Div,
        _ => unreachable!(),
    }
}

fn parse_p1() -> HashMap<Key, P1Job> {
    INPUT
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let id = parse_id(line);
            let job = if let Ok(output) = isize::from_str(from_utf8(&line[6..]).unwrap()) {
                Job::Const(output)
            } else {
                Job::Op(
                    false,
                    parse_op(line[11]),
                    parse_id(&line[6..]),
                    parse_id(&line[13..]),
                )
            };
            (id, job)
        })
        .collect()
}

fn parse_p2() -> HashMap<Key, P2Job> {
    let map = INPUT
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let id = parse_id(line);
            if id == HUMN {
                (id, Job::Human)
            } else {
                let job = if let Ok(output) = isize::from_str(from_utf8(&line[6..]).unwrap()) {
                    Job::Const(output)
                } else {
                    let child_1_id = parse_id(&line[6..]);
                    let child_2_id = parse_id(&line[13..]);
                    Job::Op(
                        false,
                        parse_op(line[11]),
                        Child::Monkey(child_1_id),
                        Child::Monkey(child_2_id),
                    )
                };
                (id, job)
            }
        })
        .collect();
    map
}

fn do_op(op: Op, arg_1_value: isize, arg_2_value: isize) -> isize {
    match op {
        Op::Add => arg_1_value + arg_2_value,
        Op::Sub => arg_1_value - arg_2_value,
        Op::Mul => arg_1_value * arg_2_value,
        Op::Div => arg_1_value / arg_2_value,
    }
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
