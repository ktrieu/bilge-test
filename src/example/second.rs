use bilge::prelude::*;

#[bitsize(6)]
#[derive(FromBits)]
pub struct Sibling {
    parent: u2,
    other: u4,
}
