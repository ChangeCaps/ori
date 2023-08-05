use std::ops::Range;

use glam::Vec2;
use ori_graphics::Rect;
use ori_style::{Length, StyleAttributeEnum, StyleSpec};

use crate::Context;

/// The amount of space a [`View`](crate::View) is allowed to take up.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AvailableSpace {
    /// The minimum size the view can be.
    pub min: Vec2,
    /// The maximum size the view can be.
    pub max: Vec2,
}

impl AvailableSpace {
    #[allow(missing_docs)]
    pub const ZERO: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::ZERO,
    };

    #[allow(missing_docs)]
    pub const UNBOUNDED: Self = Self {
        min: Vec2::ZERO,
        max: Vec2::splat(f32::INFINITY),
    };

    /// Create a new [`AvailableSpace`] with the given minimum and maximum sizes.
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self {
            min: min.ceil(),
            max: max.ceil(),
        }
    }

    /// Create a new [`AvailableSpace`] for a window with the given width and height.
    pub fn window(width: u32, height: u32) -> Self {
        Self {
            min: Vec2::ZERO,
            max: Vec2::new(width as f32, height as f32),
        }
    }

    /// Loosen the constraints by setting the minimum size to zero.
    pub fn loosen(self) -> Self {
        Self {
            min: Vec2::ZERO,
            max: self.max,
        }
    }

    /// Shrink the constraints by the given amount.
    pub fn shrink(self, size: Vec2) -> Self {
        Self {
            min: Vec2::max(self.min - size, Vec2::ZERO),
            max: Vec2::max(self.max - size, Vec2::ZERO),
        }
    }

    /// Constrain the given size to the constraints.
    pub fn constrain(self, size: Vec2) -> Vec2 {
        size.ceil().clamp(self.min, self.max)
    }

    /// Returns true if the given size is within the constraints.
    pub fn contains(self, size: Vec2) -> bool {
        size.cmpge(self.min).all() && size.cmple(self.max).all()
    }

    /// Returns a range representing the x-axis of the constraints.
    pub fn x_axis(self) -> Range<f32> {
        self.min.x..self.max.x
    }

    /// Returns a range representing the y-axis of the constraints.
    pub fn y_axis(self) -> Range<f32> {
        self.min.y..self.max.y
    }

    /// Apply the given [`Padding`] to the constraints.
    pub fn apply_padding(self, padding: Padding) -> Self {
        Self {
            min: Vec2::max(self.min - padding.size(), Vec2::ZERO),
            max: Vec2::max(self.max - padding.size(), Vec2::ZERO),
        }
    }

    /// Apply the given [`Margin`] to the constraints.
    pub fn apply_margin(self, margin: Margin) -> Self {
        Self {
            min: Vec2::max(self.min - margin.size(), Vec2::ZERO),
            max: Vec2::max(self.max - margin.size(), Vec2::ZERO),
        }
    }
}

/// The space around the content of a [`View`](crate::View).
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Padding {
    /// The amount of space to the left of the content.
    pub left: f32,
    /// The amount of space to the right of the content.
    pub right: f32,
    /// The amount of space above the content.
    pub top: f32,
    /// The amount of space below the content.
    pub bottom: f32,
}

impl Padding {
    #[allow(missing_docs)]
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Create a new [`Padding`] with the given amount of space on all sides.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn from_style_named(cx: &mut Context<'_>, space: AvailableSpace, name: &str) -> Self {
        let t = format!("{}-top", name);
        let r = format!("{}-right", name);
        let b = format!("{}-bottom", name);
        let l = format!("{}-left", name);

        let padding = cx.query_style::<Length>(name);

        let t = cx.query_style::<Length>(&t);
        let r = cx.query_style::<Length>(&r);
        let b = cx.query_style::<Length>(&b);
        let l = cx.query_style::<Length>(&l);

        let t = StyleSpec::select(&[t, padding], Length::ZERO);
        let r = StyleSpec::select(&[r, padding], Length::ZERO);
        let b = StyleSpec::select(&[b, padding], Length::ZERO);
        let l = StyleSpec::select(&[l, padding], Length::ZERO);

        Self {
            left: cx.resolve_length(l, space.x_axis()),
            right: cx.resolve_length(r, space.x_axis()),
            top: cx.resolve_length(t, space.y_axis()),
            bottom: cx.resolve_length(b, space.y_axis()),
        }
    }

    /// Create a new [`Padding`] from the style of the element.
    pub fn from_style(context: &mut Context<'_>, space: AvailableSpace) -> Self {
        Self::from_style_named(context, space, "padding")
    }

    /// Returns the top left offset of the padding.
    pub fn top_left(self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    /// Returns the offset of the padding.
    pub fn offset(self) -> Vec2 {
        self.top_left()
    }

    /// Returns the size of the padding.
    pub fn size(self) -> Vec2 {
        Vec2::new(self.left + self.right, self.top + self.bottom)
    }

    /// Apply the padding to the given [`Rect`].
    pub fn apply(self, rect: Rect) -> Rect {
        Rect {
            min: rect.min + self.top_left(),
            max: rect.max - self.size(),
        }
    }
}

/// The space around a [`View`](crate::View).
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Margin {
    /// The amount of space to the left of the view.
    pub left: f32,
    /// The amount of space to the right of the view.
    pub right: f32,
    /// The amount of space above the view.
    pub top: f32,
    /// The amount of space below the view.
    pub bottom: f32,
}

impl Margin {
    #[allow(missing_docs)]
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Create a new [`Margin`] with the given amount of space on all sides.
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Create a new [`Margin`] from the style of the element.
    pub fn from_style(cx: &mut Context<'_>, space: AvailableSpace) -> Self {
        let margin = cx.query_style::<Length>("margin");

        let t = cx.query_style::<Length>("margin-top");
        let r = cx.query_style::<Length>("margin-right");
        let b = cx.query_style::<Length>("margin-bottom");
        let l = cx.query_style::<Length>("margin-left");

        let t = StyleSpec::select(&[t, margin], Length::ZERO);
        let r = StyleSpec::select(&[r, margin], Length::ZERO);
        let b = StyleSpec::select(&[b, margin], Length::ZERO);
        let l = StyleSpec::select(&[l, margin], Length::ZERO);

        Self {
            left: cx.resolve_length(l, space.x_axis()),
            right: cx.resolve_length(r, space.x_axis()),
            top: cx.resolve_length(t, space.y_axis()),
            bottom: cx.resolve_length(b, space.y_axis()),
        }
    }

    /// Returns the top left offset of the margin.
    pub fn top_left(self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    /// Returns the offset of the margin.
    pub fn offset(self) -> Vec2 {
        self.top_left()
    }

    /// Returns the size of the margin.
    pub fn size(self) -> Vec2 {
        Vec2::new(self.left + self.right, self.top + self.bottom)
    }

    /// Apply the margin to the given [`Rect`].
    pub fn apply(self, rect: Rect) -> Rect {
        Rect {
            min: rect.min - self.top_left(),
            max: rect.max + self.size(),
        }
    }
}

/// An axis, either horizontal or vertical.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Axis {
    /// The horizontal axis.
    Horizontal,
    /// The vertical axis, default.
    #[default]
    Vertical,
}

impl Axis {
    /// Returns the cross axis.
    pub const fn cross(self) -> Self {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }

    /// Returns the minor axis.
    pub const fn minor(self, size: Vec2) -> f32 {
        match self {
            Axis::Horizontal => size.y,
            Axis::Vertical => size.x,
        }
    }

    /// Returns the major axis.
    pub const fn major(self, size: Vec2) -> f32 {
        match self {
            Axis::Horizontal => size.x,
            Axis::Vertical => size.y,
        }
    }

    /// Packs the major and minor axis into a [`Vec2`].
    pub const fn pack(self, major: f32, minor: f32) -> Vec2 {
        match self {
            Axis::Horizontal => Vec2::new(major, minor),
            Axis::Vertical => Vec2::new(minor, major),
        }
    }

    /// Unpacks the major and minor axis from a [`Vec2`].
    pub const fn unpack(self, size: Vec2) -> (f32, f32) {
        match self {
            Axis::Horizontal => (size.x, size.y),
            Axis::Vertical => (size.y, size.x),
        }
    }
}

impl StyleAttributeEnum for Axis {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "horizontal" | "row" => Some(Axis::Horizontal),
            "vertical" | "column" => Some(Axis::Vertical),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Axis::Horizontal => "horizontal",
            Axis::Vertical => "vertical",
        }
    }
}

/// Justify content for a flex layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JustifyContent {
    /// Items are packed toward the start of the major axis.
    Start,
    /// Items are packed toward the center of the major axis.
    Center,
    /// Items are packed toward the end of the major axis.
    End,
    /// Items are evenly distributed along the major axis.
    SpaceBetween,
    /// Items are evenly distributed along the major axis, with half-size spaces on either end.
    SpaceAround,
    /// Items are evenly distributed along the major axis, with equal-size spaces between them.
    SpaceEvenly,
}

impl JustifyContent {
    /// Layout the given children along the major axis.
    pub fn layout(&self, content: &[f32], container_size: f32, gap: f32) -> Vec<f32> {
        if content.is_empty() {
            return Vec::new();
        }

        let mut positions = Vec::with_capacity(content.len());

        let total_gap = gap * (content.len() - 1) as f32;
        let total_size = content.iter().sum::<f32>() + total_gap;

        match self {
            JustifyContent::Start => {
                let mut position = 0.0;

                for &item in content {
                    positions.push(position);
                    position += item + gap;
                }
            }
            JustifyContent::Center => {
                let mut position = container_size / 2.0 - total_size / 2.0;

                for &item in content {
                    positions.push(position);
                    position += item + gap;
                }
            }
            JustifyContent::End => {
                let mut position = container_size - total_size;

                for &item in content {
                    positions.push(position);
                    position += item + gap;
                }
            }
            JustifyContent::SpaceBetween => {
                let gap = (container_size - total_size + total_gap) / (content.len() - 1) as f32;

                let mut position = 0.0;

                for &item in content {
                    positions.push(position);
                    position += item + gap;
                }
            }
            JustifyContent::SpaceAround => {
                let gap = (container_size - total_size) / content.len() as f32;

                let mut position = gap / 2.0;

                for &item in content {
                    positions.push(position);
                    position += item + gap;
                }
            }
            JustifyContent::SpaceEvenly => {
                let gap = container_size / content.len() as f32;

                let mut position = gap / 2.0;

                for _ in content {
                    positions.push(position);
                    position += gap;
                }
            }
        }

        positions
    }
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::Start
    }
}

impl StyleAttributeEnum for JustifyContent {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "start" => Some(JustifyContent::Start),
            "center" => Some(JustifyContent::Center),
            "end" => Some(JustifyContent::End),
            "space-between" => Some(JustifyContent::SpaceBetween),
            "space-around" => Some(JustifyContent::SpaceAround),
            "space-evenly" => Some(JustifyContent::SpaceEvenly),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            JustifyContent::Start => "start",
            JustifyContent::Center => "center",
            JustifyContent::End => "end",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        }
    }
}

/// Align items for a flex layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlignItem {
    /// Items are aligned toward the start of the minor axis.
    Start,
    /// Items are aligned toward the center of the minor axis.
    Center,
    /// Items are aligned toward the end of the minor axis.
    End,
    /// Items are stretched to fill the minor axis.
    Stretch,
}

impl Default for AlignItem {
    fn default() -> Self {
        Self::Start
    }
}

impl AlignItem {
    /// Align the given child along the minor axis.
    pub fn align(&self, start: f32, end: f32, size: f32) -> f32 {
        match self {
            AlignItem::Start => start,
            AlignItem::Center => start + (end - start - size) / 2.0,
            AlignItem::End => end - size,
            AlignItem::Stretch => start,
        }
    }
}

impl StyleAttributeEnum for AlignItem {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "start" => Some(AlignItem::Start),
            "center" => Some(AlignItem::Center),
            "end" => Some(AlignItem::End),
            "stretch" => Some(AlignItem::Stretch),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            AlignItem::Start => "start",
            AlignItem::Center => "center",
            AlignItem::End => "end",
            AlignItem::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl FlexWrap {
    pub const fn is_wrap(&self) -> bool {
        matches!(self, FlexWrap::Wrap | FlexWrap::WrapReverse)
    }
}

impl StyleAttributeEnum for FlexWrap {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "nowrap" => Some(FlexWrap::NoWrap),
            "wrap" => Some(FlexWrap::Wrap),
            "wrap-reverse" => Some(FlexWrap::WrapReverse),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }
}
