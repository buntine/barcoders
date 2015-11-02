pub mod sym;

mod tests {
    use super::sym::*;

    #[test]
    fn new_upca() {
        let upca = UPCA::new("123412".to_string());

        assert_eq!(upca.data, "123412".to_string());
    }
}
