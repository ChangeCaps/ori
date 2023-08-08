use glam::Vec2;
use ori_reactive::Event;

use crate::{AvailableSpace, DrawContext, EventContext, IntoView, LayoutContext, Node, StateView};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    Row,
    Column,
}

impl Axis {
    pub const fn cross(self) -> Self {
        match self {
            Self::Row => Self::Column,
            Self::Column => Self::Row,
        }
    }

    pub fn major(self, vec: Vec2) -> f32 {
        match self {
            Self::Row => vec.x,
            Self::Column => vec.y,
        }
    }

    pub fn minor(self, vec: Vec2) -> f32 {
        match self {
            Self::Row => vec.y,
            Self::Column => vec.x,
        }
    }

    pub fn pack(self, major: f32, minor: f32) -> Vec2 {
        match self {
            Self::Row => Vec2::new(major, minor),
            Self::Column => Vec2::new(minor, major),
        }
    }

    pub fn unpack(self, vec: Vec2) -> (f32, f32) {
        match self {
            Self::Row => (vec.x, vec.y),
            Self::Column => (vec.y, vec.x),
        }
    }
}

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
    fn layout(self, content: &[f32], size: f32, gap: f32) -> Vec<f32> {
        if content.is_empty() {
            return Vec::new();
        }

        let mut positions = Vec::with_capacity(content.len());

        let total_gap = gap * (content.len() - 1) as f32;
        let total_size = content.iter().sum::<f32>() + total_gap;

        match self {
            JustifyContent::Start => {
                let mut position = 0.0;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
            JustifyContent::Center => {
                let mut position = (size - total_size) / 2.0;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
            JustifyContent::End => {
                let mut position = size - total_size;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceBetween => {
                let gap = (size - total_size) / (content.len() - 1) as f32;
                let mut position = 0.0;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceAround => {
                let gap = (size - total_size) / content.len() as f32;
                let mut position = gap / 2.0;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
            JustifyContent::SpaceEvenly => {
                let gap = (size - total_size) / (content.len() + 1) as f32;
                let mut position = gap;

                for &size in content {
                    positions.push(position);
                    position += size + gap;
                }
            }
        }

        positions
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Stretch,
}

impl AlignItems {
    pub const fn is_stretch(&self) -> bool {
        matches!(self, Self::Stretch)
    }
}

pub type AlignContent = JustifyContent;
pub type AlignSelf = AlignItems;

#[macro_export]
macro_rules! row {
    ($($child:expr),* $(,)?) => {
        $crate::views::Flex::row()
            $(.child($crate::views::FlexChild::new($child)))*
    };
}

#[macro_export]
macro_rules! column {
    ($($child:expr),* $(,)?) => {
        $crate::views::Flex::column()
            $(.child($crate::views::FlexChild::new($expr)))*
    };
}

#[derive(Clone, Debug)]
pub struct FlexChild {
    node: Node,
    flex: f32,
    align: Option<AlignSelf>,
}

impl<T: IntoView> From<T> for FlexChild {
    fn from(view: T) -> Self {
        Self {
            node: Node::new(view),
            flex: 0.0,
            align: None,
        }
    }
}

impl FlexChild {
    pub fn new(view: impl Into<FlexChild>) -> Self {
        view.into()
    }

    pub fn flex(mut self, flex: f32) -> Self {
        self.flex = flex;
        self
    }

    pub fn align(mut self, align: impl Into<Option<AlignSelf>>) -> Self {
        self.align = align.into();
        self
    }

    fn is_flex(&self) -> bool {
        self.flex > 0.0
    }

    fn is_stretch(&self) -> bool {
        matches!(self.align, Some(AlignSelf::Stretch))
    }
}

#[derive(Clone, Debug)]
pub struct Flex {
    content: Vec<FlexChild>,
    axis: Axis,
    justify_content: JustifyContent,
    align_items: AlignItems,
    align_content: AlignContent,
    gap: Vec2,
}

impl Flex {
    pub const fn new(axis: Axis) -> Self {
        Self {
            content: Vec::new(),
            axis,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            align_content: AlignContent::Start,
            gap: Vec2::splat(5.0),
        }
    }

    pub const fn row() -> Self {
        Self::new(Axis::Row)
    }

    pub const fn column() -> Self {
        Self::new(Axis::Column)
    }

    pub fn child(mut self, child: impl Into<FlexChild>) -> Self {
        self.content.push(child.into());
        self
    }

    fn measure_fixed(
        &self,
        state: &mut FlexState,
        cx: &mut LayoutContext<'_>,
        max_major: f32,
        space: AvailableSpace,
    ) {
        state.lines.clear();

        let major_gap = self.axis.major(self.gap);

        let mut major = 0.0;
        let mut minor = 0.0;
        let mut flex_sum = 0.0;

        let mut start = 0;

        for (i, child) in self.content.iter().enumerate() {
            flex_sum += child.flex;

            let size = cx.child(i, &child.node, space);
            let (child_major, child_minor) = self.axis.unpack(size);
            state.majors[i] = child_major;

            let gap = if i > start + 1 || i == 1 {
                major_gap
            } else {
                0.0
            };

            if major + child_major + gap > max_major {
                state.lines.push(FlexLine {
                    start,
                    end: i,
                    major,
                    minor,
                    flex_sum,
                });

                start = i;
                major = child_major;
                minor = child_minor;
                flex_sum = child.flex;
            } else {
                major += child_major + gap;
                minor = minor.max(child_minor);
            }
        }

        state.lines.push(FlexLine {
            start,
            end: self.content.len(),
            major,
            minor: 0.0,
            flex_sum,
        });
    }

    fn measure_flex(
        &self,
        state: &mut FlexState,
        cx: &mut LayoutContext<'_>,
        min_major: f32,
        max_major: f32,
        max_minor: f32,
    ) {
        for line in state.lines.iter_mut() {
            let overflow = line.major - max_major;
            let underflow = min_major - line.major;

            let px_per_flex = if overflow > 0.0 {
                -overflow / line.flex_sum
            } else if underflow > 0.0 {
                underflow / line.flex_sum
            } else {
                continue;
            };

            for i in line.start..line.end {
                let child = &self.content[i];

                let is_stretch = self.align_items.is_stretch() || child.is_stretch();

                if !is_stretch && !child.is_flex() {
                    continue;
                }

                let desired_major = state.majors[i] + px_per_flex * child.flex;

                let space = if is_stretch {
                    AvailableSpace::new(
                        self.axis.pack(desired_major, line.minor),
                        self.axis.pack(desired_major, line.minor),
                    )
                } else {
                    AvailableSpace::new(
                        self.axis.pack(desired_major, 0.0),
                        self.axis.pack(desired_major, max_minor),
                    )
                };

                let size = cx.child(i, &child.node, space);
                let child_major = self.axis.major(size);

                line.major += child_major - state.majors[i];
                state.majors[i] = child_major;
            }
        }
    }
}

struct FlexLine {
    start: usize,
    end: usize,
    major: f32,
    minor: f32,
    flex_sum: f32,
}

#[doc(hidden)]
pub struct FlexState {
    lines: Vec<FlexLine>,
    positions: Vec<Vec2>,
    majors: Vec<f32>,
}

impl FlexState {
    fn line_minors(&self) -> Vec<f32> {
        self.lines.iter().map(|line| line.minor).collect()
    }

    fn major(&self) -> f32 {
        let mut major = 0.0f32;

        for line in self.lines.iter() {
            major = major.max(line.major);
        }

        major
    }

    fn minor(&self, minor_gap: f32) -> f32 {
        let total_gap = minor_gap * (self.lines.len() - 1) as f32;
        self.lines.iter().map(|line| line.minor).sum::<f32>() + total_gap
    }
}

impl FlexState {
    pub fn new(len: usize) -> Self {
        Self {
            lines: Vec::new(),
            positions: vec![Vec2::ZERO; len],
            majors: vec![0.0; len],
        }
    }
}

impl StateView for Flex {
    type State = FlexState;

    fn build(&self) -> Self::State {
        FlexState::new(self.content.len())
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        for (i, child) in self.content.iter().enumerate() {
            let position = state.positions[i];

            cx.with_translation(position, |cx| {
                cx.child(i, &child.node, event);
            });
        }
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let (max_major, max_minor) = self.axis.unpack(space.max);
        let (min_major, min_minor) = self.axis.unpack(space.min);

        self.measure_fixed(state, cx, max_major, space);
        self.measure_flex(state, cx, min_major, max_major, max_minor);

        let (major_gap, minor_gap) = self.axis.unpack(self.gap);

        let major = state.major().min(max_major);
        let minor = state.minor(minor_gap).max(min_minor);

        let minors = state.line_minors();
        let line_positions = self.align_content.layout(&minors, minor, minor_gap);

        for (i, line) in state.lines.iter().enumerate() {
            let child_majors = &state.majors[line.start..line.end];
            let child_positions = self.justify_content.layout(child_majors, major, major_gap);
            let line_position = line_positions[i];

            for j in line.start..line.end {
                let child_position = child_positions[j - line.start];
                state.positions[j] = self.axis.pack(child_position, line_position);
            }
        }

        self.axis.pack(major, minor).max(space.min)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        for (i, child) in self.content.iter().enumerate() {
            let position = state.positions[i];

            cx.with_translation(position, |cx| {
                cx.child(i, &child.node);
            });
        }
    }
}
