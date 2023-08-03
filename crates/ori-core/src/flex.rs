use glam::Vec2;
use smallvec::{smallvec, SmallVec};

use crate::{
    AlignItem, AvailableSpace, Axis, Children, Context, FlexWrap, JustifyContent, LayoutContext,
    Node, Padding,
};

/// A layout that lays out children in a flexbox-like manner.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlexLayout {
    /// Padding around the children.
    pub padding: Padding,
    /// The axis to use for laying out the children.
    pub axis: Axis,
    /// The wrap mode of the children.
    pub wrap: FlexWrap,
    /// The justification of the children.
    pub justify_content: JustifyContent,
    /// The alignment of wrapped lines.
    pub align_content: JustifyContent,
    /// The alignment of the children.
    pub align_items: AlignItem,
    /// The gap between columns.
    pub column_gap: f32,
    /// The gap between rows.
    pub row_gap: f32,
}

impl Default for FlexLayout {
    fn default() -> Self {
        Self {
            padding: Padding::default(),
            axis: Axis::Vertical,
            wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::Start,
            align_content: JustifyContent::Start,
            align_items: AlignItem::Start,
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }
}

impl FlexLayout {
    /// Create a new flex layout.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new vertical flex layout.
    pub fn vertical() -> Self {
        Self {
            axis: Axis::Vertical,
            ..Self::default()
        }
    }

    /// Creates a new horizontal flex layout.
    pub fn horizontal() -> Self {
        Self {
            axis: Axis::Horizontal,
            ..Self::default()
        }
    }

    /// Creates a new row flex layout.
    pub fn row() -> Self {
        Self::horizontal()
    }

    /// Creates a new column flex layout.
    pub fn column() -> Self {
        Self::vertical()
    }

    /// Gets the flex layout from the style of an element.
    pub fn from_style(cx: &mut LayoutContext) -> Self {
        let padding = cx.node.padding;
        let axis = cx.style::<Axis>("direction");
        let wrap = cx.style::<FlexWrap>("flex-wrap");
        let justify_content = cx.style("justify-content");
        let align_content = cx.style("align-content");
        let align_items = cx.style("align-items");

        let column_range = cx.parent_space.min.x..cx.parent_space.max.x;
        let row_range = cx.parent_space.min.y..cx.parent_space.max.y;

        let column_gap = cx.style_length_group(&["column-gap", "gap"], column_range);
        let row_gap = cx.style_length_group(&["row-gap", "gap"], row_range);

        Self {
            padding,
            axis,
            wrap,
            justify_content,
            align_content,
            align_items,
            column_gap,
            row_gap,
        }
    }
}

#[derive(Clone, Copy)]
struct WrapLine {
    start: usize,
    end: usize,
    major: f32,
    minor: f32,
    flex_grow_sum: f32,
    flex_shrink_sum: f32,
}

impl WrapLine {
    pub fn nodes(self, children: &Children) -> impl Iterator<Item = Node> + '_ {
        let len = self.end - self.start;
        children.nodes().skip(self.start).take(len)
    }

    pub fn nodes_enumerate(self, children: &Children) -> impl Iterator<Item = (usize, Node)> + '_ {
        let len = self.end - self.start;
        children.nodes().enumerate().skip(self.start).take(len)
    }
}

impl Children {
    fn child_flex(cx: &mut LayoutContext, child: &Node) -> (Option<f32>, Option<f32>) {
        // get the flex grow and shrink factors
        let flex_grow = child.style::<Option<f32>>(cx, "flex-grow");
        let flex_shrink = child.style::<Option<f32>>(cx, "flex-shrink");

        // get the flex shorthand property
        let flex = child.style::<Option<f32>>(cx, "flex");
        match flex {
            Some(flex) => (
                Some(flex_shrink.unwrap_or(flex)),
                Some(flex_grow.unwrap_or(1.0)),
            ),
            None => (flex_grow, flex_shrink),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_fixed(
        &self,
        cx: &mut LayoutContext,
        axis: Axis,
        gap_major: f32,
        wrap: FlexWrap,
        max_major: f32,
        child_majors: &mut [f32],
        child_flexes: &mut [(Option<f32>, Option<f32>)],
        space: AvailableSpace,
    ) -> SmallVec<[WrapLine; 2]> {
        let needs_layout = self.needs_layout();
        let loosend_space = space.loosen();

        let mut minor = 0.0;
        let mut major = 0.0;
        let mut flex_grow_sum = 0.0;
        let mut flex_shrink_sum = 0.0;
        let mut wraps = smallvec![];

        let mut start = 0;

        // first we need to measure the fixed-sized children to determine their size
        for (i, child) in self.nodes().enumerate() {
            let (flex_grow, flex_shrink) = Self::child_flex(cx, &child);
            let is_flex = flex_grow.is_some() || flex_shrink.is_some();

            // add the flex grow and shrink factors to the sum
            flex_grow_sum += flex_grow.unwrap_or(0.0);
            flex_shrink_sum += flex_shrink.unwrap_or(0.0);

            // store the flex grow and shrink factors
            child_flexes[i] = (flex_grow, flex_shrink);

            // layout the child
            let space_changed = child.space_changed(space);
            let size = if needs_layout || space_changed {
                let size = child.layout(cx, loosend_space);
                child.set_available_space(space);
                size
            } else {
                child.size()
            };

            let (child_major, child_minor) = axis.unpack(size);

            // store the size
            child_majors[i] = child_major;

            if major > 0.0 {
                major += gap_major;
            }

            if major + child_major > max_major && wrap.is_wrap() {
                let line = WrapLine {
                    start,
                    end: i,
                    major,
                    minor,
                    flex_grow_sum,
                    flex_shrink_sum,
                };
                wraps.push(line);

                start = i;
                major = 0.0;
                minor = child_minor;
                flex_grow_sum = 0.0;
                flex_shrink_sum = 0.0;
            }

            // update the major and minor axis
            major += child_major;

            if !is_flex {
                minor = minor.max(child_minor);
            }
        }

        let line = WrapLine {
            start,
            end: self.len(),
            major,
            minor,
            flex_grow_sum,
            flex_shrink_sum,
        };
        wraps.push(line);

        wraps
    }

    #[allow(clippy::too_many_arguments)]
    fn measure_flex(
        &self,
        cx: &mut LayoutContext,
        axis: Axis,
        max_major: f32,
        max_minor: f32,
        lines: &mut [WrapLine],
        child_majors: &mut [f32],
        child_flexes: &[(Option<f32>, Option<f32>)],
    ) {
        for line in lines {
            // now we need to measure the flex-sized children to determine their size
            let remaining_major = max_major - line.major;
            let should_grow = remaining_major > 0.0;

            // calculate the amount of pixels per flex
            let px_per_flex = if should_grow {
                remaining_major / line.flex_grow_sum
            } else {
                remaining_major / line.flex_shrink_sum
            };

            for (i, child) in line.nodes_enumerate(self) {
                // if the child has a flex property, now is the time
                let (flex_grow, flex_shrink) = child_flexes[i];
                if flex_grow.is_none() && should_grow || flex_shrink.is_none() && !should_grow {
                    continue;
                }

                // calculate the desired size of the child
                let desired_major = if should_grow {
                    child_majors[i] + px_per_flex * flex_grow.unwrap()
                } else {
                    child_majors[i] + px_per_flex * flex_shrink.unwrap()
                };

                if desired_major == child_majors[i] {
                    continue;
                }

                let child_space = AvailableSpace {
                    min: axis.pack(desired_major, 0.0),
                    max: axis.pack(desired_major, max_minor),
                };

                let size = child.relayout(cx, child_space);
                let (child_major, child_minor) = axis.unpack(size);

                // update the major and minor axis
                line.minor = line.minor.max(child_minor);
                line.major += child_major - child_majors[i];

                // store the size
                child_majors[i] = child_major;
            }
        }
    }

    fn stretch_items(
        &self,
        cx: &mut LayoutContext,
        axis: Axis,
        align_items: AlignItem,
        lines: &[WrapLine],
        child_majors: &mut [f32],
    ) {
        for line in lines {
            for (i, child) in line.nodes_enumerate(self) {
                let align_self = child.style::<Option<AlignItem>>(cx, "align-self");

                if align_items != AlignItem::Stretch && align_self != Some(AlignItem::Stretch) {
                    continue;
                }

                // calculate the constraints for the child
                let child_major = child_majors[i];
                let child_size = axis.pack(child_major, line.minor);
                let child_space = AvailableSpace {
                    min: child_size,
                    max: child_size,
                };

                // FIXME: calling layout again is not ideal, but it's the only way to get the
                // correct size for the child, since we don't know the minor size until we've
                // measured all the children
                let size = if child_size != child.size() {
                    child.relayout(cx, child_space)
                } else {
                    child.size()
                };

                child_majors[i] = axis.major(size);
            }
        }
    }

    /// Layout the children using a FlexLayout.
    pub(crate) fn flex_layout_padded(
        &self,
        cx: &mut LayoutContext,
        space: AvailableSpace,
        flex: FlexLayout,
    ) -> Vec2 {
        let FlexLayout {
            padding,
            axis,
            wrap,
            justify_content,
            align_content,
            align_items,
            column_gap,
            row_gap,
        } = flex;

        // calculate the bounds of the major and minor axis
        let (min_major, min_minor) = axis.unpack(space.min);
        let (max_major, max_minor) = axis.unpack(space.max);
        let (gap_major, gap_minor) = axis.unpack(Vec2::new(column_gap, row_gap));

        // NOTE: using a SmallVec here is a bit faster than using a Vec, but it's not a huge
        // difference
        let mut child_majors: SmallVec<[f32; 4]> = smallvec![0.0; self.len()];
        let mut child_flexes: SmallVec<[_; 4]> = smallvec![(None, None); self.len()];

        let mut lines = self.measure_fixed(
            cx,
            axis,
            gap_major,
            wrap,
            max_major,
            &mut child_majors,
            &mut child_flexes,
            space,
        );

        self.measure_flex(
            cx,
            axis,
            max_major,
            max_minor,
            &mut lines,
            &mut child_majors,
            &child_flexes,
        );

        self.stretch_items(cx, axis, align_items, &lines, &mut child_majors);

        let mut major: f32 = 0.0;
        let mut minor: f32 = 0.0;

        for line in &lines {
            major = major.max(line.major);

            if minor > 0.0 {
                minor += gap_minor;
            }

            minor += line.minor;
        }

        major = major.max(min_major);
        minor = minor.max(min_minor);

        if !wrap.is_wrap() {
            lines[0].minor = minor;
        }

        let line_minors: SmallVec<[_; 2]> = lines.iter().map(|wrap| wrap.minor).collect();
        let line_minors = align_content.layout(&line_minors, minor, gap_minor);

        let lines: SmallVec<[_; 2]> = if wrap == FlexWrap::WrapReverse {
            lines.into_iter().rev().collect()
        } else {
            lines
        };

        for (i, wrap) in lines.into_iter().enumerate() {
            let child_majors = &child_majors[wrap.start..wrap.end];
            let child_offsets = justify_content.layout(child_majors, major, gap_major);
            let minor = line_minors[i];

            // now we can layout the children
            for (child, align_major) in wrap.nodes(self).zip(child_offsets) {
                // get the align item for the child
                let align_item = match child.style::<Option<AlignItem>>(cx, "align-self") {
                    Some(align) => align,
                    None => align_items,
                };

                // align the minor axis
                let child_minor = axis.minor(child.size());
                let align_minor = align_item.align(0.0, wrap.minor, child_minor);

                // set the offset for the child
                let child_offset = axis.pack(align_major, minor + align_minor);
                child.set_offset(padding.top_left() + child_offset);
            }
        }

        // return the size of the flex container
        axis.pack(major, minor).max(space.min)
    }
}
