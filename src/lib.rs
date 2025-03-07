//! `axvisor_api` is a library that provides:
//! - a set of Rust API for all components of the `axvisor` Hypervisor, including:
//!
//!   - memory management,
//!   - time and timer management,
//!   - interrupt management,
//!   - ...
//!
//!   these APIs are defined here, should be implemented by the axvisor Hypervisor, and can be use by all components.
//!
//! - a standard way to define and implement APIs, including the [`api_mod!`] and the [`api_mod_impl`] attributes, which
//!   the components can utilize to define and implement their own APIs.
//!
//! # How to define and implement APIs
//!
//! ## Define APIs
//!
//! To define APIs, you can use the `api_mod!` attribute to define a module containing the API functions. An API
//! function is defined with the `extern fn` syntax. Note that Vanilla Rust does not support defining extern functions
//! in such a way, so the definition of the API functions can easily be distinguished from the regular functions.
//!
//! ```rust, standalone_crate
//! # use axvisor_api::{api_mod, __priv}; // some inconviniece brought by proc-macro-name and doctest
//! # fn main() {}
//! #[api_mod]
//! /// Memory-related API
//! pub mod memory_demo {
//!     pub use memory_addr::PhysAddr;
//!
//!     /// Allocate a frame
//!     extern fn alloc_frame() -> Option<PhysAddr>;
//!     /// Deallocate a frame
//!     extern fn dealloc_frame(addr: PhysAddr);
//! }
//! ```
//!
//! Defined APIs can be invoked by other components:
//!
//! ```rust, no_run, standalone_crate
//! # use axvisor_api::{api_mod, __priv}; // some inconviniece brought by proc-macro-name and doctest
//! # fn main() {}
//! # #[api_mod]
//! # /// Memory-related API
//! # pub mod memory_demo {
//! #     pub use memory_addr::PhysAddr;
//! #
//! #     /// Allocate a frame
//! #     extern fn alloc_frame() -> Option<PhysAddr>;
//! #     /// Deallocate a frame
//! #     extern fn dealloc_frame(addr: PhysAddr);
//! # }
//! struct SomeComponent;
//!
//! impl SomeComponent {
//!     fn some_method() {
//!         let frame = memory_demo::alloc_frame().unwrap();
//!         // Do something with the frame
//!         memory_demo::dealloc_frame(frame);
//!     }
//! }
//! ```
//!
//! ## Implement APIs
//!
//! Defined APIs should be implemented by the Hypervisor, or other components that are able and responsible to do so. To
//! implement APIs, you can use the `api_mod_impl` attribute, with the path of the module defining the APIs as the
//! argument, on a module containing the implementation of the API functions. The implementations of the API functions
//! are also defined with the `extern fn` syntax.
//!
//! ```rust, no_run, standalone_crate
//! # use axvisor_api::{api_mod, api_mod_impl, __priv}; // some inconviniece brought by proc-macro-name and doctest
//! # fn main() {}
//! # #[api_mod]
//! # /// Memory-related API
//! # pub mod memory_demo {
//! #     pub use memory_addr::PhysAddr;
//! #
//! #     /// Allocate a frame
//! #     extern fn alloc_frame() -> Option<PhysAddr>;
//! #     /// Deallocate a frame
//! #     extern fn dealloc_frame(addr: PhysAddr);
//! # }
//! #[api_mod_impl(memory_demo)]
//! mod memory_impl {
//!     use memory_addr::PhysAddr;
//!
//!     extern fn alloc_frame() -> Option<PhysAddr> {
//!         // Implementation of the `alloc_frame` API
//!         todo!()
//!     }
//!
//!     extern fn dealloc_frame(addr: PhysAddr) {
//!         // Implementation of the `dealloc_frame` API
//!         todo!()
//!     }
//! }
//! ```
//!
//! ## Tricks behind the macros
//!
//! [`api_mod`] and [`api_mod_impl`] are wrappers around the great [`crate_interface`] crate, with some macro tricks to
//! make the usage more convenient.
//!

#![no_std]

pub use axvisor_api_proc::{api_mod, api_mod_impl};

#[api_mod]
/// Memory-related API.
pub mod memory {
    pub use memory_addr::{PhysAddr, VirtAddr};

    /// Allocate a frame.
    extern fn alloc_frame() -> Option<PhysAddr>;
    /// Deallocate a frame.
    extern fn dealloc_frame(addr: PhysAddr);
    /// Convert a physical address to a virtual address.
    extern fn phys_to_virt(addr: PhysAddr) -> VirtAddr;
    /// Convert a virtual address to a physical address.
    extern fn virt_to_phys(addr: VirtAddr) -> PhysAddr;
}

#[api_mod]
/// Time-and-timer-related API.
pub mod time {
    extern crate alloc;
    use alloc::boxed::Box;
    use core::time::Duration;

    /// Time value.
    pub type TimeValue = Duration;
    /// Nanoseconds count.
    pub type Nanos = u64;
    /// Tick count.
    pub type Ticks = u64;
    /// Cancel token， used to cancel a scheduled timer event.
    pub type CancelToken = usize;

    /// Get the current tick count.
    extern fn current_ticks() -> Ticks;
    /// Get the current time in nanoseconds.
    pub fn current_time_nanos() -> Nanos {
        ticks_to_nanos(current_ticks())
    }
    /// Get the current time.
    pub fn current_time() -> TimeValue {
        Duration::from_nanos(current_time_nanos())
    }

    /// Convert ticks to nanoseconds.
    extern fn ticks_to_nanos(ticks: Ticks) -> Nanos;
    /// Convert ticks to time.
    pub fn ticks_to_time(ticks: Ticks) -> TimeValue {
        Duration::from_nanos(ticks_to_nanos(ticks))
    }
    /// Convert nanoseconds to ticks.
    extern fn nanos_to_ticks(nanos: Nanos) -> Ticks;
    /// Convert time to ticks.
    pub fn time_to_ticks(time: TimeValue) -> Ticks {
        nanos_to_ticks(time.as_nanos() as Nanos)
    }

    /// Register a timer
    extern fn register_timer(
        deadline: TimeValue,
        callback: Box<dyn FnOnce(TimeValue) + Send + 'static>,
    ) -> CancelToken;
    /// Cancel a timer
    extern fn cancel_timer(token: CancelToken);
}

#[api_mod]
pub mod vmm {
    /// Virtual machine ID
    pub type VMId = usize;
    /// Virtual CPU ID
    pub type VCpuId = usize;
    /// Interrupt vector
    pub type InterruptVector = u8;

    /// Get the ID of the current virtual machine
    extern fn current_vm_id() -> VMId;
    /// Get the ID of the current virtual CPU
    extern fn current_vcpu_id() -> VCpuId;

    extern fn vcpu_num(vm_id: VMId) -> usize;

    extern fn active_vcpus(vm_id: VMId) -> usize;

    /// Inject an interrupt to a virtual CPU
    extern fn inject_interrupt(vm_id: VMId, vcpu_id: VCpuId, vector: InterruptVector);
    /// TODO: determine whether we can skip this function
    extern fn notify_vcpu_timer_expired(vm_id: VMId, vcpu_id: VCpuId);
}

#[doc(hidden)]
pub mod __priv {
    pub mod crate_interface {
        pub use crate_interface::{call_interface, def_interface, impl_interface};
    }
}

#[cfg(test)]
mod test;
