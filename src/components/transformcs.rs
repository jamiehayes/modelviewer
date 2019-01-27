// external refs
use specs::{Component, VecStorage};

// local refs
use crate::numerics::*;

pub struct TransformCI {
    transform: TransformF
}

impl Component for TransformCI {
    type Storage = VecStorage<Self>;
}