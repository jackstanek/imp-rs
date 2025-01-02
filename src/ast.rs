use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
}

impl BinOp {
    pub fn into_fn(self) -> impl Fn(i64, i64) -> i64 {
        match self {
            BinOp::Add => std::ops::Add::add,
            BinOp::Sub => std::ops::Sub::sub,
            BinOp::Mult => std::ops::Mul::mul,
            BinOp::Div => std::ops::Div::div,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AExpr {
    Val(i64),
    Var(String),
    BinOp(BinOp, Box<AExpr>, Box<AExpr>),
    Neg(Box<AExpr>),
}

impl AExpr {
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::BinOp(BinOp::Add, Box::new(lhs), Box::new(rhs))
    }
    pub fn sub(lhs: Self, rhs: Self) -> Self {
        Self::BinOp(BinOp::Sub, Box::new(lhs), Box::new(rhs))
    }
    pub fn mult(lhs: Self, rhs: Self) -> Self {
        Self::BinOp(BinOp::Mult, Box::new(lhs), Box::new(rhs))
    }
    pub fn div(lhs: Self, rhs: Self) -> Self {
        Self::BinOp(BinOp::Div, Box::new(lhs), Box::new(rhs))
    }
}

impl From<String> for AExpr {
    fn from(value: String) -> Self {
        Self::Var(value)
    }
}

impl From<i64> for AExpr {
    fn from(value: i64) -> Self {
        Self::Val(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cmp {
    Eq,
    Neq,
    Le,
    Lt,
    Ge,
    Gt,
}

impl Cmp {
    pub fn compare(&self, l: i64, r: i64) -> bool {
        match self {
            Cmp::Eq => l == r,
            Cmp::Neq => l != r,
            Cmp::Le => l <= r,
            Cmp::Lt => l < r,
            Cmp::Ge => l >= r,
            Cmp::Gt => l > r,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BExpr {
    Bool(bool),
    And(Box<BExpr>, Box<BExpr>),
    Or(Box<BExpr>, Box<BExpr>),
    Not(Box<BExpr>),
    Cmp(Cmp, AExpr, AExpr),
}

impl BExpr {
    pub fn and(lhs: Self, rhs: Self) -> Self {
        Self::And(Box::new(lhs), Box::new(rhs))
    }

    pub fn or(lhs: Self, rhs: Self) -> Self {
        Self::Or(Box::new(lhs), Box::new(rhs))
    }

    pub fn not(term: Self) -> Self {
        Self::Not(Box::new(term))
    }

    pub fn cmp(cmp: Cmp, lhs: AExpr, rhs: AExpr) -> Self {
        Self::Cmp(cmp, lhs, rhs)
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Skip,
    Asgn(String, AExpr),
    Seq(Vec<Stmt>),
    If(BExpr, Box<Stmt>, Box<Stmt>),
    While(BExpr, Box<Stmt>),
}

impl<'stmt> IntoIterator for &'stmt Stmt {
    type Item = &'stmt Stmt;

    type IntoIter = StmtIter<&'stmt Stmt>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub struct StmtIter<T> {
    stmts: VecDeque<T>,
}

impl<T> Iterator for StmtIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stmts.pop_front()
    }
}

impl<'stmt> StmtIter<&'stmt Stmt> {
    pub fn new(stmts: &'stmt Stmt) -> Self {
        Self {
            stmts: if let Stmt::Seq(seq) = stmts {
                seq.iter().collect()
            } else {
                let mut vec = VecDeque::new();
                vec.push_back(stmts);
                vec
            },
        }
    }
}

impl Stmt {
    pub fn ite(if_: BExpr, then_: Self, else_: Self) -> Self {
        Self::If(if_, Box::new(then_), Box::new(else_))
    }
}

impl From<Vec<Stmt>> for Stmt {
    fn from(value: Vec<Stmt>) -> Self {
        Self::Seq(value)
    }
}
