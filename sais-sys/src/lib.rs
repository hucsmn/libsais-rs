//! Low-level rust bindings to libsais.

#[cfg(all(feature = "openmp", not(windows)))]
extern crate openmp_sys;

pub mod errors;

mod common;

#[cfg(feature = "sais16")]
pub mod sais16;

#[cfg(feature = "sais32")]
pub mod sais32;

#[cfg(feature = "sais64")]
pub mod sais64;

#[cfg(test)]
mod tests;
