pub trait MathOperators : std::marker::Sized + std::ops::Add + std::fmt::Display {}
impl<T> MathOperators for T where T : std::marker::Sized + std::ops::Add<Output=T> + std::fmt::Display {}

pub enum SymbolicData<T> where T : MathOperators {
    Equation(SymbolicEquation<T>),
    Variable(SymbolicVariable<T>),
}

pub enum Operation {
    VAR,
    ADD,
}

pub struct SymbolicVariable<T> where T : MathOperators {
    name: String,
    phantom: std::marker::PhantomData<T>,
}

impl<T> SymbolicVariable<T> where T : MathOperators {
    pub fn new(name: String) -> SymbolicVariable<T> {
        SymbolicVariable{name, phantom: std::marker::PhantomData}
    }
}

pub struct SymbolicEquation<T> where T : MathOperators {
    op: Operation,
    operands: Vec<SymbolicData<T>>,
}

impl<T> SymbolicEquation<T> where T : MathOperators {
    pub fn new(op: Operation, operands: Vec<SymbolicData<T>>) -> SymbolicEquation<T> {
        SymbolicEquation{op, operands}
    }

    pub fn new_from_var(variable: SymbolicVariable<T>) -> SymbolicEquation<T> {
        SymbolicEquation::new(Operation::VAR, vec![
          SymbolicData::Variable(variable)
        ])
    }
}

impl<T> std::fmt::Display for SymbolicData<T> where T : MathOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SymbolicData::Equation(equation) => write!(f, "{}", equation),
            SymbolicData::Variable(variable) => write!(f, "{}", variable),
        }
    }
}

impl<T> std::fmt::Display for SymbolicVariable<T> where T : MathOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}:{}>", self.name, std::mem::size_of::<T>())
    }
}

impl<T> std::fmt::Display for SymbolicEquation<T> where T : MathOperators {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.op {
            Operation::VAR => write!(f, "{}", self.operands[0]),
            Operation::ADD => write!(f, "(ADD {} {})", self.operands[0], self.operands[1]),
        }
    }
}

impl<T> std::ops::Add for SymbolicVariable<T> where T : MathOperators {
    type Output = SymbolicEquation<T>;

    fn add(self, other: SymbolicVariable<T>) -> SymbolicEquation<T> {
        SymbolicEquation::new(Operation::ADD, vec![
          SymbolicData::Equation(SymbolicEquation::new_from_var(self)),
          SymbolicData::Equation(SymbolicEquation::new_from_var(other)),
        ])
    }
}

impl<T> std::ops::Add<SymbolicEquation<T>> for SymbolicVariable<T> where T : MathOperators {
    type Output = SymbolicEquation<T>;

    fn add(self, other: SymbolicEquation<T>) -> SymbolicEquation<T> {
        let mut new_operands = vec![SymbolicData::Variable(self)];
        new_operands.extend(other.operands);
        SymbolicEquation::new(Operation::ADD, new_operands)
    }
}


impl<T> std::ops::Add for SymbolicEquation<T> where T : MathOperators {
    type Output = SymbolicEquation<T>;

    fn add(self, other: SymbolicEquation<T>) -> SymbolicEquation<T> {
        let mut new_operands : Vec<SymbolicData<T>> = vec![];
        new_operands.extend(self.operands);
        new_operands.extend(other.operands);
        SymbolicEquation::new(Operation::ADD, new_operands)
    }
}

impl<T> std::ops::Add<SymbolicVariable<T>> for SymbolicEquation<T> where T : MathOperators {
    type Output = SymbolicEquation<T>;

    // TODO: assumes commutative
    fn add(self, other: SymbolicVariable<T>) -> SymbolicEquation<T> {
        other + self
    }
}
