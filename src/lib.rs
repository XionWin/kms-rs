#[macro_use]
extern crate colored_rs;

mod kms;
mod vertical_synchronize;
mod utility;
mod oflag;
mod context;

pub use kms::*;
pub use context::*;

pub use egl_rs::def::SurfaceType;
