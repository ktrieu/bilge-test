use bilge::prelude::*;

#[bitsize(6)]
#[derive(FromBits)]
pub struct Sibling {
    field: u2,
    other: u4,
}
