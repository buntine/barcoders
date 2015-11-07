pub mod sym;
pub mod generators;

#[cfg(test)]
mod tests {
    use ::sym::ean13::*;
    use ::sym::ean8::*;
    use ::generators::ascii::*;

    #[test]
    fn ean_13_usage() {
        let ean13 = EAN13::new("123456123456".to_string());

        assert!(ean13.is_ok());

        let ean13 = ean13.unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&ean13), "SWAG".to_string());
    }

    #[test]
    fn ean_8_usage() {
        let ean8 = EAN8::new("1234567".to_string());

        assert!(ean8.is_ok());

        let ean8 = ean8.unwrap();
        let ascii = ASCII::new();

        assert_eq!(ascii.generate(&ean8), "SWAG".to_string());
    }
}
