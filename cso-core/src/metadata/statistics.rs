use crate::any::AsAny;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Stats: Debug + AsAny {
    fn should_update(&self, new_stats: &Rc<dyn Stats>) -> bool;
}
