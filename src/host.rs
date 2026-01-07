//! Host system related APIs.

#[crate::api_def]
pub trait HostIf {
    /// Get the total number of cpus in the host system.
    fn get_host_cpu_num() -> usize;
}
