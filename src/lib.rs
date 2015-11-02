pub mod sym;

#[cfg(test)]
mod tests {
    use ::sym::upca::UPCA;

    #[test]
    fn new_upca() {
        let upca = UPCA::new("123412".to_string());

        assert_eq!(upca.data, "123412".to_string());
    }
}
