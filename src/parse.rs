use std::ops::Range;

use crate::concrete::*;
use crate::lex::Token;

peg::parser! {

    pub grammar prgrm_parse() for str {
        rule _ = quiet!{[' ' | '\n' | '\t']*}

        rule true_parser() -> Literal
            = "true" { Literal::Bool(true) }

        rule false_parse() -> Literal
            = "false" { Literal::Bool(false) }

        rule int_parser() -> Literal
            = v:$(['0'..='9']+) { Literal::Int(v.parse::<i32>().unwrap()) }

        rule float_parser() -> Literal
            = v:$(['0'..='9']+ "." ['0'..='9']+) { Literal::Float(v.parse::<f32>().unwrap()) }

        rule string_parser() -> Literal
            = v:$("\"" ([^'"'|'\\'] / r"\\t" / r"\\u" / r"\\n" / "\"")* "\"")
            { Literal::String(v.to_string()) }

        rule wild() -> Var
            = _ "_" _ { Var::Wild }

        rule ident() -> Var
            = _ v:$(['_' | 'a'..='z' | 'A'..='Z'] ['_' | 'a'..='z' | 'A'..='Z' | '0'..='9']*) _
            { Var::Ident(v.to_string()) }

        rule var() -> Var
            = (ident() / wild())

        rule literal() -> Literal
            = (true_parser() / false_parse() / float_parser() / int_parser() / string_parser())

        rule aexpr_var() -> AExpr
            = tv:tvar()
            { AExpr::Var(tv) }

        rule aexpr_literal() -> AExpr
            = l:literal()
            { AExpr::Literal(l) }

        rule aexpr_star() -> AExpr
            = "*" { AExpr::Star }

        rule aexpr_box() -> AExpr
            = "box" { AExpr::Box }

        rule aexpr_paren() -> AExpr
            = "(" _ a:aexpr() _ ")"
            { a }

        #[cache_left_rec]
        rule aexpr() -> AExpr
            = aexpr_box() / aexpr_star() / aexpr_var() / aexpr_literal() / aexpr_paren()

        rule explicit_tvar() -> TVar
            = v:var() _ ":" _ aexp:aexpr()
            { TVar::Explicit(v, Box::new(aexp)) }

        rule implicit_tvar() -> TVar
            = v:var()
            { TVar::Implicit(v) }

        rule tvar() -> TVar
            = explicit_tvar() / implicit_tvar()

        rule bexpr() -> BExpr
            = aa:(aexpr() ** _)
            { BExpr::App(aa) }
            / "(" _ aa:(aexpr() ** _) _ ")"
            { BExpr::App(aa) }

        rule expr_lambda() -> Expr
            = "\\" _ tvars:(tvar() ** _) _ "." _ e:expr()
            { Expr::Lam(tvars, Box::new(e)) }

        rule expr_bexpr() -> Expr
            = b:bexpr()
            { Expr::BExpr(b) }

        rule expr_pi() -> Expr
            = "|~|" _ tvars:(tvar() ** _) _ "." _ e:expr()
            { Expr::Pi(tvars, Box::new(e)) }

        rule expr_biglam() -> Expr
            = "/\\" _ tvars:(tvar() ** _) _ "." _ e:expr()
            { Expr::BigLam(tvars, Box::new(e)) }

        rule expr_forall() -> Expr
            = "\\/" _ tvars:(tvar() ** _) _ "." _ e:expr()
            { Expr::Forall(tvars, Box::new(e)) }

        rule expr_arrow() -> Expr
            = b:bexpr() _ "->" _ e:expr()
            { Expr::Arrow(Box::new(b), Box::new(e)) }

        #[cache_left_rec]
        rule expr_localdec() -> Expr
            = v:vdecl() _ "in" _ e:expr()
            { Expr::LocalDec(v, Box::new(e)) }

        rule expr_caseat() -> Expr
            = "case" _ e:expr() _  "of" _ "{"
                _ aa:(alt() ** (_ ";" _)) _ "}" _ "at"
                _ "{" _ axs:(aexpr() ** (_ ";" _)) _ "}"
            { Expr::CaseAt(Box::new(e), aa, axs) }

        rule expr_case() -> Expr
            = "case" _  e:expr() _  "of" _ "{"
            _ aa:(aexpr() ** (_ ";" _)) _ "}"
            { Expr::Case(Box::new(e), aa) }


        #[cache_left_rec]
        rule expr() -> Expr
            = expr_caseat() / expr_case() / expr_bexpr() / expr_lambda() / expr_biglam() / expr_pi()
            / expr_forall() / expr_arrow() / expr_localdec() 

        rule alt_arrow() -> Alt
            = p:pat() _ "->" _ e:expr()
            { Alt::Arrow(p, e) }

        rule alt_many() -> Alt
            = p:pat() _ tvars:(tvar() ** _) _ "->" _ e:expr()
            { Alt::Many(p, tvars, e) }

        rule alt() -> Alt
            = alt_arrow() / alt_many()

        rule pat_tvar() -> Pat
            = t:tvar()
            { Pat::TVar(t) }

        rule pat_lit() -> Pat
            = l:literal()
            { Pat::Lit(l) }

        rule pat() -> Pat
            = pat_tvar() / pat_lit()

        rule vdecl_let() -> VDecl
            = "let" _ "{" _ b:bind() _ "}"
            { VDecl::Let(b) }

        rule vedecl_letrec() -> VDecl
            = "letrec" _ "{" _ bs:(bind() ** _) _ "}"
            { VDecl::Letrec(bs) }

        rule vdecl() -> VDecl
            = vdecl_let() / vedecl_letrec()

        rule bind() -> Bind
            = t:tvar() _ "=" _ e:expr()
            { Bind::Assign(t, Box::new(e)) }

        rule tdecl() -> TDecl
            = "data" _ tv:tvar() _ "=" _ "{" _
            tvars:(tvar() ** _) _ "}"
            { TDecl::new(tv, tvars) }

        pub rule program() -> Program
            = ts:(tdecl() **<0,> _) _ vs:(vdecl() ** _) ![_]
            { Program::new(ts, vs) }
    }

}
