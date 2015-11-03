pub mod sym;

#[cfg(test)]
mod tests {
    use ::sym::upca::*;
    use ::sym::code39::*;
    use ::sym::ToASCII;

    #[test]
    fn new_upca() {
        let upca = UPCA::new("123456123456".to_string());

        assert!(upca.is_ok());
    }

    #[test]
    fn invalid_data_upca() {
        let upca = UPCA::new("1234er123412".to_string());

        assert!(upca.is_err());
    }

    #[test]
    fn invalid_len_upca() {
        let upca = UPCA::new("1111112222222333333".to_string());

        assert!(upca.is_err());
    }

    #[test]
    fn upca_to_ascii() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();

        assert_eq!(upca.to_ascii(), "SWAG".to_string());
    }

    #[test]
    fn new_code39() {
        let code39 = UPCA::new("123456123456".to_string());

        assert!(code39.is_ok());
    }

    #[test]
    fn invalid_data_code39() {
        let code39 = Code39::new("1212s".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn invalid_len_code39() {
        let code39 = Code39::new("1111112222222333333".to_string());

        assert!(code39.is_err());
    }

    #[test]
    fn code39_to_ascii() {
        let code39 = Code39::new("123412".to_string()).unwrap();

        assert_eq!(code39.to_ascii(), "SWOLE".to_string());
    }

}
