// Import the appropriate libraries based on platform
#[cfg(not(target_arch = "wasm32"))]
use crate::utils::time_utils::{current_time_millis, elapsed_millis, now, sleep};
#[cfg(target_arch = "wasm32")]
use crate::utils::time_utils::{current_time_millis, elapsed_millis, now, sleep};
// Function to get the current time since UNIX_EPOCH in milliseconds
pub fn current_time_millis() -> u64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_millis() as u64
    }

    #[cfg(target_arch = "wasm32")]
    {
        // For WASM, use web_time
        web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap_or(web_time::Duration::from_secs(0))
            .as_millis() as u64
    }
}

// Create an instant for timing purposes
pub fn now() -> Instant {
    Instant::now()
}

// Get elapsed milliseconds from an instant
pub fn elapsed_millis(instant: &Instant) -> u64 {
    instant.elapsed().as_millis() as u64
}

// Sleep function (note: actual sleep isn't recommended in WASM)
#[cfg(not(target_arch = "wasm32"))]
pub fn sleep(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

#[cfg(target_arch = "wasm32")]
pub fn sleep(_millis: u64) {
    // No-op on WASM - sleeping in WASM is generally not recommended
    // For timing in WASM, you should use callbacks or requestAnimationFrame
}
