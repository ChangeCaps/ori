use ori_core::{Scene, SceneRender};

use crate::Render;

impl SceneRender for Render {
    fn render_scene(&mut self, scene: &mut Scene, width: u32, height: u32) {
        self.render_scene(scene, width, height);
    }
}
