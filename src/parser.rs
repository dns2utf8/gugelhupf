use super::*;



#[derive(Eq,PartialEq,Debug,Copy,Clone)]
enum Keyword {
    Undefined, Null,
    Var, Let, Const,
    Function,
    If, While, For,
}

named!(undefined_lit<Keyword>, do_parse!(
    tag!("undefined") >>
    (Keyword::Undefined)
    ));
named!(null_lit<Keyword>, do_parse!(
    tag!("null") >>
    (Keyword::Null)
    ));

named!(var_lit<Keyword>, do_parse!(
    tag!("var") >>
    (Keyword::Var)
    ));
named!(let_lit<Keyword>, do_parse!(
    tag!("let") >>
    (Keyword::Let)
    ));
named!(const_lit<Keyword>, do_parse!(
    tag!("const") >>
    (Keyword::Const)
    ));

named!(function_lit<Keyword>, do_parse!(
    tag!("function") >>
    (Keyword::Function)
    ));

named!(if_lit<Keyword>, do_parse!(
    tag!("if") >>
    (Keyword::If)
    ));
named!(if_while<Keyword>, do_parse!(
    tag!("while") >>
    (Keyword::While)
    ));
named!(for_lit<Keyword>, do_parse!(
    tag!("for") >>
    (Keyword::For)
    ));


#[derive(Eq,PartialEq,Debug,Copy,Clone)]
enum Operator {
    Plus, Minus, Mul, Div
}

named!(plus_op<Operator>, do_parse!(
    tag!("+") >>
    (Operator::Plus)
    ));
named!(minus_op<Operator>, do_parse!(
    tag!("-") >>
    (Operator::Minus)
    ));
named!(mul_op<Operator>, do_parse!(
    tag!("*") >>
    (Operator::Mul)
    ));
named!(div_op<Operator>, do_parse!(
    tag!("/") >>
    (Operator::Div)
    ));


#[cfg(test)]
mod test {
    use super::*;

    /*
    const SIMPLE_FUNCTION: &str = "function simple() { }";

    #[test]
    fn parse_simple_function() {
    }*/

    #[test]
    fn parse_undefined_literal() {
        let src = "undefined";
        let ast = undefined_lit(src.as_bytes());

        assert_eq!(Keyword::Undefined, ast.unwrap().1);
    }

    #[test]
    fn parse_function_literal() {
        let src = "function";
        let ast = function_lit(src.as_bytes());

        assert_eq!(Keyword::Function, ast.unwrap().1);
    }

    #[test]
    fn parse_null_literal() {
        let src = "null";
        let ast = null_lit(src.as_bytes());

        assert_eq!(Keyword::Null, ast.unwrap().1);
    }
}
