pub mod sym;
pub mod generators;

#[cfg(test)]
mod tests {
    use ::sym::upca::*;
    use ::sym::code39::*;
    use ::generators::ascii::*;
    use ::sym::Encode;

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
    fn upca_raw_data() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();

        assert_eq!(upca.raw_data(), "123456123456");
    }

    #[test]
    fn upca_encode() {
        let upca1 = UPCA::new("12345612345".to_string()).unwrap();
        let upca2 = UPCA::new("00118999561".to_string()).unwrap();

        assert_eq!(upca1.encode(), "10100110010010011011110101000110110001010111101010110011011011001000010101110010011101101100101".to_string());
        assert_eq!(upca2.encode(), "10100011010001101001100100110010110111000101101010111010011101001001110101000011001101101100101".to_string());
    }

    #[test]
    fn upca_to_ascii() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&upca), "SWAG".to_string());
    }

    #[test]
    fn upca_to_ascii_with_large_height() {
        let upca = UPCA::new("123456123456".to_string()).unwrap();
        let ascii = ASCII::new().height(40).xdim(2);

        assert_eq!(ascii.height, 40);
        assert_eq!(ascii.xdim, 2);
        assert_eq!(ascii.generate(&upca), "SWAG".to_string());
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
    fn code39_raw_data() {
        let code39 = UPCA::new("123456123456".to_string()).unwrap();

        assert_eq!(code39.raw_data(), "123456123456");
    }

//    #[test]
//    fn code39_to_ascii() {
//        let code39 = Code39::new("123412".to_string()).unwrap();
//        let ascii = ASCII::new();
//
//        assert_eq!(ascii.generate(&code39), "SWAG".to_string());
//    }

}
