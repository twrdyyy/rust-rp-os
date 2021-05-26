#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/exception.rs"]
mod arch_exception;

pub mod asynchronous;
pub use arch_exception::current_privilege_level;

/// Privileges Enum
///
/// # Safety
///
/// - User L2
/// - Kernel L1
/// - Hypervisor L0
#[allow(missing_docs)]
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Unknown,
}