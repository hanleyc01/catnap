#![allow(unused, dead_code)]

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"\{")]
    LBrack,
    #[regex(r"\}")]
    RBrack,
    #[regex(r"\(")]
    LParen,
    #[regex(r"\)")]
    RParen,
    #[regex(r"\\")]
    Left,
    #[regex(r"|~|")]
    Pi,
    #[regex(r"/\\")]
    BigLam,
    #[regex(r"\\/")]
    Forall,
    #[regex("->")]
    Arrow,
    #[regex(r"\+")]
    Plus,
    #[regex(r"-")]
    Minus,
    #[regex(r"\*")]
    Star,
    #[regex(r";")]
    Semicolon,
    #[regex(r"\.")]
    Dot,
    #[regex(r"=")]
    Equal,
    #[regex(r"and")]
    And,
    #[regex(r"or")]
    Or,
    #[regex(r"not")]
    Not,
    #[regex(r"<")]
    Lt,
    #[regex(r"<=")]
    Le,
    #[regex(r">")]
    Gt,
    #[regex(r">=")]
    Ge,

    #[regex(r"case")]
    Case,

    #[regex(r"at")]
    At,

    #[regex(r":=")]
    Assign,
    #[regex(r"in")]
    In,
    #[regex(r"let")]
    Let,
    #[regex(r"letrec")]
    LetRec,
    #[regex(r"data")]
    Data,

    #[regex(r"true")]
    True,

    #[regex(r"false")]
    False,

    #[regex(r"[0-9]+", |lex| lex.clone().slice().parse::<i32>().unwrap())]
    Int(i32),

    #[regex(r"[0-9]+.[0-9]+", |lex| lex.clone().slice().parse::<f32>().unwrap())]
    Float(f32),

    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.clone().slice().to_string())]
    String(String),

    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", |lex| lex.clone().slice().to_string())]
    Ident(String),

    #[error]
    #[regex(r" \t\r\f")]
    Error,
}
