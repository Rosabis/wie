use alloc::{vec, vec::Vec};

use crate::{
    base::{JavaClassProto, JavaMethodProto},
    r#impl::java::{io::InputStream, lang::String},
    string::from_java_string,
    JavaContext, JavaMethodFlag, JavaObjectProxy, JavaResult,
};

// class java.lang.Class
pub struct Class {}

impl Class {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    JavaMethodFlag::NONE,
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut dyn JavaContext, this: JavaObjectProxy<Class>) -> JavaResult<()> {
        log::warn!("stub java.lang.Class::<init>({:#x})", this.ptr_instance);

        Ok(())
    }

    async fn get_resource_as_stream(
        context: &mut dyn JavaContext,
        this: JavaObjectProxy<Class>,
        name: JavaObjectProxy<String>,
    ) -> JavaResult<JavaObjectProxy<InputStream>> {
        log::warn!(
            "stub java.lang.Class::getResourceAsStream({:#x}, {:#x})",
            this.ptr_instance,
            name.ptr_instance
        );

        let name = from_java_string(context, &name)?;
        log::debug!("getResourceAsStream name: {}", name);
        let normalized_name = if let Some(x) = name.strip_prefix('/') { x } else { &name };

        let resource = context.backend().resource().id(normalized_name);
        if let Some(x) = resource {
            let data_u32 = context.backend().resource().data(x).iter().map(|x| *x as u32).collect::<Vec<_>>();

            let array = context.instantiate_array("B", data_u32.len() as u32)?;
            context.store_array(&array, 0, &data_u32)?;

            let result = context.instantiate("Ljava/io/ByteArrayInputStream;")?.cast();
            context.call_method(&result.cast(), "<init>", "([B)V", &[array.ptr_instance]).await?;

            Ok(result)
        } else {
            Ok(JavaObjectProxy::new(0))
        }
    }
}
