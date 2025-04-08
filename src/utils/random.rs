use getrandom::getrandom;
use std::ops::RangeInclusive;

/// Generate a random number within the given range, compatible with all platforms including WASM
pub fn gen_range<R>(range: R) -> u64
where
    R: Into<RangeInclusive<u64>>,
{
    let range = range.into();
    let min = *range.start();
    let max = *range.end();

    if min == max {
        return min;
    }

    let range_size = max - min + 1;

    // Create a buffer to store random bytes
    let mut buffer = [0u8; 8];

    // Fill buffer with random bytes in a platform-independent way
    getrandom(&mut buffer).expect("Failed to generate random bytes");

    // Convert bytes to u64 and get value within range
    min + (u64::from_ne_bytes(buffer) % range_size)
}

/// Shuffle a vector in place using the Fisher-Yates algorithm
pub fn shuffle<T>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }

    for i in (1..vec.len()).rev() {
        // Get a random index between 0 and i (inclusive)
        let j = gen_range(0..=i as u64) as usize;
        vec.swap(i, j);
    }
}
