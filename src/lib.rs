pub mod sym;

#[cfg(test)]
mod tests {
    use ::sym::upca::*;
    use ::sym::code39::*;
    use ::sym::ToASCII;

    #[test]
    fn new_upca() {
        let upca = UPCA::parse("123412".to_string());

        assert!(upca.is_ok());
    }

    #[test]
    fn upca_to_ascii() {
        let upca = UPCA::parse("123412".to_string()).unwrap();

        assert_eq!(upca.to_ascii(), "SWAG".to_string());
    }

    #[test]
    fn new_code39() {
        let code39 = UPCA::parse("123412".to_string());

        assert!(code39.is_ok());
    }

    #[test]
    fn code39_to_ascii() {
        let code39 = Code39::parse("123412".to_string()).unwrap();

        assert_eq!(code39.to_ascii(), "SWOLE".to_string());
    }

}
