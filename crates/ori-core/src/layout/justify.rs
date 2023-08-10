pub type AlignContent = JustifyContent;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
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
        let total_size = sizes.clone().sum::<f32>() + total_gap;

        match self {
            JustifyContent::Start => {
                let mut position = 0.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::Center => {
                let mut position = (size - total_size) / 2.0;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
            JustifyContent::End => {
                let mut position = size - total_size;

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
                let gap = (size - total_size) / (sizes.len() + 1) as f32;
                let mut position = gap;

                for (i, size) in sizes.enumerate() {
                    set_position(i, position);
                    position += size + gap;
                }
            }
        }
    }
}
