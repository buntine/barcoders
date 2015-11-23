/// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
pub fn join_slices(slices: &[&[u8]]) -> Vec<u8> {
    slices.iter()
          .flat_map(|b| b.into_iter())
          .cloned()
          .collect()
}

/// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
/// TODO: How to make this and join_slices generic??
pub fn join_vecs(vecs: &[Vec<u8>]) -> Vec<u8> {
    vecs.iter()
        .flat_map(|b| b.into_iter())
        .cloned()
        .collect()
}

/// Joins and flattens the given slice of &[u8] slices into a Vec<u8>.
/// TODO: How to make this and join_slices generic??
/// TODO: SERIOUSLY, HOW DO I MAKE THESE ONE FUNCTION??
pub fn join_arrays(arrs: &[[u8; 7]]) -> Vec<u8> {
    arrs.iter()
        .flat_map(|b| b.into_iter())
        .cloned()
        .collect()
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
