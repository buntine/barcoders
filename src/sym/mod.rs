pub mod upca;

pub trait ToASCII {
    fn to_ascii(&self) -> String;
}
