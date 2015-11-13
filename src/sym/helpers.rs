pub fn join_vecs(vecs: &[Vec<u8>]) -> Vec<u8> {
    vecs.iter()
        .flat_map(|b| b.into_iter())
        .cloned()
        .collect()
}
