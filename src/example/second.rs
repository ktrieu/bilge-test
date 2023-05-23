use bilge::prelude::*;

use super::Parent;

#[bitsize(6)]
#[derive(FromBits)]
pub struct Sibling {
    parent: Parent,
    other: u4,
}
