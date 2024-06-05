#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code128C<'a> {
    data: &'a [u8],
}