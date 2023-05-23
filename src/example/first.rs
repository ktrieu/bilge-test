use bilge::prelude::*;

use super::second::Sibling;

#[bitsize(8)]
#[derive(FromBits)]
pub struct Example {
    sibling: Sibling,
    parent: u2,
}
