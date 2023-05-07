use super::{into_body, CContext, CMethodBody, CResult};

fn dummy(_: &mut CContext) -> CResult<u32> {
    log::debug!("graphics dummy called");

    Ok(0)
}

fn get_screen_frame_buffer(_: &mut CContext, a0: u32) -> CResult<u32> {
    log::debug!("get_screen_frame_buffer({:#x})", a0);

    Ok(1234)
}

pub fn get_graphics_method_table() -> Vec<CMethodBody> {
    vec![into_body(dummy), into_body(dummy), into_body(get_screen_frame_buffer)]
}
