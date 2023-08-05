use std::f32::consts::PI;

use glam::Vec2;
use ori_graphics::{Curve, Quad, Rect};
use ori_macro::Build;
use ori_reactive::{Event, OwnedSignal};
use ori_style::Style;

use crate::{AvailableSpace, DrawContext, Element, EventContext, LayoutContext, PointerEvent};

/// A knob element.
#[derive(Clone, Debug, Build)]
pub struct Knob {
    /// The value of the knob.
    #[bind]
    pub value: OwnedSignal<f32>,
    /// The minimum value of the knob.
    #[prop]
    pub min: f32,
    /// The maximum value of the knob.
    #[prop]
    pub max: f32,
}

impl Default for Knob {
    fn default() -> Self {
        Self {
            value: OwnedSignal::new(0.0),
            min: 0.0,
            max: 1.0,
        }
    }
}

impl Element for Knob {
    type State = Option<Vec2>;

    fn build(&self) -> Self::State {
        None
    }

    fn style(&self) -> Style {
        Style::new("knob")
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if pointer_event.is_press() && cx.hovered() {
                cx.activate();
            } else if pointer_event.is_release() && cx.active() {
                *state = None;
                cx.deactivate();
            }

            if cx.active() {
                if let Some(prev_position) = *state {
                    let delta = pointer_event.position - prev_position;
                    let delta = delta.x - delta.y;
                    let range = self.max - self.min;
                    let delta = delta / cx.rect().width() * range * 0.15;
                    let value = self.value.get();
                    let new_value = f32::clamp(value + delta, self.min, self.max);
                    if new_value != value {
                        self.value.set(new_value);
                    }
                }

                *state = Some(pointer_event.position);
                cx.request_redraw();
            }
        }
    }

    fn layout(&self, _: &mut Self::State, cx: &mut LayoutContext, space: AvailableSpace) -> Vec2 {
        let size = cx.style_length("size", space.min.max_element()..space.max.min_element());
        space.constrain(Vec2::splat(size))
    }

    fn draw(&self, _: &mut Self::State, cx: &mut DrawContext) {
        let size = cx.rect().size().min_element();

        let diameter = size * 0.7;

        let background = cx.style_group(&["background-color", "background"]);

        let center = Quad {
            rect: Rect::center_size(cx.rect().center(), Vec2::splat(diameter)),
            background,
            border_radius: [diameter * 0.5; 4],
            border_width: cx.style_border_width("border", cx.parent_size),
            border_color: cx.style("border-color"),
        };
        cx.draw(center);

        let center = cx.rect().center();
        let ring_radius = diameter * 0.65;
        let ring_start = -PI * 1.25;
        let ring_end = PI * 0.25;
        let ring_length = ring_end - ring_start;
        let ring_stroke = diameter * 0.075;

        let ring_track = Curve::arc_center_angle(center, ring_radius, ring_start, ring_end);
        let mesh = ring_track.stroke(ring_stroke, background);
        cx.draw(mesh);

        let range = self.max - self.min;
        let value = self.value.get();
        let t = f32::clamp((value - self.min) / range, 0.0, 1.0);
        let angle = ring_start + t * ring_length;

        let ring = Curve::arc_center_angle(center, ring_radius, ring_start, angle);
        let mesh = ring.stroke(ring_stroke, cx.style("color"));
        cx.draw(mesh);

        let mut arm = Curve::new();
        arm.add_point(center);
        arm.add_point(center + Vec2::new(angle.cos(), angle.sin()) * ring_radius);

        let mesh = arm.stroke(ring_stroke, cx.style("color"));
        cx.draw(mesh);
    }
}
