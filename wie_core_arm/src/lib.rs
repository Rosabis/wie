#![no_std]
extern crate alloc;

mod allocator;
mod context;
mod core;
mod function;
mod future;

pub use self::{
    allocator::Allocator,
    core::{ArmCore, ArmCoreError, PEB_BASE},
    function::{EmulatedFunction, EmulatedFunctionParam},
};