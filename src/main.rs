#![allow(unused, dead_code)]

//! The core of the language of Catnap will be based
//! on the [Henk](https://www.microsoft.com/en-us/research/wp-content/uploads/1997/01/henk.pdf)
//! intermediate language

use lalrpop_util::*;

pub mod concrete;
lalrpop_mod!(pub catnap_grammar);

use std::fs::read_to_string;

// Cool shit: "Debugging a compiler can be anightmare. The only evidence of
// an incorrect transformation can be a segmentation fault in a large program
// compiled by the compiler. Identifying the cause of the crash, and tracing
// it back to the faulty transformation bugs, is a long, slow business. If, instead,
// the the compile type-checks the intermediate-language program after every transformation,
// the huge majority of transformation bugs can be nailed in short order."

fn main() {
    let fname = "test.ctnp";
    let conts = read_to_string(fname).unwrap();
    dbg!(&conts);
    let vdecl = catnap_grammar::ProgramParser::new()
        .parse(&conts)
        .unwrap();
    dbg!(vdecl);
}
