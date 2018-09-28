pub trait ValueT :
    std::marker::Sized +
    std::fmt::Display +
    std::ops::Add +
    std::ops::Sub +
    std::ops::Mul +
    std::ops::Div +
    std::ops::BitAnd +
    std::ops::BitOr +
    std::ops::BitXor {}

impl<T> ValueT for T where T :
    std::marker::Sized +
    std::fmt::Display +
    std::ops::Add<Output=T> +
    std::ops::Sub<Output=T> +
    std::ops::Mul<Output=T> +
    std::ops::Div<Output=T> +
    std::ops::BitAnd<Output=T> +
    std::ops::BitOr<Output=T> +
    std::ops::BitXor<Output=T> {}

#[derive(Clone)]
pub enum Value<T> where T : ValueT {
    Concrete(ConcreteValue<T>),
    Variable(SymbolicVariable<T>),
    Equation(SymbolicEquation<T>),
}

#[derive(Clone)]
pub enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    XOR,
}

#[derive(Clone)]
pub struct ConcreteValue<T> where T : ValueT {
    value: T,
}

impl<T> ConcreteValue<T> where T : ValueT {
    fn new(value: T) -> Value<T> {
        Value::Concrete(ConcreteValue{value})
    }
}

pub fn get_concrete<T>(value: T) -> Value<T> where T : ValueT {
    ConcreteValue::new(value)
}

#[derive(Clone)]
pub struct SymbolicVariable<T> where T : ValueT {
    name: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> SymbolicVariable<T> where T : ValueT {
    fn new(name: String) -> Value<T> {
        Value::Variable(SymbolicVariable{name, phantom: std::marker::PhantomData})
    }
}

pub fn get_symbolic<T>(name: String) -> Value<T> where T : ValueT {
    SymbolicVariable::new(name)
}

#[derive(Clone)]
pub struct SymbolicEquation<T> where T : ValueT {
    op: Operation,
    operands: Vec<Value<T>>,
}

impl<T> SymbolicEquation<T> where T : ValueT {
    fn new(op: Operation, operands: Vec<Value<T>>) -> Value<T> {
        Value::Equation(SymbolicEquation{op, operands})
    }
}

impl<T> std::fmt::Display for Value<T> where T : ValueT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Concrete(concrete) => write!(f, "{}", concrete),
            Value::Equation(equation) => write!(f, "{}", equation),
            Value::Variable(variable) => write!(f, "{}", variable),
        }
    }
}

impl<T> std::fmt::Display for ConcreteValue<T> where T : ValueT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> std::fmt::Display for SymbolicVariable<T> where T : ValueT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}:{}>", self.name, std::mem::size_of::<T>() * 8)
    }
}

impl<T> std::fmt::Display for SymbolicEquation<T> where T : ValueT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.op {
            Operation::ADD => write!(f, "(ADD {} {})", self.operands[0], self.operands[1]),
            Operation::SUB => write!(f, "(SUB {} {})", self.operands[0], self.operands[1]),
            Operation::MUL => write!(f, "(MUL {} {})", self.operands[0], self.operands[1]),
            Operation::DIV => write!(f, "(DIV {} {})", self.operands[0], self.operands[1]),
            Operation::AND => write!(f, "(AND {} {})", self.operands[0], self.operands[1]),
            Operation::OR => write!(f, "(OR {} {})", self.operands[0], self.operands[1]),
            Operation::XOR => write!(f, "(XOR {} {})", self.operands[0], self.operands[1]),
        }
    }
}

impl<T> std::ops::Add for Value<T> where T : ValueT + std::ops::Add<Output=T> {
    type Output = Value<T>;

    fn add(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value + rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::ADD, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::Sub for Value<T> where T : ValueT + std::ops::Sub<Output=T> {
    type Output = Value<T>;

    fn sub(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value - rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::SUB, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::Mul for Value<T> where T : ValueT + std::ops::Mul<Output=T> {
    type Output = Value<T>;

    fn mul(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value * rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::MUL, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::Div for Value<T> where T : ValueT + std::ops::Div<Output=T> {
    type Output = Value<T>;

    fn div(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value / rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::DIV, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::BitAnd for Value<T> where T : ValueT + std::ops::BitAnd<Output=T> {
    type Output = Value<T>;

    fn bitand(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value & rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::AND, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::BitOr for Value<T> where T : ValueT + std::ops::BitOr<Output=T> {
    type Output = Value<T>;

    fn bitor(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value | rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::OR, operands: new_operands})
            },
        }
    }
}

impl<T> std::ops::BitXor for Value<T> where T : ValueT + std::ops::BitXor<Output=T> {
    type Output = Value<T>;

    fn bitxor(self, other: Value<T>) -> Value<T> {
        match (self, other) {
            (Value::Concrete(lhs), Value::Concrete(rhs)) =>
                Value::Concrete(ConcreteValue{value: lhs.value ^ rhs.value}),
            (Value::Concrete(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicVariable::new(rhs.name)
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            }
            (Value::Concrete(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    ConcreteValue::new(lhs.value),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },

            (Value::Variable(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },
            (Value::Variable(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicVariable::new(lhs.name),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },

            (Value::Equation(lhs), Value::Concrete(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    ConcreteValue::new(rhs.value),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Variable(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicVariable::new(rhs.name),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },
            (Value::Equation(lhs), Value::Equation(rhs)) => {
                let new_operands = vec![
                    SymbolicEquation::new(lhs.op, lhs.operands),
                    SymbolicEquation::new(rhs.op, rhs.operands),
                ];
                Value::Equation(SymbolicEquation{op: Operation::XOR, operands: new_operands})
            },
        }
    }
}
