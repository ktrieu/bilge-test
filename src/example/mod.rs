use bilge::prelude::*;

pub mod first;
pub mod second;

#[bitsize(2)]
#[derive(FromBits)]
pub enum Parent {
    First,
    Second,
    Third,
    Fourth,
}
