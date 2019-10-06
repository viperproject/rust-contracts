pub mod ast;
pub mod parser;

#[cfg(test)]
mod tests {

    use crate::parser;
    use proc_macro2::TokenStream;
    use syn;

    fn parse(spec: &str) -> syn::Result<String> {
        let tokens = spec.parse::<TokenStream>().unwrap();
        let assertion = parser::parse_assertion(tokens)?;
        Ok(format!("{}", assertion))
    }

    #[test]
    fn spaces1() {
        let p = parse("5 <= self.0 && self.0 < 10").unwrap();
        assert_eq!(p, "A(C(A(E(5 <= self . 0)) && A(E(self . 0 < 10))))");
    }

    #[test]
    fn spaces2() {
        let p = parse("5      <=      self.0 \n\n&& self   .   0 < 10").unwrap();
        assert_eq!(p, "A(C(A(E(5 <= self . 0)) && A(E(self . 0 < 10))))");
    }

    #[test]
    fn spaces3() {
        let p = parse("((5      <=      self.0) \n)\n&& self   .   0 < 10").unwrap();
        assert_eq!(p, "A(C(A(E(5 <= self . 0)) && A(E(self . 0 < 10))))");
    }

    #[test]
    fn conjunctions_and_expression1() {
        let p = parse("a && b && c && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(E(b)) && A(E(c)) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn conjunctions_and_expression2() {
        let p = parse("a && (b && c) && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(C(A(E(b)) && A(E(c)))) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn conjunctions_and_expression3() {
        let p = parse("a && (b && c) + 1 && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(E(( b && c ) + 1)) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn conjunctions_and_expression4() {
        let p = parse("a && 1 + (b && c) && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(E(1 + ( b && c ))) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn conjunctions_and_expression5() {
        let p = parse("a && 1 + (b && c) + 1&& d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(E(1 + ( b && c ) + 1)) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn conjunctions_and_expression_err1() {
        let p = parse("a && (b) c && d && e");
        assert!(p.is_err());
    }

    #[test]
    fn conjunctions_and_expression_err2() {
        let p = parse("a &&");
        assert!(p.is_err());
    }

    #[test]
    fn conjunctions_and_expression_err3() {
        let p = parse("&& b");
        assert!(p.is_err());
    }

    #[test]
    fn implications1() {
        let p = parse("a && b ==> c && d && e").unwrap();
        assert_eq!(
            p,
            "A(I(A(C(A(E(a)) && A(E(b)))) ==> A(C(A(E(c)) && A(E(d)) && A(E(e))))))"
        );
    }

    #[test]
    fn implications2() {
        let p = parse("a && (b ==> c) && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(I(A(E(b)) ==> A(E(c)))) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn implications3() {
        let p = parse("(a && b ==> c) && d && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(I(A(C(A(E(a)) && A(E(b)))) ==> A(E(c)))) && A(E(d)) && A(E(e))))"
        );
    }

    #[test]
    fn implications4() {
        let p = parse("a && (b ==> c && d) && e").unwrap();
        assert_eq!(
            p,
            "A(C(A(E(a)) && A(I(A(E(b)) ==> A(C(A(E(c)) && A(E(d)))))) && A(E(e))))"
        );
    }

    #[test]
    fn implications_err1() {
        let p = parse("a && b (==> c && d) && e");
        assert!(p.is_err());
    }

    #[test]
    fn implications_err2() {
        let p = parse("a && (b ==>) c && d && e");
        assert!(p.is_err());
    }

    #[test]
    fn implications_err3() {
        let p = parse("a ==> ");
        assert!(p.is_err());
    }

    #[test]
    fn implications_err4() {
        let p = parse("==> b");
        assert!(p.is_err());
    }
}
