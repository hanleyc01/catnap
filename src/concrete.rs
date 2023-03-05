#![allow(dead_code, unused)]

#[derive(Debug, Clone)]
pub struct Program {
    type_decls: Vec<TDecl>,
    value_decls: Vec<VDecl>,
}

impl Program {
    pub fn new(type_decls: Vec<TDecl>, value_decls: Vec<VDecl>) -> Self {
        Self {
            type_decls,
            value_decls,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TDecl {
    name: TVar,
    values: Vec<TVar>,
}

impl TDecl {
    pub fn new(name: TVar, values: Vec<TVar>) -> Self {
        Self { name, values }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f32),
}

#[derive(Debug, Clone)]
pub enum VDecl {
    Let(Bind),
    Letrec(Vec<Bind>),
}

#[derive(Debug, Clone)]
pub enum Bind {
    Assign(TVar, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    BExpr(BExpr),
    Lam(Vec<TVar>, Box<Expr>),
    Pi(Vec<TVar>, Box<Expr>),
    BigLam(Vec<TVar>, Box<Expr>),
    Forall(Vec<TVar>, Box<Expr>),
    Arrow(Box<BExpr>, Box<Expr>),
    LocalDec(VDecl, Box<Expr>),
    CaseAt(Box<Expr>, Vec<Alt>, Vec<AExpr>),
    Case(Box<Expr>, Vec<AExpr>),
}

#[derive(Debug, Clone)]
pub enum BExpr {
    App(Vec<AExpr>),
}

#[derive(Debug, Clone)]
pub enum AExpr {
    Var(TVar),
    Literal(Literal),
    Star,
    Box,
    Expr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum TVar {
    Explicit(Var, Box<AExpr>),
    Implicit(Var),
}

#[derive(Debug, Clone)]
pub enum Var {
    Wild,
    Ident(String),
}

#[derive(Debug, Clone)]
pub enum Alt {
    Arrow(Pat, Expr),
    Many(Pat, Vec<TVar>, Expr),
}

#[derive(Debug, Clone)]
pub enum Pat {
    TVar(TVar),
    Lit(Literal),
}
