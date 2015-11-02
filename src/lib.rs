pub mod sym;

#[cfg(test)]
mod tests {
    use ::sym::upca::UPCA;
    use ::sym::ToASCII;

    #[test]
    fn new_upca() {
        let upca = UPCA::new("123412".to_string());

        assert_eq!(upca.data, "123412".to_string());
    }

    #[test]
    fn upca_to_ascii() {
        let upca = UPCA::new("123412".to_string());

        assert_eq!(upca.to_ascii(), "SWAG".to_string());
    }

}
