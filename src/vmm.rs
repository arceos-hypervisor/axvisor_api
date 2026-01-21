//! Virtual machine management APIs.

/// Virtual machine ID.
pub type VMId = usize;
/// Virtual CPU ID.
pub type VCpuId = usize;
/// Interrupt vector.
pub type InterruptVector = u8;

/// The maximum number of virtual CPUs supported in a virtual machine.
pub const MAX_VCPU_NUM: usize = 64;

pub type VCpuSet = cpumask::CpuMask<MAX_VCPU_NUM>;

/// The API trait for virtual machine management functionalities.
#[crate::api_def]
pub trait VmmIf {
    /// Get the ID of the virtual machine executing on the current physical CPU.
    /// It MAY differ from the ID of the virtual machine calling this function.
    fn current_vm_id() -> VMId;
    /// Get the ID of the virtual CPU executing on the current physical CPU.
    /// It MAY differ from the ID of the virtual CPU calling this function.
    fn current_vcpu_id() -> VCpuId;
    /// Get the number of virtual CPUs in a virtual machine.
    fn vcpu_num(vm_id: VMId) -> Option<usize>;
    /// Get the mask of active virtual CPUs in a virtual machine.
    fn active_vcpus(vm_id: VMId) -> Option<usize>;
    /// Inject an interrupt to a virtual CPU.
    fn inject_interrupt(vm_id: VMId, vcpu_id: VCpuId, vector: InterruptVector);
    /// Inject an interrupt to a set of virtual CPUs.
    fn inject_interrupt_to_cpus(vm_id: VMId, vcpu_set: VCpuSet, vector: InterruptVector);
    /// Notify that a virtual CPU timer has expired.
    ///
    /// TODO: determine whether we can skip this function.
    fn notify_vcpu_timer_expired(vm_id: VMId, vcpu_id: VCpuId);
}

/// Get the number of virtual CPUs in the virtual machine executing on the
/// current physical CPU.
pub fn current_vm_vcpu_num() -> usize {
    vcpu_num(current_vm_id()).unwrap()
}

/// Get the mask of active virtual CPUs in the virtual machine executing on the
/// current physical CPU.
pub fn current_vm_active_vcpus() -> usize {
    active_vcpus(current_vm_id()).unwrap()
}
