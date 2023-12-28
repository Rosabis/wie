use alloc::vec;

use crate::{
    base::{JavaClassProto, JavaContext, JavaMethodFlag, JavaMethodProto, JavaResult},
    proxy::JvmClassInstanceProxy,
};

// class java.lang.Object
pub struct Object {}

impl Object {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("getClass", "()Ljava/lang/Class;", Self::get_class, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut dyn JavaContext, this: JvmClassInstanceProxy<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.Object::<init>({:?})", &this);

        Ok(())
    }

    async fn get_class(context: &mut dyn JavaContext, this: JvmClassInstanceProxy<Self>) -> JavaResult<JvmClassInstanceProxy<Self>> {
        tracing::warn!("stub java.lang.Object::get_class({:?})", &this);

        let result = context.jvm().instantiate_class("java/lang/Class").await?;

        context.jvm().invoke_method(&result, "java/lang/Class", "<init>", "()V", &[]).await?;

        Ok(JvmClassInstanceProxy::new(result))
    }
}
