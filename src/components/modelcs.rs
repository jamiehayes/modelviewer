/*
// external refs
use specs::{System, Component, VecStorage, ReadStorage, WriteStorage};

// local refs
use crate::gfx::Model;
use crate::components::transformcs::TransformCI;

///
/// Model component instance data
///
pub struct ModelCI {
    model: Model
}

impl Component for ModelCI {
    type Storage = VecStorage<Self>;
}

///
/// Model component update system
///
pub struct ModelCSUpdate;

impl<'a> System<'a> for ModelCSUpdate {
    type SystemData = ( WriteStorage<'a, TransformCI>, ReadStorage<'a, ModelCI> );

    fn run(&mut self, (transormci, mut modelci): Self::SystemData) {
    }
}

///
/// Model component render system
///
pub struct ModelCSRender;

impl<'a> System<'a> for ModelCSRender {
    type SystemData = ReadStorage<'a, ModelCI>;

    fn run(&mut self, model: Self::SystemData) {
    }
}
*/