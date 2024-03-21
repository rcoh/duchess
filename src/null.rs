use crate::{AsJRef, JavaObject, NullJRef};

#[derive(Copy, Clone, Debug)]
pub struct Null;

impl<J: JavaObject> AsJRef<J> for Null {
    fn as_jref(&self) -> crate::Nullable<&J> {
        Err(NullJRef)
    }
}
