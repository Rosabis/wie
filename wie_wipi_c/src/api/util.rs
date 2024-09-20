use wie_util::Result;

use crate::{context::WIPICContext, WIPICWord};

pub async fn htons(_context: &mut dyn WIPICContext, val: WIPICWord) -> Result<WIPICWord> {
    tracing::debug!("MC_utilHtons({})", val);

    Ok((val as u16).to_be() as _) // XXX we're always on little endian
}
