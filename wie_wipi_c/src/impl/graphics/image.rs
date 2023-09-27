use bytemuck::{Pod, Zeroable};

use wie_backend::canvas::decode_image;

use crate::base::{CContext, CMemoryId};

use super::WIPICFramebuffer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct WIPICImage {
    pub img: WIPICFramebuffer,
    pub mask: WIPICFramebuffer,
    pub loop_count: u32,
    pub delay: u32,
    pub animated: u32,
    pub buf: CMemoryId,
    pub offset: u32,
    pub current: u32,
    pub len: u32,
}

impl WIPICImage {
    pub fn new(context: &mut dyn CContext, buf: CMemoryId, offset: u32, len: u32) -> anyhow::Result<Self> {
        let ptr_image_data = context.data_ptr(buf)?;
        let data = context.read_bytes(ptr_image_data + offset, len)?;
        let image = decode_image(&data)?;

        let img_framebuffer = WIPICFramebuffer::from_image(context, &*image)?;
        let mask_framebuffer = WIPICFramebuffer::empty();

        Ok(Self {
            img: img_framebuffer,
            mask: mask_framebuffer,
            loop_count: 0,
            delay: 0,
            animated: 0,
            buf,
            offset,
            current: 0,
            len,
        })
    }
}