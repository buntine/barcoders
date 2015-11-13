pub mod sym;
pub mod generators;

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::sym::code39::*;
    use ::generators::ascii::*;

    #[test]
    fn ean_13_usage() {
        let ean13 = EAN13::new("123456123456".to_string());

        assert!(ean13.is_ok());

        let ean13 = ean13.unwrap();
        let encoded = ean13.encode();
        let ascii = ASCII::new();

        assert!(ascii.generate(&encoded).is_ok());
    }

    #[test]
    fn ean_8_usage() {
        let ean8 = EAN8::new("1234567".to_string());

        assert!(ean8.is_ok());

        let ean8 = ean8.unwrap();
        let encoded = ean8.encode();
        let ascii = ASCII::new();

        assert!(ascii.generate(&encoded).is_ok());
    }

    #[test]
    fn upca_usage() {
        let upca = UPCA::new("012345123456".to_string());

        assert!(upca.is_ok());

        let upca = upca.unwrap();
        let encoded = upca.encode();
        let ascii = ASCII::new();

        assert!(ascii.generate(&encoded).is_ok());
    }

    #[test]
    fn code39_usage() {
        let code39 = Code39::new("AB2C1674+1".to_string());

        assert!(code39.is_ok());

        let code39 = code39.unwrap();
        let encoded = code39.encode();
        let ascii = ASCII::new();

        assert!(ascii.generate(&encoded).is_ok());
    }

}
