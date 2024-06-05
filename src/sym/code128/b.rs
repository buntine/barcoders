#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code128B<'a> {
    data: &'a [u8],
}