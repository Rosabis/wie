use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, JvmResult};

use crate::{
    classes::org::kwis::msp::media::Clip,
    context::{WIPIJavaClassProto, WIPIJavaContext},
};

// class org.kwis.msp.media.Player
pub struct Player {}

impl Player {
    pub fn as_proto() -> WIPIJavaClassProto {
        WIPIJavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("play", "(Lorg/kwis/msp/media/Clip;Z)Z", Self::play, MethodAccessFlags::STATIC),
                JavaMethodProto::new("stop", "(Lorg/kwis/msp/media/Clip;)Z", Self::stop, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
        }
    }

    async fn play(_: &Jvm, _: &mut WIPIJavaContext, clip: ClassInstanceRef<Clip>, repeat: bool) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::play({:?}, {})", &clip, repeat);

        Ok(false)
    }

    async fn stop(_: &Jvm, _: &mut WIPIJavaContext, clip: ClassInstanceRef<Clip>) -> JvmResult<bool> {
        tracing::warn!("stub org.kwis.msp.media.Player::stop({:?})", &clip,);

        Ok(false)
    }
}
