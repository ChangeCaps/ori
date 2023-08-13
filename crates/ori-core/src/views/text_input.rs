use ori_graphics::{
    math::Vec2, Color, FontFamily, FontStretch, FontStyle, FontWeight, Glyph, Glyphs, Quad, Rect,
    TextAlign, TextSection, TextWrap,
};
use ori_reactive::{Callback, Event, OwnedSignal};

use crate::{
    AvailableSpace, Code, Context, Cursor, DrawContext, EventContext, Key, KeyboardEvent,
    LayoutContext, Length, Modifiers, PointerEvent, RequestLayoutEvent, Size, StateView, Style,
    Styled, Unit,
};

#[derive(Clone)]
pub struct TextInput {
    pub text: OwnedSignal<String>,
    pub placeholder: String,
    pub multiline: bool,
    pub size: Size,
    pub on_submit: Option<Callback<'static, str>>,
    pub font_size: Style<Unit>,
    pub font_family: Style<FontFamily>,
    pub font_weight: Style<FontWeight>,
    pub font_stretch: Style<FontStretch>,
    pub font_style: Style<FontStyle>,
    pub color: Style<Color>,
    pub v_align: Style<TextAlign>,
    pub h_align: Style<TextAlign>,
    pub line_height: Style<f32>,
    pub wrap: Style<TextWrap>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            text: OwnedSignal::new(String::new()),
            placeholder: String::from("Text..."),
            multiline: false,
            size: Size::content(),
            on_submit: None,
            font_size: Style::new(Self::FONT_SIZE),
            font_family: Style::new(Self::FONT_FAMILY),
            font_weight: Style::new(Self::FONT_WEIGHT),
            font_stretch: Style::new(Self::FONT_STRETCH),
            font_style: Style::new(Self::FONT_STYLE),
            color: Style::new(Self::COLOR),
            v_align: Style::new(Self::V_ALIGN),
            h_align: Style::new(Self::H_ALIGN),
            line_height: Style::new(Self::LINE_HEIGHT),
            wrap: Style::new(Self::WRAP),
        }
    }
}

impl TextInput {
    pub const FONT_SIZE: Key<Unit> = Key::new("text-input.font-size");
    pub const FONT_FAMILY: Key<FontFamily> = Key::new("text-input.font-family");
    pub const FONT_WEIGHT: Key<FontWeight> = Key::new("text-input.font-weight");
    pub const FONT_STRETCH: Key<FontStretch> = Key::new("text-input.font-stretch");
    pub const FONT_STYLE: Key<FontStyle> = Key::new("text-input.font-style");
    pub const COLOR: Key<Color> = Key::new("text-input.color");
    pub const V_ALIGN: Key<TextAlign> = Key::new("text-input.v-align");
    pub const H_ALIGN: Key<TextAlign> = Key::new("text-input.h-align");
    pub const LINE_HEIGHT: Key<f32> = Key::new("text-input.line-height");
    pub const WRAP: Key<TextWrap> = Key::new("text-input.wrap");

    pub fn new(text: impl Into<OwnedSignal<String>>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
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

    pub fn on_submit(mut self, on_submit: impl FnMut(&str) + Send + 'static) -> Self {
        self.on_submit = Some(on_submit.into());
        self
    }

    pub fn font_size(mut self, font_size: impl Styled<Unit>) -> Self {
        self.font_size = font_size.style();
        self
    }

    pub fn font_family(mut self, font_family: impl Styled<FontFamily>) -> Self {
        self.font_family = font_family.style();
        self
    }

    pub fn font_weight(mut self, font_weight: impl Styled<FontWeight>) -> Self {
        self.font_weight = font_weight.style();
        self
    }

    pub fn font_stretch(mut self, font_stretch: impl Styled<FontStretch>) -> Self {
        self.font_stretch = font_stretch.style();
        self
    }

    pub fn font_style(mut self, font_style: impl Styled<FontStyle>) -> Self {
        self.font_style = font_style.style();
        self
    }

    pub fn color(mut self, color: impl Styled<Color>) -> Self {
        self.color = color.style();
        self
    }

    pub fn v_align(mut self, v_align: impl Styled<TextAlign>) -> Self {
        self.v_align = v_align.style();
        self
    }

    pub fn h_align(mut self, h_align: impl Styled<TextAlign>) -> Self {
        self.h_align = h_align.style();
        self
    }

    pub fn line_height(mut self, line_height: impl Styled<f32>) -> Self {
        self.line_height = line_height.style();
        self
    }

    pub fn wrap(mut self, wrap: impl Styled<TextWrap>) -> Self {
        self.wrap = wrap.style();
        self
    }

    fn is_placeholder(&self) -> bool {
        self.text.get().is_empty()
    }

    fn text(&self) -> String {
        let mut text = self.text.get();

        if text.is_empty() {
            text = self.placeholder.clone();
        }

        text.push(' ');
        text
    }

    fn cursor_select(&self, state: &mut TextInputState, cx: &EventContext<'_>, local: Vec2) {
        if self.text.get().is_empty() {
            state.cursor_index = 0;
            return;
        }

        let mut line = None;
        let mut dist = f32::MAX;

        for glyph in state.glyphs.iter().flatten() {
            let delta = local - glyph.rect.center();

            if glyph.rect.contains(local) {
                state.cursor_index = glyph.byte_offset;

                if delta.x > 0.0 {
                    state.cursor_index += glyph.code.len_utf8();
                }

                break;
            }

            if line != Some(glyph.line) && line.is_some() {
                continue;
            }

            let line_top = glyph.baseline - glyph.line_descent;
            let line_bottom = glyph.baseline - glyph.line_ascent;

            if local.y < line_bottom || local.y > line_top {
                continue;
            }

            if delta.length_squared() < dist {
                line = Some(glyph.line);
                dist = delta.length_squared();

                state.cursor_index = glyph.byte_offset;
            }
        }

        state.cursor_blink = 0.0;
        cx.request_redraw();
    }

    fn handle_pointer_event(
        &self,
        state: &mut TextInputState,
        cx: &mut EventContext<'_>,
        event: &PointerEvent,
    ) -> bool {
        let local = cx.local(event.position);

        let hovered = cx.rect().contains(local);

        if !state.hovered && hovered {
            cx.set_cursor(Some(Cursor::Text));
        } else {
            cx.set_cursor(None);
        }

        if event.is_press() && hovered {
            state.focused = true;
            self.cursor_select(state, cx, local);
            cx.request_redraw();
            return true;
        }

        if event.is_press() && !hovered {
            state.focused = false;
            return false;
        }

        false
    }

    fn prev_char(&self, state: &TextInputState) -> Option<char> {
        let text = self.text.get();

        for i in 1..=4 {
            if state.cursor_index < i {
                continue;
            }

            if text.is_char_boundary(state.cursor_index - i) {
                return text[state.cursor_index - i..].chars().next();
            }
        }

        None
    }

    fn next_char(&self, state: &TextInputState) -> Option<char> {
        self.text.get()[state.cursor_index..].chars().next()
    }

    fn input_text(&self, state: &mut TextInputState, cx: &EventContext<'_>, input: &str) {
        let mut input = input.replace('\x08', "");

        if !self.multiline {
            input = input.replace(['\n', '\r'], "");
        }

        let mut text = self.text.modify();

        text.insert_str(state.cursor_index, &input);
        state.cursor_index += input.len();
        state.cursor_blink = 0.0;

        cx.request_layout();
    }

    fn input_key(
        &self,
        state: &mut TextInputState,
        cx: &EventContext,
        modifiers: Modifiers,
        key: Code,
    ) -> bool {
        match key {
            Code::Backspace => self.input_backspace(state, cx),
            Code::Enter => self.input_enter(state, modifiers),
            Code::Left => self.input_left(state, cx),
            Code::Right => self.input_right(state, cx),
            _ => return false,
        }

        true
    }

    fn input_backspace(&self, state: &mut TextInputState, cx: &EventContext<'_>) {
        let mut text = self.text.modify();
        let Some(prev_char) = self.prev_char(state) else { return };
        text.remove(state.cursor_index - prev_char.len_utf8());
        state.cursor_index -= prev_char.len_utf8();
        state.cursor_blink = 0.0;

        cx.request_layout();
    }

    fn input_enter(&self, state: &mut TextInputState, modifiers: Modifiers) {
        let text = self.text.get();

        if self.multiline && state.cursor_index < text.len() || modifiers.shift {
            return;
        }

        if let Some(ref on_submit) = self.on_submit {
            on_submit.emit(&text);
            state.focused = false;
        }
    }

    fn input_left(&self, state: &mut TextInputState, _cx: &EventContext<'_>) {
        if let Some(prev_char) = self.prev_char(state) {
            state.cursor_index -= prev_char.len_utf8();
            state.cursor_blink = 0.0;
        }
    }

    fn input_right(&self, state: &mut TextInputState, _cx: &EventContext<'_>) {
        if let Some(next_char) = self.next_char(state) {
            state.cursor_index += next_char.len_utf8();
            state.cursor_blink = 0.0;
        }
    }

    fn handle_keyboard_event(
        &self,
        state: &mut TextInputState,
        cx: &EventContext<'_>,
        event: &KeyboardEvent,
    ) -> bool {
        if !state.focused {
            return false;
        }

        if let Some(ref text) = event.text {
            self.input_text(state, cx, text);
            return true;
        }

        if let Some(key) = event.key {
            if event.is_press() {
                return self.input_key(state, cx, event.modifiers, key);
            }
        }

        true
    }

    fn find_glyph(&self, state: &TextInputState) -> Option<Glyph> {
        let glyphs = state.glyphs.as_ref()?;

        glyphs
            .iter()
            .find(|glyph| glyph.byte_offset == state.cursor_index)
            .copied()
    }

    fn cursor_position(&self, state: &TextInputState) -> Option<Vec2> {
        let glyph = self.find_glyph(state)?;

        Some(Vec2::new(
            glyph.rect.min.x,
            glyph.baseline - (glyph.line_ascent + glyph.line_descent) / 2.0,
        ))
    }
}

#[doc(hidden)]
#[derive(Default, Debug)]
pub struct TextInputState {
    glyphs: Option<Glyphs>,
    hovered: bool,
    focused: bool,
    cursor_blink: f32,
    cursor_index: usize,
    callback: Callback<'static>,
}

impl StateView for TextInput {
    type State = TextInputState;

    fn build(&mut self, cx: &mut Context<'_>) -> Self::State {
        let sink = cx.event_sink().clone();

        let callback = Callback::new(move |()| {
            sink.send(RequestLayoutEvent);
        });

        TextInputState {
            callback,
            ..Default::default()
        }
    }

    fn event(&mut self, state: &mut Self::State, cx: &mut EventContext<'_>, event: &Event) {
        if let Some(pointer_event) = event.get::<PointerEvent>() {
            if self.handle_pointer_event(state, cx, pointer_event) {
                event.handle();
            }
        }

        if let Some(keyboard_event) = event.get::<KeyboardEvent>() {
            if self.handle_keyboard_event(state, cx, keyboard_event) {
                event.handle();
            }
        }
    }

    fn layout(
        &mut self,
        state: &mut Self::State,
        cx: &mut LayoutContext<'_>,
        space: AvailableSpace,
    ) -> Vec2 {
        self.text.subscribe(&state.callback);

        let mut color = self.color.get(cx.theme);

        if self.is_placeholder() {
            color = color.brighten(0.3);
        }

        let content_space = self.size.content_space(cx, space);
        let section = TextSection {
            text: &self.text(),
            font_size: self.font_size.get(cx.theme).get(cx),
            font_family: self.font_family.get(cx.theme),
            font_weight: self.font_weight.get(cx.theme),
            font_stretch: self.font_stretch.get(cx.theme),
            font_style: self.font_style.get(cx.theme),
            color,
            v_align: self.v_align.get(cx.theme),
            h_align: self.h_align.get(cx.theme),
            line_height: self.line_height.get(cx.theme),
            wrap: self.wrap.get(cx.theme),
            bounds: content_space.max,
        };

        state.glyphs = cx.fonts.layout_glyphs(&section);
        let content_size = state.glyphs.as_ref().map_or(Vec2::ZERO, Glyphs::size);
        self.size.get(cx, content_size, space)
    }

    fn draw(&mut self, state: &mut Self::State, cx: &mut DrawContext<'_>) {
        let Some(ref glyphs) = state.glyphs else { return };

        cx.draw_text(glyphs, cx.rect());

        if !state.focused {
            return;
        }

        let offset = glyphs.offset(cx.rect());
        let cursor_center = match self.cursor_position(state) {
            Some(position) => position + offset,
            None => cx.rect().left_center(),
        };

        let font_size = self.font_size.get(cx.theme).get(cx);
        let cursor_size = Vec2::new(1.0, font_size);
        let cursor_min = cursor_center - cursor_size / 2.0;

        state.cursor_blink += cx.dt() * 10.0;
        cx.request_redraw();

        let mut color = self.color.get(cx.theme);
        color.a = state.cursor_blink.sin() * 0.5 + 0.5;

        cx.draw_rounded(Quad {
            rect: Rect::min_size(cursor_min.round(), cursor_size),
            background_color: color,
            ..Default::default()
        });
    }
}
