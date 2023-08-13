use std::any::{Any, TypeId};

use ori_graphics::math::Vec2;

use crate::Cursor;

#[derive(Default)]
pub struct Tree {
    pub(crate) view_state: Option<Box<dyn Any>>,
    pub(crate) layout_size: Option<Vec2>,
    pub(crate) cursor: Option<Cursor>,
    pub(crate) children: Vec<Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn view_state_type_id(&self) -> Option<TypeId> {
        Some(<dyn Any>::type_id(self.view_state.as_ref()?))
    }

    pub fn set_view_state(&mut self, state: Box<dyn Any>) {
        self.view_state = Some(state);
    }

    pub fn take_view_state(&mut self) -> Option<Box<dyn Any>> {
        self.view_state.take()
    }

    pub fn view_state_mut(&mut self) -> Option<&mut dyn Any> {
        self.view_state.as_mut().map(|state| state.as_mut())
    }

    pub fn size(&self) -> Option<Vec2> {
        self.layout_size
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.layout_size = Some(size);
    }

    pub fn set_cursor(&mut self, cursor: Option<Cursor>) {
        self.cursor = cursor;
    }

    fn get_cursor_recurse(&self) -> Option<Cursor> {
        for child in self.children.iter() {
            let cursor = child.get_cursor_recurse();

            if cursor.is_some() {
                return cursor;
            }
        }

        self.cursor
    }

    pub fn get_cursor(&self) -> Cursor {
        self.get_cursor_recurse().unwrap_or_default()
    }

    pub fn child(&mut self, index: usize) -> &mut Tree {
        if self.children.len() <= index {
            self.children.resize_with(index + 1, Tree::new);
        }

        &mut self.children[index]
    }
}
