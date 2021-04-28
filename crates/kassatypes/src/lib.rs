#![allow(unused)]

pub mod api;
mod champions;
mod queue;
mod rank;
mod region;

pub mod consts {
    pub use super::region::Region;
    pub use super::queue::*;
    pub use super::rank::*;
    pub use super::champions::Champion;
}