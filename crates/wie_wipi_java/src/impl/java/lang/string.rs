use alloc::vec;

use crate::{
    base::{JavaClassProto, JavaFieldProto, JavaMethodAccessFlag, JavaMethodProto},
    JavaContext, JavaFieldAccessFlag, JavaObjectProxy, JavaResult,
};

// class java.lang.String
pub struct String {}

impl String {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, JavaMethodAccessFlag::NONE),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, JavaMethodAccessFlag::NONE),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, JavaMethodAccessFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("length", "I", JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(context: &mut dyn JavaContext, instance: JavaObjectProxy, length: u32) -> JavaResult<()> {
        log::trace!("java.lang.String::<init>({:#x}, {})", instance.ptr_instance, length);

        let array = context.instantiate_array("I", length)?;
        context.put_field(&instance, "value", array.ptr_instance)?;
        context.put_field(&instance, "length", length)?;

        Ok(())
    }

    async fn init_with_partial_char_array(
        context: &mut dyn JavaContext,
        instance: JavaObjectProxy,
        value: JavaObjectProxy,
        offset: u32,
        count: u32,
    ) -> JavaResult<()> {
        log::trace!(
            "java.lang.String::<init>({:#x}, {}, {}, {})",
            instance.ptr_instance,
            value.ptr_instance,
            offset,
            count
        );

        let array = context.instantiate_array("I", count)?;
        context.put_field(&instance, "value", array.ptr_instance)?;
        context.put_field(&instance, "length", count)?;

        let data = context.load_array(&value, offset, count)?;
        context.store_array(&array, 0, &data)?;

        Ok(())
    }

    async fn get_bytes(context: &mut dyn JavaContext, instance: JavaObjectProxy) -> JavaResult<JavaObjectProxy> {
        log::trace!("java.lang.String::getBytes({:#x})", instance.ptr_instance);

        let array = JavaObjectProxy::new(context.get_field(&instance, "value")?);

        Ok(array)
    }
}
