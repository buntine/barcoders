/// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
/// TODO: Work out how to use join_iters with slices and then remove this function.
pub fn join_slices(slices: &[&[u8]]) -> Vec<u8> {
    slices.iter().flat_map(|b| b.iter()).cloned().collect()
}

/// Joins and flattens the given iterator of iterables into a Vec<u8>.
pub fn join_iters<'a, T: Iterator>(iters: T) -> Vec<u8>
where
    T::Item: IntoIterator<Item = &'a u8>,
{
    iters.flat_map(|b| b.into_iter()).cloned().collect()
}

/// Calculates the checksum digit using a modulo-10 weighting algorithm.
pub fn modulo_10_checksum(data: &[u8], even_start: bool) -> u8 {
    let mut odds = 0;
    let mut evens = 0;

    for (i, d) in data.iter().enumerate() {
        match i % 2 {
            1 => odds += *d,
            _ => evens += *d,
        }
    }

    // EAN-13 (and some others?) barcodes use EVEN-first weighting to maintain
    // backwards compatibility.
    if even_start {
        odds *= 3;
    } else {
        evens *= 3;
    }

    match 10 - ((odds + evens) % 10) {
        10 => 0,
        n => n,
    }
}
