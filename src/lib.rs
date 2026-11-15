#![no_std]

mod structs;
    pub use structs::*;

#[cfg(not(feature="legacy_build"))]
pub mod generic;
#[cfg(feature="legacy_build")]
#[path="legacy/generic.rs"]
pub mod generic;
    pub use generic::*;

#[cfg(not(feature="legacy_build"))]
pub mod constant;
#[cfg(feature="legacy_build")]
#[path="legacy/generic.rs"]
pub mod constant;
    pub use constant::*;

#[cfg(not(feature="legacy_build"))]
pub mod trigonometry;
#[cfg(feature="legacy_build")]
#[path="legacy/generic.rs"]
pub mod trigonometry;
    pub use trigonometry::*;

#[cfg(not(feature="legacy_build"))]
pub mod vector;
#[cfg(feature="legacy_build")]
#[path="legacy/generic.rs"]
pub mod vector;
    pub use vector::*;

#[cfg(not(feature="legacy_build"))]
pub mod matrix;
#[cfg(feature="legacy_build")]
#[path="legacy/generic.rs"]
pub mod matrix;
    pub use matrix::*;

mod utils;

mod traits;

mod parse;