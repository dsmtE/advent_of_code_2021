use std::{collections::{HashMap, VecDeque}, str::FromStr};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug)]
enum Monkey<'a> {
    Num(i64),
    Calc(Op, &'a str, &'a str),
}

impl std::fmt::Display for Monkey<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Monkey::Num(n) => write!(f, "{}", n),
            Monkey::Calc(operator, lhs, rhs) => write!(f, "{} {} {}", lhs, operator, rhs),
        }
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}


impl std::str::FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            _ => panic!("Invalid math operator"),
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}
impl Op {
    fn to_fn(&self) -> fn(i64, i64) -> i64 {
        match self {
            Op::Add => |a, b| a + b,
            Op::Sub => |a, b| a - b,
            Op::Mul => |a, b| a * b,
            Op::Div => |a, b| a / b,
        }
    }
}

fn parse_monkeys(input: &str) -> HashMap<&str, Monkey> {
    input.lines()
        .map(|line| {
            let (name, right) = line.split_once(": ").unwrap();
            let monkey = match right.parse() {
                Ok(n) => Monkey::Num(n),
                Err(_) => {
                    let mut iter = right.split_ascii_whitespace();
                    let lhs = iter.next().unwrap();
                    let operator = Op::from_str(iter.next().unwrap()).unwrap();
                    let rhs = iter.next().unwrap();
                    Monkey::Calc(operator, lhs, rhs)
                }
            };

            (name, monkey)
        })
        .collect()
}

fn compute_for(name: &str, monkeys: &HashMap<&str, Monkey>) -> i64 {
    match &monkeys[name] {
        Monkey::Num(n) => *n,
        Monkey::Calc(operator, lhs, rhs) => {
            let lhs_num = compute_for(lhs, monkeys);
            let rhs_num = compute_for(rhs, monkeys);
            operator.to_fn()(lhs_num, rhs_num)
        }
    }
}

fn precompute_except<'a>(monkeys: &HashMap<&'a str, Monkey>, except_name: &str) -> HashMap<&'a str, i64> {
    let mut precomputed: HashMap<&'a str, i64> = HashMap::new();

    let mut depend_on: HashMap<&'a str, &Monkey> = HashMap::new();
    let mut priority_queue: VecDeque<&str> = VecDeque::new();

    monkeys.iter().for_each(|(name, monkey)| {
        match monkey {
            Monkey::Num(n) => {
                if except_name != *name {
                    precomputed.insert(name, *n);
                } else {
                    depend_on.insert(name, monkey);
                }
            }
            Monkey::Calc(_, _, _) => priority_queue.push_back(name)
        }
    });

    while let Some(name) = priority_queue.pop_front() {
        let monkey = &monkeys[name];
        match monkey {
            Monkey::Num(_) => panic!("Should not happen"),
            Monkey::Calc(operator, lhs_name, rhs_name) => {
                let lhs_value = precomputed.get(lhs_name);
                let rhs_value = precomputed.get(rhs_name);
                // if precomputed value can be found for both sides, compute and insert
                if lhs_value.is_some() && rhs_value.is_some() {
                    precomputed.insert(
                        name,
                        operator.to_fn()(*lhs_value.unwrap(), *rhs_value.unwrap())
                    );
                } else {
                    // it depends on a least one monkey that depends itself on the except name 
                    let lhs_monkey = depend_on.get(lhs_name);
                    let rhs_monkey = depend_on.get(rhs_name);
                    if lhs_monkey.is_some() || rhs_monkey.is_some() {
                        depend_on.insert(name, monkey);
                    } else {
                        // else, push back to the priority queue
                        priority_queue.push_back(name);
                    }
                }
            }
        }
    }

    precomputed
}

fn guess_excepted(
    monkey_name: &str,
    value: i64,
    monkeys: &HashMap<&str, Monkey>,
    precomputed: &HashMap<&str, i64>
) -> i64 {
    if monkey_name == "humn" { return value; };

    let monkey = &monkeys[monkey_name];

    // equality for the root case
    if monkey_name == "root" {
        return match monkey {
            Monkey::Num(_) => panic!("Should not happen"),
            Monkey::Calc(_, lhs, rhs) => {
                let rhs_value = precomputed.get(rhs);
                let lhs_value = precomputed.get(lhs);

                if let Some(lhs_value) = lhs_value {
                    guess_excepted(rhs, *lhs_value, &monkeys, &precomputed)
                }else if let Some(rhs_value) = rhs_value {
                    guess_excepted(lhs, *rhs_value, &monkeys, &precomputed)
                }else {
                    panic!("Should not happen")
                }
            }
        };
    }

    match monkey {
        Monkey::Num(_) => panic!("Should not happen"),
        Monkey::Calc(operator, lhs, rhs) => {
            let rhs_value = precomputed.get(rhs);
            let lhs_value = precomputed.get(lhs);
            // if the excepted monkey is on one side

            // println!("{}: {}({} {} {}) (expected value: {})",
            //     monkey_name,
            //     monkey,
            //     lhs_value.map_or("?".to_owned(), |v| v.to_string()),
            //     operator,
            //     rhs_value.map_or("?".to_owned(), |v| v.to_string()),
            //     value
            // );
            if let Some(lhs_value) = lhs_value {
                guess_excepted(
                    rhs,
                    match operator {
                        Op::Add => |a, b| a - b,
                        Op::Sub => |a, b| b - a,
                        Op::Mul => |a, b| a / b,
                        Op::Div => |a, b| b / a,
                    }(value, *lhs_value),
                    monkeys,
                    precomputed
                )
            } else if let Some(rhs_value) = rhs_value {
                guess_excepted(
                    lhs,
                    match operator {
                        Op::Add => |a, b| a - b,
                        Op::Sub => |a, b| a + b,
                        Op::Mul => |a, b| a / b,
                        Op::Div => |a, b| a * b,
                    }(value, *rhs_value),
                    monkeys,
                    precomputed
                )
            } else {
                panic!("Should not happen")
            }
        }
    }
}

fn main() {
    let monkeys = parse_monkeys(INPUT);
        
    println!("part01: {}", compute_for("root", &monkeys));

    let precomputed = precompute_except(&monkeys, "humn");
    println!("part02: {}", guess_excepted("root", 0, &monkeys, &precomputed));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "root: pppw + sjmn
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
hmdt: 32";

    #[test]
    fn parsing_test() {
        let monkeys = parse_monkeys(TEST_INPUT);
        assert_eq!(compute_for("root", &monkeys), 152);

        let precomputed = precompute_except(&monkeys, "humn");
        // dbg!(&precomputed);
        assert_eq!(guess_excepted("root", 0, &monkeys, &precomputed), 301);
    }
}
