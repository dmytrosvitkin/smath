#![no_std]

#[cfg(not(feature="legacy_build"))]
mod new;
#[cfg(not(feature="legacy_build"))]
pub use new::*;

#[cfg(feature="legacy_build")]
mod legacy;
#[cfg(feature="legacy_build")]
pub use legacy::*;