pub mod upca;
pub mod code39;

pub trait ToASCII {
    fn to_ascii(&self) -> String;
}
