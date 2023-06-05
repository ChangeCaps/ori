use glam::Vec2;
use ori_graphics::{Color, Glyph, Quad, Rect, TextSection};
use ori_macro::Build;
use ori_reactive::{Emitter, Event, OwnedSignal};
use ori_style::Style;

use crate::{
    AvailableSpace, Context, DrawContext, Element, EventContext, Key, KeyboardEvent, LayoutContext,
    PointerEvent,
};

#[derive(Clone, Debug, Build)]
pub struct TextInput {
    /// The text of the input.
    #[prop]
    #[bind]
    pub text: OwnedSignal<String>,
    /// The placeholder text of the input.
    #[prop]
    pub placeholder: String,
    /// The callback to be called when enter is pressed.
    #[event]
    pub on_input: Emitter<String>,
    /// Whether the input can have multiple lines.
    #[prop]
    pub multiline: bool,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: OwnedSignal::new(String::new()),
            placeholder: String::from("Type here..."),
            on_input: Emitter::new(),
            multiline: false,
        }
    }
}

impl TextInput {
    fn is_placeholder(&self) -> bool {
        self.text.get().is_empty()
    }

    fn text(&self) -> String {
        let text = self.text.get();

        if text.is_empty() {
            self.placeholder.clone()
        } else {
            text
        }
    }

    fn section(&self, state: &TextInputState, cx: &mut impl Context) -> TextSection {
        let color = if self.is_placeholder() {
            cx.style("placeholder-color")
        } else {
            cx.style("color")
        };

        TextSection {
            text: "",
            font_size: state.font_size,
            font_family: cx.style("font-family"),
            font_weight: cx.style("font-weight"),
            font_stretch: cx.style("font-stretch"),
            font_style: cx.style("font-style"),
            color,
            v_align: cx.style("text-valign"),
            h_align: cx.style("text-align"),
            line_height: cx.style("line-height"),
            wrap: cx.style("text-wrap"),
            rect: cx.padding().apply(cx.rect()),
        }
    }

    fn cursor_select(&self, state: &mut TextInputState, cx: &mut impl Context, position: Vec2) {
        let section = TextSection {
            text: &self.text.get(),
            ..self.section(state, cx)
        };

        let mut line = None;
        let mut dist = f32::MAX;

        for glyph in cx.fonts_mut().text_glyphs(&section).into_iter().flatten() {
            let delta = position - glyph.rect.center();

            if glyph.rect.contains(position) {
                state.cursor = glyph.byte_offset;

                if delta.x > 0.0 {
                    state.cursor += glyph.code.len_utf8();
                }

                break;
            }

            if line != Some(glyph.line) && line.is_some() {
                continue;
            }

            let line_top = glyph.baseline - glyph.line_descent;
            let line_bottom = glyph.baseline - glyph.line_ascent;

            if position.y < line_bottom || position.y > line_top {
                continue;
            }

            if delta.length_squared() < dist {
                line = Some(glyph.line);
                dist = delta.length_squared();

                state.cursor = glyph.byte_offset;
            }
        }

        state.cursor_blink = 0.0;
        cx.request_redraw();
    }

    fn pointer_event(
        &self,
        state: &mut TextInputState,
        cx: &mut impl Context,
        event: &PointerEvent,
    ) -> bool {
        if event.is_press() && cx.hovered() {
            self.cursor_select(state, cx, event.position);
            cx.focus();
            return true;
        }

        if event.is_press() && !cx.hovered() {
            cx.unfocus();
            return false;
        }

        false
    }

    fn key(&self, state: &mut TextInputState, cx: &mut impl Context, key: Key) -> bool {
        match key {
            Key::Backspace => self.input_backspace(state, cx),
            Key::Enter => self.input_enter(state, cx),
            Key::Left => self.input_left(state, cx),
            Key::Right => self.input_right(state, cx),
            _ => return false,
        }

        true
    }

    fn prev_char(&self, state: &TextInputState) -> Option<char> {
        let text = self.text.get();

        for i in 1..=4 {
            if state.cursor < i {
                continue;
            }

            if text.is_char_boundary(state.cursor - i) {
                return text[state.cursor - i..].chars().next();
            }
        }

        None
    }

    fn next_char(&self, state: &TextInputState) -> Option<char> {
        self.text.get()[state.cursor..].chars().next()
    }

    fn input_backspace(&self, state: &mut TextInputState, cx: &mut impl Context) {
        let mut text = self.text.modify();
        let Some(prev_char) = self.prev_char(state) else { return };
        text.remove(state.cursor - prev_char.len_utf8());
        state.cursor -= prev_char.len_utf8();
        state.cursor_blink = 0.0;

        cx.request_layout();
    }

    fn input_enter(&self, state: &mut TextInputState, cx: &mut impl Context) {
        if self.on_input.is_empty() {
            return;
        }

        let text = self.text.get();

        if self.multiline && state.cursor <= text.len() {
            return;
        }

        self.on_input.emit(&text);
        cx.unfocus();
    }

    fn input_left(&self, state: &mut TextInputState, _cx: &mut impl Context) {
        if let Some(prev_char) = self.prev_char(state) {
            state.cursor -= prev_char.len_utf8();
            state.cursor_blink = 0.0;
        }
    }

    fn input_right(&self, state: &mut TextInputState, _cx: &mut impl Context) {
        if let Some(next_char) = self.next_char(state) {
            state.cursor += next_char.len_utf8();
            state.cursor_blink = 0.0;
        }
    }

    fn input_text(&self, state: &mut TextInputState, cx: &mut impl Context, input: &str) {
        let mut input = input.replace('\x08', "");

        if !self.multiline {
            input = input.replace(|c: char| c == '\n' || c == '\r', "");
        }

        let mut text = self.text.modify();

        text.insert_str(state.cursor, &input);
        state.cursor += input.len();
        state.cursor_blink = 0.0;

        cx.request_layout();
    }

    fn keyboard_event(
        &self,
        state: &mut TextInputState,
        cx: &mut impl Context,
        event: &KeyboardEvent,
    ) -> bool {
        if !cx.focused() {
            return false;
        }

        if let Some(ref text) = event.text {
            self.input_text(state, cx, text);
            return true;
        }

        if let Some(key) = event.key {
            if event.is_press() {
                return self.key(state, cx, key);
            }
        }

        false
    }

    fn find_glyph(&self, state: &TextInputState, glyphs: &[Glyph]) -> Option<Glyph> {
        glyphs
            .iter()
            .find(|glyph| glyph.byte_offset == state.cursor)
            .copied()
    }

    fn cursor_position(&self, state: &TextInputState, glyphs: &[Glyph]) -> Option<Vec2> {
        match self.find_glyph(state, glyphs) {
            Some(glyph) => Some(Vec2::new(
                glyph.rect.min.x,
                glyph.baseline + glyph.line_descent,
            )),
            None => {
                let last = glyphs.last()?;
                Some(Vec2::new(
                    last.rect.max.x,
                    last.baseline + last.line_descent,
                ))
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextInputState {
    pub cursor_blink: f32,
    pub font_size: f32,
    pub cursor: usize,
}

impl Element for TextInput {
    type State = TextInputState;

    fn build(&self) -> Self::State {
        TextInputState::default()
    }

    fn style(&self) -> Style {
        Style::new("text-input")
    }

    fn event(&self, state: &mut Self::State, cx: &mut EventContext, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if self.pointer_event(state, cx, pointer_event) {
                event.handle();
            }
        }

        if let Some(keyboard_event) = event.get::<KeyboardEvent>() {
            if self.keyboard_event(state, cx, keyboard_event) {
                event.handle();
            }
        }
    }

    fn layout(
        &self,
        state: &mut Self::State,
        cx: &mut LayoutContext,
        space: AvailableSpace,
    ) -> Vec2 {
        state.font_size = cx.style_range("font-size", 0.0..cx.parent_space.max.y);

        let section = TextSection {
            text: &self.text(),
            rect: Rect::min_size(Vec2::ZERO, space.max - cx.padding().size()),
            ..self.section(state, cx)
        };

        let mut text_size = cx.measure_text(&section).size() + cx.padding().size();
        text_size.y = f32::max(text_size.y, state.font_size + cx.padding().size().y);
        space.constrain(text_size)
    }

    fn draw(&self, state: &mut Self::State, cx: &mut DrawContext) {
        cx.draw_quad();

        let section = TextSection {
            text: &self.text(),
            ..self.section(state, cx)
        };

        cx.draw_text(&section);

        if !cx.focused() {
            return;
        }

        let glyphs = cx.fonts_mut().text_glyphs(&section).unwrap_or_default();
        let Some(cursor_center) = self.cursor_position(state, &glyphs) else { return; };

        let cursor_size = Vec2::new(1.0, state.font_size);
        let cursor_rect = Rect::center_size(cursor_center, cursor_size);

        state.cursor_blink += cx.delta_time() * 10.0;
        cx.request_redraw();

        let mut color = cx.style::<Color>("color");
        color.a = state.cursor_blink.sin() * 0.5 + 0.5;

        let quad = Quad {
            rect: cursor_rect,
            background: color,
            border_radius: [0.0; 4],
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        };

        cx.draw(quad);
    }
}
