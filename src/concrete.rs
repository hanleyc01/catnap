#![allow(dead_code, unused)]

#[derive(Debug)]
pub struct Program {
    type_decls: Vec<TDecl>,
    value_decls: Vec<VDecl>,
}

#[derive(Debug)]
pub struct TDecl {
    name: TVar,
    values: Vec<TVar>,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Int(i32),
    Bool(bool),
    Float(f32),
}

#[derive(Debug)]
pub enum VDecl {
    Let(Bind),
    Letrec(Vec<Bind>),
}

#[derive(Debug)]
pub enum Bind {
    Assign(TVar, Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    BExpr(BExpr),
    Lam(Vec<TVar>, Box<Expr>),
    Pi(Vec<TVar>, Box<Expr>),
    BigLam(Vec<TVar>, Box<Expr>),
    LitLam(Vec<TVar>, Box<Expr>),
    Arrow(Box<BExpr>, Box<Expr>),
    LocalDec(VDecl, Box<Expr>),
    Case(Box<Expr>, Vec<Alt>),
    At(Vec<AExpr>),
}

#[derive(Debug)]
pub enum BExpr {
    Appl(Box<BExpr>, AExpr),
    AExpr(AExpr),
}

#[derive(Debug)]
pub enum AExpr {
    Var(TVar),
    Literal(Literal),
    Star,
    Box,
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub enum TVar {
    Explicit(Var, Box<AExpr>),
    Implicit(Var),
}

#[derive(Debug)]
pub enum Var {
    Wild,
    Ident(String),
}

#[derive(Debug)]
pub enum Alt {
    Arrow(Pat, Expr),
    Many(Vec<TVar>, Expr),
}

#[derive(Debug)]
pub enum Pat {
    TVar(TVar),
    Lit(Literal),
}
