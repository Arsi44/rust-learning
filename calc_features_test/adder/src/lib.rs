#[cfg(not(feature = "rnd"))]
pub fn add(value: usize) -> usize {
    value + 1
}

#[cfg(feature = "rnd")]
pub fn add(value: usize) -> usize {
    let rand_val: usize = rand::random();

    value.saturating_add(rand_val)
}
