pub type AlignContent = JustifyContent;

/// The justify content of a stack container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    /// Items are packed toward the start of the stack.
    Start,
    /// Items are packed toward the end of the stack.
    End,
    /// Items are packed toward the center of the stack.
    Center,
    /// Items are evenly distributed in the stack, with equal-size spaces between them.
    SpaceBetween,
    /// Items are evenly distributed in the stack, with half-size spaces on either end.
    SpaceAround,
    /// Items are evenly distributed in the stack.
    SpaceEvenly,
}

impl JustifyContent {
    /// Layout the items in a stack container.
    pub fn layout(
        self,
        sizes: impl ExactSizeIterator<Item = f32> + Clone,
        mut set_position: impl FnMut(usize, f32),
        size: f32,
        gap: f32,
    ) {
        if sizes.len() == 0 {
            return;
        }

        let total_gap = gap * (sizes.len() - 1) as f32;
        let total_size = sizes.clone().sum::<f32>();

        match self {
            JustifyContent::Start => {
                let mut position = 0.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::Center => {
                let mut position = (size - total_size - total_gap) / 2.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::End => {
                let mut position = size - total_size - total_gap;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceBetween => {
                let gap = (size - total_size) / (sizes.len() - 1) as f32;
                let mut position = 0.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceAround => {
                let gap = (size - total_size) / sizes.len() as f32;
                let mut position = gap / 2.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceEvenly => {
                let gap = size / (sizes.len() + 1) as f32;
                let mut position = gap / 2.0;

                for (i, _) in sizes.enumerate() {
                    set_position(i, position);
                    position += gap;
                }
            }
        }
    }
}
