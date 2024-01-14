#[macro_use]
extern crate colored_rs;

mod kms;
mod vertical_synchronize;
mod utility;
mod oflag;

pub use kms::*;

pub use egl_rs::def::SurfaceType;
