use std::{fmt::Display, collections::HashMap, rc::Rc};

use super::super::day::Day;

pub struct Day21
{
    monkeys: HashMap<String, Monkey>,
}

impl Day21 {
    pub fn new() -> Day21
    {
        let input = include_str!("input21");
        //let input = include_str!("input21_example");

        let monkeys = input.trim().lines()
            .map(Monkey::from_str).collect();

        Day21 { monkeys }
    }

    fn resolve(&self, target: &str) -> Value {
        let target_monkey = self.monkeys.get(target).unwrap().clone();

        if let Some(value) = target_monkey.value {
            value
        } else if let Some(op) = &target_monkey.operation {
            let left_value = self.resolve(&op.left);
            let right_value = self.resolve(&op.right);

            op.op_type.apply(left_value, right_value)
        } else {
            panic!("Didn't have a value or an operation!");
        }
    }

    fn find_x(&self, mut lhs: Rc<Value>, mut rhs: Rc<Value>) -> isize {
        // lets make sure that left is the known, and right is the expression
        if matches!(&lhs.as_ref(), Value::Expression(_,_,_)) {
            std::mem::swap(&mut lhs, &mut rhs);
        }

        loop {
            match (lhs.as_ref(), rhs.as_ref()) {
                (Value::Known(l), Value::Unknown) => return *l,
                (Value::Known(target), Value::Expression(op, l, r)) => {
                    let (new_target, new_rhs) = simplify(op, l.as_ref().to_owned(), r.as_ref().to_owned(), *target);
                    lhs = Rc::new(Value::Known(new_target));
                    rhs = Rc::new(new_rhs);
                }
                _ => panic!("Unexpected expression layout"),
            }
        }
    }
}

fn simplify(op: &OperationType, mut l: Value, mut r: Value, mut target: isize) -> (isize, Value) {
    // lets make sure that left is the expression, and right is the known value
    if !matches!(r, Value::Known(_)) {
        std::mem::swap(&mut l, &mut r);

        // If the operation is non-commutative, we need to invert the target as well
        // i.e., we can't turn 5 = 3 - x into 5 = x - 3, we need to turn it into -5 = x - 3 (mult both sides by -1)
        if let OperationType::Sub = op {
            target = -target;
        }
        else if let OperationType::Div = op {
            panic!("Maybe intentionally, this never comes up. We would have to 1/target which would obviously not work with integers")
        }
    }

    if let Value::Known(expression_known) = r {
        let inverse = op.inverse();
        let new_target = inverse.apply_known(target, expression_known);
        (new_target, l)
    } else {
        panic!("Swap didn't work");
    }
}

impl Day for Day21 {
    fn day_name(&self) -> String { String::from("21") }
    fn answer1(&self) -> String { String::from("31017034894002") }
    fn answer2(&self) -> String { String::from("3555057453229") }

    fn part1(&mut self) -> String {
        self.resolve("root").to_string()
    }

    fn part2(&mut self) -> String {
        // the 'humn' monkey is actually me, I return an unknown
        self.monkeys.get_mut("humn").unwrap().value = Some(Value::Unknown);

        if let Value::Expression(_, l, r) = self.resolve("root") {
            self.find_x(l, r).to_string()
        } else {
            panic!("Second solve did not return an expression...")
        }
    }
}

#[derive(Clone)]
struct Monkey {
    id: String,
    operation: Option<Operation>,
    value: Option<Value>,
}


#[derive(Clone)]
struct Operation {
    op_type: OperationType, 
    left: String, 
    right: String,
}

#[derive(Clone, Copy, PartialEq)]
enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, PartialEq)]
enum Value {
    Known(isize), Unknown, Expression(OperationType, Rc<Value>, Rc<Value>)
}

impl Monkey {
    fn from_str(input: &str) -> (String, Monkey) {
        let (id, job) = input.split_once(": ").unwrap();
        let id = id.to_string();
        let (operation, value) = Operation::from_str(job);
        (id.to_string(), Monkey { id, operation, value })
    }
 }

impl Operation {
    fn from_str(input: &str) -> (Option<Operation>, Option<Value>) {
        if !input.contains(' ') {
            (None, Some(Value::Known(input.parse().unwrap())))
        } else {
            let parts = input.split(' ').collect::<Vec<_>>();
            let left = parts[0].to_string();
            let op_type = OperationType::from_str(parts[1]);
            let right = parts[2].to_string();
            (Some(Operation { op_type, left, right }), None)
        }
    }
}

impl OperationType {
    fn from_str(input: &str) -> OperationType {
        match input {
            "+" => OperationType::Add,
            "-" => OperationType::Sub,
            "*" => OperationType::Mul,
            "/" => OperationType::Div,
            _ => panic!("Invalid operation type"),            
        }
    }

    fn apply(&self, left: Value, right: Value) -> Value {
        // if both are known values, collapse into known value, otherwise we have to build an expression
        match (left, right) {
            (Value::Known(l), Value::Known(r)) => Value::Known(self.apply_known(l, r)),
            (left, right) => Value::Expression(*self, Rc::new(left), Rc::new(right)),
        }
    }

    fn apply_known(&self, left: isize, right: isize) -> isize {
        match self {
            OperationType::Add => left + right,
            OperationType::Sub => left - right,
            OperationType::Mul => left * right,
            OperationType::Div => left / right,
        }
    }

    pub(crate) fn inverse(&self) -> OperationType {
        match self {
            OperationType::Add => OperationType::Sub,
            OperationType::Sub => OperationType::Add,
            OperationType::Mul => OperationType::Div,
            OperationType::Div => OperationType::Mul,
        }
    }
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OperationType::Add => "+",
            OperationType::Sub => "-",
            OperationType::Mul => "*",
            OperationType::Div => "/",
        })
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.op_type, self.right)
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}{}", 
            self.id, 
            if let Some(job) = &self.operation { job.to_string() } else { "".to_string() }, 
            if let Some(v) = &self.value { v.to_string() } else { "".to_string() })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Value::Known(i) => i.to_string(),
            Value::Unknown => "x".to_string(),
            Value::Expression(op, left, right) => format!("({left} {op} {right})")
        })
    }
}