use bilge::prelude::*;

use super::{second::Sibling, Parent};

#[bitsize(8)]
#[derive(FromBits)]
pub struct Example {
    sibling: Sibling,
    parent: Parent,
}
