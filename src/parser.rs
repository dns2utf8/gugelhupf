use super::*;

//named!(function)
named!(undefined, tag!("undefined")
    );

#[derive(Eq,PartialEq,Debug,Copy,Clone)]
enum Keyword {
    Undefined,
    Null,
    Function,
}

#[cfg(test)]
mod test {
    use super::*;

    const SIMPLE_FUNCTION: &str = "function simple() { }";

    #[test]
    fn parse_simple_function() {
    }

    fn parse_undefined_literal() {
        let src = "undefined";
        let ast = undefined(src.as_bytes());

        assert_eq!(Keyword::Undefined, ast.unwrap().1);
    }
}
