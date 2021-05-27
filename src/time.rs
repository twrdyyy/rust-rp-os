#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/time.rs"]
mod arch_time;

pub use arch_time::time_manager;

/// Interface used for time operations on kernel
pub mod interface {
    use core::time::Duration;

    /// TimeManager functions
    pub trait TimeManager {

        fn resolution(&self) -> Duration;

        fn uptime(&self) -> Duration;

        fn spin_for(&self, duration: Duration);
    }
}