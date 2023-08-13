use ori_graphics::math::Vec2;
use ori_reactive::Event;

use crate::{
    AlignContent, AlignItems, AlignSelf, AvailableSpace, Axis, Context, DrawContext, EventContext,
    JustifyContent, LayoutContext, Length, Node, Size, StateView, Unit,
};

#[derive(Debug)]
pub struct Flex {
    pub content: Node,
    pub flex: f32,
    pub align: Option<AlignSelf>,
}

impl<T: Into<Node>> From<T> for Flex {
    fn from(view: T) -> Self {
        Self::new(0.0, view)
    }
}

impl Flex {
    pub fn new(flex: f32, view: impl Into<Node>) -> Self {
        Self {
            content: view.into(),
            flex,
            align: None,
        }
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

#[derive(Debug)]
pub struct Stack {
    pub content: Vec<Flex>,
    pub size: Size,
    pub axis: Axis,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    pub gap_column: Unit,
    pub gap_row: Unit,
}

impl Default for Stack {
    fn default() -> Self {
        Self::column()
    }
}

impl Stack {
    pub const fn new(axis: Axis) -> Self {
        Self {
            content: Vec::new(),
            size: Size::content(),
            axis,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            align_content: AlignContent::Start,
            gap_column: Unit::Em(0.5),
            gap_row: Unit::Em(0.5),
        }
    }

    pub const fn row() -> Self {
        Self::new(Axis::Row)
    }

    pub const fn column() -> Self {
        Self::new(Axis::Column)
    }

    pub const fn hstack() -> Self {
        Self::row()
    }

    pub const fn vstack() -> Self {
        Self::column()
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.size.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.size.height = height.into();
        self
    }

    pub fn justify_content(mut self, justify_content: JustifyContent) -> Self {
        self.justify_content = justify_content;
        self
    }

    pub fn align_items(mut self, align_items: AlignItems) -> Self {
        self.align_items = align_items;
        self
    }

    pub fn align_content(mut self, align_content: AlignContent) -> Self {
        self.align_content = align_content;
        self
    }

    pub fn gap(mut self, gap: impl Into<Unit>) -> Self {
        self.gap_column = gap.into();
        self.gap_row = self.gap_column;
        self
    }

    pub fn gap_column(mut self, gap: impl Into<Unit>) -> Self {
        self.gap_column = gap.into();
        self
    }

    pub fn gap_row(mut self, gap: impl Into<Unit>) -> Self {
        self.gap_row = gap.into();
        self
    }

    pub fn push(&mut self, child: impl Into<Flex>) {
        self.content.push(child.into());
    }

    pub fn with(mut self, child: impl Into<Flex>) -> Self {
        self.push(child);
        self
    }

    fn measure_fixed(
        &mut self,
        state: &mut StackState,
        cx: &mut LayoutContext<'_>,
        gap_major: f32,
        max_major: f32,
        space: AvailableSpace,
    ) {
        state.lines.clear();

        let mut major = 0.0;
        let mut minor = 0.0;
        let mut flex_sum = 0.0;

        let mut start = 0;

        for (i, child) in self.content.iter_mut().enumerate() {
            flex_sum += child.flex;

            let size = child.content.layout_indexed(i, cx, space);
            let (child_major, child_minor) = self.axis.unpack(size);
            state.majors[i] = child_major;
            state.minors[i] = child_minor;

            let gap = if i > start + 1 || i == 1 {
                gap_major
            } else {
                0.0
            };

            if major + child_major + gap > max_major {
                state.lines.push(StackLine {
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

        state.lines.push(StackLine {
            start,
            end: self.content.len(),
            major,
            minor,
            flex_sum,
        });
    }

    fn measure_flex(
        &mut self,
        state: &mut StackState,
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
                let child = &mut self.content[i];

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

                let size = child.content.layout_indexed(i, cx, space);
                let (child_major, child_minor) = self.axis.unpack(size);

                line.major += child_major - state.majors[i];
                state.majors[i] = child_major;
                state.minors[i] = child_minor;
            }
        }
    }
}

#[derive(Debug)]
struct StackLine {
    start: usize,
    end: usize,
    major: f32,
    minor: f32,
    flex_sum: f32,
}

#[doc(hidden)]
pub struct StackState {
    lines: Vec<StackLine>,
    line_offsets: Vec<f32>,
    positions: Vec<Vec2>,
    majors: Vec<f32>,
    minors: Vec<f32>,
}

impl StackState {
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

impl StackState {
    pub fn new(len: usize) -> Self {
        Self {
            lines: Vec::new(),
            line_offsets: Vec::new(),
            positions: vec![Vec2::ZERO; len],
            majors: vec![0.0; len],
            minors: vec![0.0; len],
        }
    }
}

impl StateView for Stack {
    type State = StackState;

    fn build(&mut self, _cx: &mut Context<'_>) -> Self::State {
        StackState::new(self.content.len())
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        for (i, child) in self.content.iter_mut().enumerate() {
            let position = state.positions[i];

            cx.with_translation(position, |cx| {
                child.content.event_indexed(i, cx, event);
            });
        }
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        let content_space = self.size.content_space(cx, space);

        let (max_major, max_minor) = self.axis.unpack(content_space.max);
        let (min_major, min_minor) = self.axis.unpack(content_space.min);

        let gap_column = self.gap_column.get(cx);
        let gap_row = self.gap_row.get(cx);

        let (gap_major, gap_minor) = self.axis.unpack(Vec2::new(gap_column, gap_row));

        self.measure_fixed(state, cx, gap_major, max_major, content_space);
        self.measure_flex(state, cx, min_major, max_major, max_minor);

        let content_major = state.major().min(max_major);
        let content_minor = state.minor(gap_minor).max(min_minor);

        let content_size = self.axis.pack(content_major, content_minor);
        let size = self.size.get(cx, content_size, space);

        let (major, minor) = self.axis.unpack(size);

        state.line_offsets.resize(state.lines.len(), 0.0);

        self.align_content.layout(
            state.lines.iter().map(|line| line.minor),
            |index, offset| state.line_offsets[index] = offset,
            minor,
            gap_minor,
        );

        for (i, line) in state.lines.iter().enumerate() {
            let line_offset = state.line_offsets[i];
            let child_majors = &state.majors[line.start..line.end];
            let child_minors = &state.minors[line.start..line.end];
            let child_positions = &mut state.positions[line.start..line.end];

            self.justify_content.layout(
                child_majors.iter().copied(),
                |index, offset| {
                    let align = self.align_items.align(line.minor, child_minors[index]);
                    child_positions[index] = self.axis.pack(offset, line_offset + align)
                },
                major,
                gap_major,
            );
        }

        size
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        for (i, child) in self.content.iter_mut().enumerate() {
            let position = state.positions[i];

            cx.with_translation(position, |cx| {
                child.content.draw_indexed(i, cx);
            });
        }
    }
}

/// Create a flex layout, on the horizontal axis.
#[macro_export]
macro_rules! hstack {
    ($($child:expr),* $(,)?) => {
        $crate::views::Stack::hstack()
            $(.with($child))*
    };
}

/// Create a flex layout, on the vertical axis.
#[macro_export]
macro_rules! vstack {
    ($($child:expr),* $(,)?) => {
        $crate::views::Stack::vstack()
            $(.with($child))*
    };
}
