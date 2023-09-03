use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> AsAny for T {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
}
