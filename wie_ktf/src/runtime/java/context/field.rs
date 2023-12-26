use core::mem::size_of;

use bytemuck::{Pod, Zeroable};

use wie_base::util::{read_generic, write_generic, ByteWrite};
use wie_core_arm::{Allocator, ArmCore};
use wie_impl_java::{JavaFieldAccessFlag, JavaFieldProto, JavaResult};

use super::JavaFullName;

bitflags::bitflags! {
    struct JavaFieldAccessFlagBit: u32 {
        const NONE = 0;
        const STATIC = 8;
    }
}

impl JavaFieldAccessFlagBit {
    fn from_access_flag(access_flag: JavaFieldAccessFlag) -> JavaFieldAccessFlagBit {
        match access_flag {
            JavaFieldAccessFlag::NONE => JavaFieldAccessFlagBit::NONE,
            JavaFieldAccessFlag::STATIC => JavaFieldAccessFlagBit::STATIC,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RawJavaField {
    access_flag: u32,
    ptr_class: u32,
    ptr_name: u32,
    offset_or_value: u32,
}

pub struct JavaField {
    pub(crate) ptr_raw: u32,
    core: ArmCore,
}

impl JavaField {
    pub fn from_raw(ptr_raw: u32, core: &ArmCore) -> Self {
        Self { ptr_raw, core: core.clone() }
    }

    pub fn new(core: &mut ArmCore, ptr_class: u32, proto: JavaFieldProto, offset_or_value: u32) -> JavaResult<Self> {
        let full_name = (JavaFullName {
            tag: 0,
            name: proto.name,
            descriptor: proto.descriptor,
        })
        .as_bytes();

        let ptr_name = Allocator::alloc(core, full_name.len() as u32)?;
        core.write_bytes(ptr_name, &full_name)?;

        let ptr_raw = Allocator::alloc(core, size_of::<RawJavaField>() as u32)?;

        write_generic(
            core,
            ptr_raw,
            RawJavaField {
                access_flag: JavaFieldAccessFlagBit::from_access_flag(proto.access_flag).bits(),
                ptr_class,
                ptr_name,
                offset_or_value,
            },
        )?;

        Ok(Self::from_raw(ptr_raw, core))
    }

    pub fn name(&self) -> JavaResult<JavaFullName> {
        let raw: RawJavaField = read_generic(&self.core, self.ptr_raw)?;

        JavaFullName::from_ptr(&self.core, raw.ptr_name)
    }

    pub fn offset(&self) -> JavaResult<u32> {
        let raw: RawJavaField = read_generic(&self.core, self.ptr_raw)?;

        anyhow::ensure!(raw.access_flag & 0x0008 == 0, "Field is static");

        Ok(raw.offset_or_value)
    }

    pub fn static_address(&self) -> JavaResult<u32> {
        let raw: RawJavaField = read_generic(&self.core, self.ptr_raw)?;

        anyhow::ensure!(raw.access_flag & 0x0008 != 0, "Field is not static");

        let address = self.ptr_raw + 12; // offsetof offset_or_value

        Ok(address)
    }
}
