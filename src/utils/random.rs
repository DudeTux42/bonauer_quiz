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

/// Simple range overload for convenience
pub fn gen_range_simple(min: u64, max: u64) -> u64 {
    gen_range(min..=max)
}

/// Generate a random floating point number between 0.0 and 1.0
pub fn random_f64() -> f64 {
    let mut buffer = [0u8; 8];
    getrandom(&mut buffer).expect("Failed to generate random bytes");

    // Convert to u64 and normalize to [0, 1) range
    let value = u64::from_ne_bytes(buffer);
    (value as f64) / (u64::MAX as f64)
}

/// Shuffle a vector in place using the Fisher-Yates algorithm
pub fn shuffle<T>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }

    for i in (1..vec.len()).rev() {
        // Get a random index between 0 and i (inclusive)
        let j = gen_range(0..=i) as usize;
        vec.swap(i, j);
    }
}
