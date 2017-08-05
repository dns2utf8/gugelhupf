use super::*;


named!(undefined<Keyword>, do_parse!(
    tag!("undefined") >>
    (Keyword::Undefined)
    ));

named!(function<Keyword>, do_parse!(
    tag!("function") >>
    (Keyword::Function)
    ));

named!(null<Keyword>, do_parse!(
    tag!("null") >>
    (Keyword::Null)
    ));

#[derive(Eq,PartialEq,Debug,Copy,Clone)]
enum Keyword {
    Undefined,
    Null,
    Function,
}

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
        let ast = undefined(src.as_bytes());

        assert_eq!(Keyword::Undefined, ast.unwrap().1);
    }

    #[test]
    fn parse_function_literal() {
        let src = "function";
        let ast = function(src.as_bytes());

        assert_eq!(Keyword::Function, ast.unwrap().1);
    }

    #[test]
    fn parse_null_literal() {
        let src = "null";
        let ast = null(src.as_bytes());

        assert_eq!(Keyword::Null, ast.unwrap().1);
    }
}
