use crate::solution::{Example, StructSolution};
use crate::types::AdNum;
use crate::DayData;
use get_size::GetSize;
use num::{BigUint, Integer, Num, Zero};
use std::collections::HashMap;
use std::mem;
use std::num::ParseIntError;
use std::ops::Rem;
use std::str::FromStr;

type Input1<T> = Vec<Monkey<T>>;
type Output1 = usize;
type Input2 = Vec<Monkey2>;
type Output2 = usize;

#[derive(Debug)]
pub struct Item {
    multipliers: HashMap<u16, u16>,
}

impl Item {
    pub fn new(primes: &Vec<u16>, n: &u16) -> Item {
        let m: Vec<(u16, u16)> = primes.iter().map(|p| (*p, n.mod_floor(p))).collect();
        Item {
            multipliers: HashMap::from_iter(m),
        }
    }

    pub fn multiply(&mut self, n: u16) {
        for (k, mut v) in self.multipliers.iter_mut() {
            *v = (n * *v).mod_floor(&k);
        }
    }

    pub fn add(&mut self, n: u16) {
        for (k, mut v) in self.multipliers.iter_mut() {
            *v = (n + *v).mod_floor(&k);
        }
    }

    pub fn square(&mut self) {
        for (k, mut v) in self.multipliers.iter_mut() {
            *v = (*v * *v).mod_floor(k).try_into().unwrap();
        }
    }

    pub fn divisible(&self, n: u16) -> bool {
        self.multipliers
            .get(&n)
            .expect("Multiplier doesn't exist")
            .is_zero()
    }

    pub fn operate(&mut self, operation: &Operation<u8>) {
        match operation.operator {
            Operator::Mul => match (&operation.left, &operation.right) {
                (Operand::Old, Operand::Old) => self.square(),
                (Operand::Old, Operand::Const { val }) => self.multiply(val.clone() as u16),
                _ => panic!("Unexpected test"),
            },
            Operator::Add => match (&operation.left, &operation.right) {
                (Operand::Old, Operand::Old) => self.multiply(2),
                (Operand::Old, Operand::Const { val }) => self.add(val.clone() as u16),
                _ => panic!("Unexpected test"),
            },
        }
    }
}

#[derive(Debug)]
pub enum Operand<T> {
    Old,
    Const { val: T },
}

impl<T: FromStr> FromStr for Operand<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            x => {
                let val = x.parse().or(Err(x.to_string()))?;
                Ok(Operand::Const { val })
            }
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Mul,
    Add,
}

impl Operator {
    pub fn operate<T: AdNum>(&self, left: T, right: T) -> T {
        let res = match &self {
            Operator::Mul => right.mul(left.clone()),
            Operator::Add => right.add(left.clone()),
        };
        // if (res as u32) > u32::MAX {
        //     println!("{:?}", res);
        // }
        res
    }
}

impl FromStr for Operator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Operator::Mul),
            "+" => Ok(Operator::Add),
            _ => Err(s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Operation<T> {
    left: Operand<T>,
    right: Operand<T>,
    operator: Operator,
}

impl<T: AdNum> Operation<T> {
    pub fn call(&self, old: T) -> T {
        match self.operator {
            Operator::Mul => match (&self.left, &self.right) {
                (Operand::Old, Operand::Old) => num::pow(old, 2),
                (Operand::Old, Operand::Const { val }) => old.mul(val.clone()),
                _ => panic!("Unexpected test"),
            },
            Operator::Add => match (&self.left, &self.right) {
                (Operand::Old, Operand::Old) => old.mul(T::one() + T::one()),
                (Operand::Old, Operand::Const { val }) => old.add(val.clone()),
                _ => panic!("Unexpected test"),
            },
        }
    }
}

impl<T: AdNum> FromStr for Operation<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //0          1   2 3   4 5
        //Operation: new = old * 19
        let mut split = s.split_whitespace();
        // Pretty ugly but can't index and .advance_by() is unstable
        split.next();
        split.next();
        split.next();
        let left = split.next().unwrap().parse()?;
        let operator = split.next().unwrap().parse()?;
        let right = split.next().unwrap().parse()?;
        Ok(Operation {
            left,
            operator,
            right,
        })
    }
}

#[derive(Debug)]
pub struct MonkeyTest<T> {
    divisible_by: T,
    true_monkey: usize,
    false_monkey: usize,
}

impl<T: AdNum> FromStr for MonkeyTest<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    where
        T: AdNum,
    {
        // Test: divisible by 23
        //   If true: throw to monkey 2
        //   If false: throw to monkey 3
        let mut lines_iter = s.lines();
        let test_line = lines_iter.next().unwrap();
        let x = "  Test: divisible by ";
        let divisible_by = if test_line.contains(x) {
            match test_line[x.len()..].parse() {
                Err(_) => panic!("Error"),
                Ok(x) => x,
            }
        } else {
            return Err(());
            // return Err(format!("Unexpected error"));
            // return Err(format!("Unexpected: '{}'", &test_line[x.len()..]));
        };
        let true_monkey = lines_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let false_monkey = lines_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Ok(MonkeyTest {
            divisible_by,
            true_monkey,
            false_monkey,
        })
    }
}

#[derive(Debug)]
pub struct Monkey<T> {
    items: Vec<T>,
    operation: Operation<T>,
    test: MonkeyTest<T>,
    inspections: T,
}

impl<T: AdNum> FromStr for Monkey<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // All the .ok() are because of an incredibly annoying type restriction for generics
        let (first, two_on) = s.split_once("\n").unwrap();
        let (second, three_on) = two_on.split_once("\n").unwrap();
        let (third, rest) = three_on.split_once("\n").unwrap();
        let (_, item_str) = second.split_once(": ").unwrap();
        let items: Vec<T> = item_str
            .split(", ")
            .map(|f| f.parse().ok().unwrap())
            .collect();
        let operation = third.parse().unwrap();
        let test = rest.parse().unwrap();
        Ok(Monkey {
            items,
            operation,
            test,
            inspections: T::zero(),
        })
    }
}

#[derive(Debug)]
pub struct Monkey2 {
    items: Vec<Item>,
    operation: Operation<u8>,
    test: MonkeyTest<u8>,
    inspections: usize,
}

pub fn prepare<T: AdNum>(input: String) -> Input1<T> {
    let monkeys = input.split("\n\n");
    monkeys.map(|m| m.parse().unwrap()).collect()
}

pub fn prepare_2(input: String) -> Input2 {
    let old_monkeys = prepare::<u8>(input);
    let primes = old_monkeys
        .iter()
        .map(|m| m.test.divisible_by as u16)
        .collect();
    old_monkeys
        .into_iter()
        .map(|m| Monkey2 {
            items: m
                .items
                .iter()
                .map(|it| Item::new(&primes, &(*it as u16)))
                .collect(),
            operation: m.operation,
            test: m.test,
            inspections: 0,
        })
        .collect()
}

fn divide_worry<T: AdNum>(input: T, worry_divider: &T) -> T {
    if worry_divider.is_one() {
        input
    } else {
        input.div_floor(worry_divider)
    }
}

fn handle_monkey<T: AdNum>(monkey: &mut Monkey<T>, worry_divider: &T) -> Vec<(usize, T)> {
    let items = mem::replace(&mut monkey.items, Vec::new());
    let worries = items
        .into_iter()
        // .map(|w| monkey.operation.call(w).div(worry_divider.into()));
        .map(|w| divide_worry(monkey.operation.call(w), worry_divider));
    let mut outputs = Vec::new();
    for item in worries {
        let test = item.divides(&monkey.test.divisible_by);
        let dest = if test {
            monkey.test.true_monkey
        } else {
            monkey.test.false_monkey
        };
        outputs.push((dest, item));
        let ins = mem::replace(&mut monkey.inspections, T::zero());
        monkey.inspections = ins + T::one();
    }
    outputs
}

fn handle_monkey_2(monkey: &mut Monkey2) -> Vec<(usize, Item)> {
    let items = mem::replace(&mut monkey.items, Vec::new());
    let mut outputs = Vec::new();
    for mut item in items {
        item.operate(&monkey.operation);
        let test = item.divisible(monkey.test.divisible_by as u16);
        let dest = if test {
            monkey.test.true_monkey
        } else {
            monkey.test.false_monkey
        };
        outputs.push((dest, item));
        monkey.inspections += 1;
    }
    outputs
}

fn make_usize<T: AdNum>(input: T) -> usize {
    input.to_string().parse().unwrap()
}

fn run_simulation<T: AdNum>(mut monkeys: Input1<T>, rounds: usize, worry_divider: T) -> usize {
    for round in 0..rounds {
        for monkey_i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[monkey_i];
            // println!("Monkey: {:?}", monkey);
            let outputs = handle_monkey(&mut monkey, &worry_divider);
            for (i, item) in outputs {
                // println!("{}", mem::size_of(item));
                monkeys[i].items.push(item)
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());
    make_usize(monkeys.pop().unwrap().inspections * monkeys.pop().unwrap().inspections)
}

fn run_simulation_2(mut monkeys: Input2, rounds: usize) -> usize {
    for round in 0..rounds {
        for monkey_i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[monkey_i];
            // println!("Monkey: {:?}", monkey);
            let outputs = handle_monkey_2(&mut monkey);
            for (i, item) in outputs {
                // println!("{}", mem::size_of(item));
                monkeys[i].items.push(item)
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| a.inspections.partial_cmp(&b.inspections).unwrap());
    make_usize(monkeys.pop().unwrap().inspections * monkeys.pop().unwrap().inspections)
}

pub fn part_1<T: AdNum>(mut input: Input1<T>) -> Output1 {
    run_simulation(input, 20, T::one() + T::one() + T::one())
}

pub fn part_2(input: Input2) -> Output2 {
    run_simulation_2(input, 10000)
}

pub fn make_sol() -> StructSolution<Input1<u16>, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1::<u16>,
        prepare_part_2: prepare_2,
        calc_part_2: part_2,
        example_part_1: Example::Value(10605),
        example_part_2: Example::Value(2713310158),
        day_data: DayData::new(11, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_operation() {
        let o: Operation<usize> = "Operation: new = old * 19".parse().unwrap();
    }

    #[test]
    fn parse_test_monkey() {
        let o: MonkeyTest<u32> = "  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3".parse().unwrap();
    }

    #[test]
    fn monkey_2() {
        let mut item = Item::new(&vec![3, 5], &12);
        assert_eq!(item.divisible(3), true);
        assert_eq!(item.divisible(5), false);
        item.square();
        println!("{:?}", item);
        assert_eq!(item.divisible(3), true);
        assert_eq!(item.divisible(5), false);
        item.add(1);
        println!("{:?}", item);
        assert_eq!(item.divisible(3), false);
        assert_eq!(item.divisible(5), true);
    }
}
