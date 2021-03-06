#![allow(unused)]

pub mod api;
#[cfg(feature = "lcu")]
pub mod lcu;

mod champions;
mod queue;
mod rank;
mod region;
#[cfg(feature = "socket")]
pub mod socket;

pub mod consts {
    pub use super::champions::Champion;
    pub use super::queue::*;
    pub use super::rank::*;
    pub use super::region::Region;
}
