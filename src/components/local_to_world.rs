use crate::math::Matrix4;
use shrinkwraprs::Shrinkwrap;
use std::fmt;

#[derive(Shrinkwrap, Debug, PartialEq, Clone, Copy)]
#[shrinkwrap(mutable)]
pub struct LocalToWorld(pub Matrix4<f32>);

impl LocalToWorld {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }
}

impl fmt::Display for LocalToWorld {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
