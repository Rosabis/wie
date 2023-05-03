use std::any::Any;

use crate::wipi::java::{JavaClassImpl, JavaMethodImpl};

// class org.kwis.msp.lcdui.Jlet
pub struct Jlet {}

impl Jlet {
    pub fn as_java_impl() -> JavaClassImpl {
        JavaClassImpl {
            name: "org/kwis/msp/lcdui/Jlet".to_owned(),
            methods: vec![JavaMethodImpl {
                name: "H()V+<init>".to_owned(),
                body: Box::new(Self::init),
            }],
        }
    }

    fn init(_: Vec<Box<dyn Any>>) -> Box<dyn Any> {
        log::debug!("Jlet::init");

        Box::new(())
    }
}
