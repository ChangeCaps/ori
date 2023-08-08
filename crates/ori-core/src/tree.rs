use std::any::{Any, TypeId};

use glam::Vec2;

#[derive(Default)]
pub struct Tree {
    pub(crate) state: Option<Box<dyn Any>>,
    pub(crate) size: Option<Vec2>,
    pub(crate) children: Vec<Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state_type_id(&self) -> Option<TypeId> {
        Some(self.state()?.type_id())
    }

    pub fn state(&self) -> Option<&dyn Any> {
        self.state.as_ref().map(|state| state.as_ref())
    }

    pub fn state_mut(&mut self) -> Option<&mut dyn Any> {
        self.state.as_mut().map(|state| state.as_mut())
    }

    pub fn set_state(&mut self, state: impl Any) {
        self.state = Some(Box::new(state));
    }

    pub fn size(&self) -> Option<Vec2> {
        self.size
    }

    pub fn child(&mut self, index: usize) -> &mut Tree {
        if self.children.len() <= index {
            self.children.resize_with(index + 1, Tree::new);
        }

        &mut self.children[index]
    }
}
