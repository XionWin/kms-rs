#[macro_use]
extern crate colored_rs;

mod vertical_synchronize;
mod utility;
mod oflag;
mod kms;
mod graphic;

pub use kms::*;
pub use graphic::*;

pub use egl_rs::def::SurfaceType;
