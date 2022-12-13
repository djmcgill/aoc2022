use scanf::sscanf;
use std::{mem::take, str::FromStr, time::Instant};

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Op,
    op_arg: Option<usize>, // `None` == `old`
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    checks: usize,
}

fn main() {
    let start = Instant::now();
    let mut monkeys = vec![];

    for chunk in TEST.split("\n\n") {
        let mut item_list = String::new();
        let mut op_str = String::new();
        let mut op_arg_str = String::new();
        let mut id = 0;
        let mut test_divisor = 0;
        let mut true_target = 0;
        let mut false_target = 0;
        sscanf!(
            chunk,
            "Monkey {}:
  Starting items: {}
  Operation: new = old {} {}
  Test: divisible by {}
    If true: throw to monkey {}
    If false: throw to monkey {}",
            id,
            item_list,
            op_str,
            op_arg_str,
            test_divisor,
            true_target,
            false_target,
        )
        .unwrap();

        let op = match &op_str[..] {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => unreachable!(),
        };
        let op_arg = match &op_arg_str[..] {
            "old" => None,
            x => Some(usize::from_str(x).unwrap()),
        };
        let mut items = vec![];
        for item_str in item_list.split(", ") {
            items.push(usize::from_str(item_str).unwrap());
        }
        monkeys.push(Monkey {
            id,
            items,
            op,
            op_arg,
            test_divisor,
            true_target,
            false_target,
            checks: 0,
        });
    }

    let n = monkeys.len();
    let mut div = 1;
    for monkey in &monkeys {
        div *= monkey.test_divisor;
    }

    for _round in 0..10_000 {
        for monkey_ix in 0..n {
            let old_items = take(&mut monkeys[monkey_ix].items);
            monkeys[monkey_ix].checks += old_items.len();

            for item in old_items {
                let monkey = &mut monkeys[monkey_ix];
                let op_arg = monkey.op_arg.unwrap_or(item);
                let item = match monkey.op {
                    Op::Add => item + op_arg,
                    Op::Mul => item * op_arg,
                };
                let item = item % div;
                // let item = item / 3;
                let target_ix = if item % monkey.test_divisor == 0 {
                    monkey.true_target
                } else {
                    monkey.false_target
                };
                monkeys[target_ix].items.push(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.checks);
    let p2 = monkeys[n - 1].checks * monkeys[n - 2].checks;
    let p2_time = Instant::now();
    println!("{}", p2);
    println!("{:?}", p2_time - start);
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;
