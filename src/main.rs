#![allow(unused, dead_code)]

//! The core of the language of Catnap will be based
//! on the [Henk](https://www.microsoft.com/en-us/research/wp-content/uploads/1997/01/henk.pdf)
//! intermediate language

mod concrete;
mod lex;
mod parse;

// Cool shit: "Debugging a compiler can be anightmare. The only evidence of
// an incorrect transformation can be a segmentation fault in a large program
// compiled by the compiler. Identifying the cause of the crash, and tracing
// it back to the faulty transformation bugs, is a long, slow business. If, instead,
// the the compile type-checks the intermediate-language program after every transformation,
// the huge majority of transformation bufs can be nailed in short order."

/// E ::= K           CONSTANT
///     | x           VARIABLE
///     | E E         APPLICATION
///     | \x : E . E  ABSTRACTION
///     | ^x : E . E  QUANTIFICATION
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Const(String),
    Var(String),
    App(Box<Expr>, Box<Expr>),
    Abstr(Box<Expr>, Box<Expr>),
    Quant(Box<Expr>, Box<Expr>),
}

// type abstraction that binds the type variable
// a, which in turn is used as the type of x
//
// \a : * . \x : a . id a x
//
// where `*` represents the "kind" constant, thus `a : *`
// says that `a` is of the kind `type`

// ^x : A . B
// This expression is shows the dependent product ^ (pi)
// This is the type of functions from values
// of type A to values of type B, in which the result
// of type B may perhaps depend on the value of the argument,
// x; thus A \to B is an abbreviation for ^_: A . B
// Note that this also subsumes universal quantification,
// thus
//
// ^x : * . A
//
// is just the expanded form of \forall a. A

// K-combinator
// (\a : * . \b : *. \x : a . \y : b . y) : (^a : * . ^b : * . a -> b -> b)

pub enum Sort {
    Term,
    Type,
    Kind,
}

fn main() {
    println!("Hello, world!");
}
