use std::fmt::Display;

use ori::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
struct Number {
    value: f64,
    position: Option<i8>,
}

impl Number {
    fn new(value: f64) -> Self {
        Self {
            value,
            position: None,
        }
    }

    fn add_digit(&mut self, digit: u8) {
        let Some(position) = self.position else {
            self.position = Some(1);
            self.value = digit as f64;
            return;
        };

        let sign = self.value.signum();

        if position < 0 {
            let pow = 10.0f64.powi(position as i32);
            self.value += digit as f64 * pow * sign;
            self.position = Some(position - 1);
        } else {
            self.value *= 10.0;
            self.value += digit as f64 * sign;
            self.position = Some(position + 1);
        }
    }

    fn remove_digit(&mut self) {
        let Some(position) = self.position else {
            self.position = Some(0);
            self.value = 0.0;
            return;
        };

        if position < -1 {
            self.value *= 10.0f64.powi(-position as i32 - 2);
            self.value = self.value.trunc();
            self.value /= 10.0f64.powi(-position as i32 - 2);

            if position == -2 {
                self.position = Some(0);
            } else {
                self.position = Some(position + 1);
            }
        } else if position >= 0 {
            self.value /= 10.0;
            self.value = self.value.trunc();

            self.position = Some((position - 1).max(0));
        } else {
            self.position = Some(0);
        }

        // ensure that -0.0 is not displayed
        if self.value == -0.0 {
            self.value = 0.0;
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(position) = self.position else {
            return write!(f, "{}", self.value);
        };

        if position == -1 {
            write!(f, "{}.", self.value)
        } else if position < 0 {
            write!(f, "{:.1$}", self.value, -position as usize - 1)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    None,
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn result_bar(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let text = cx.memo(move || {
        let result = result.get();
        let operator = operator.get();
        let rhs = rhs.get();

        match operator {
            Operator::None => format!("{}", result),
            Operator::Add => format!("{} + {}", result, rhs),
            Operator::Subtract => format!("{} - {}", result, rhs),
            Operator::Multiply => format!("{} × {}", result, rhs),
            Operator::Divide => format!("{} ÷ {}", result, rhs),
        }
    });

    view! {
        <Div class="result-bar">
            <Text class="result" text=text.get() />
        </Div>
    }
}

fn bar0(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let clear_all = move |_: &PointerEvent| {
        operator.set(Operator::None);
        result.set(Number::new(0.0));
        rhs.set(Number::new(0.0));
    };

    let clear = move |_: &PointerEvent| {
        if matches!(operator.get(), Operator::None) {
            result.set(Number::new(0.0));
        } else {
            rhs.set(Number::new(0.0));
        }
    };

    let remove_digit = move |_: &PointerEvent| {
        if matches!(operator.get(), Operator::None) {
            result.modify().remove_digit();
        } else {
            rhs.modify().remove_digit();
        }
    };

    let divide = move |_: &PointerEvent| {
        operator.set(Operator::Divide);
    };

    view! {
        <Div class="buttons row">
            <Button on:click=clear_all>"CE"</Button>
            <Button on:click=clear>"C"</Button>
            <Button on:click=remove_digit>
                <Text text="\u{e14a}" style:font-family="Material Icons" />
            </Button>
            <Button on:click=divide>"÷"</Button>
        </Div>
    }
}

fn add_digit(
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
    digit: u8,
) -> impl Fn(&PointerEvent) {
    move |_| {
        if matches!(operator.get(), Operator::None) {
            result.modify().add_digit(digit);
        } else {
            rhs.modify().add_digit(digit);
        }
    }
}

fn bar1(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let multiply = move |_: &PointerEvent| {
        operator.set(Operator::Multiply);
    };

    view! {
        <Div class="buttons row">
            <Button class="number" on:click=add_digit(operator, result, rhs, 7)>"7"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 8)>"8"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 9)>"9"</Button>
            <Button on:click=multiply>"×"</Button>
        </Div>
    }
}

fn bar2(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let subtract = move |_: &PointerEvent| {
        operator.set(Operator::Subtract);
    };

    view! {
        <Div class="buttons row">
            <Button class="number" on:click=add_digit(operator, result, rhs, 4)>"4"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 5)>"5"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 6)>"6"</Button>
            <Button on:click=subtract>"-"</Button>
        </Div>
    }
}

fn bar3(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let add = move |_: &PointerEvent| {
        operator.set(Operator::Add);
    };

    view! {
        <Div class="buttons row">
            <Button class="number" on:click=add_digit(operator, result, rhs, 1)>"1"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 2)>"2"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 3)>"3"</Button>
            <Button on:click=add>"+"</Button>
        </Div>
    }
}

fn bar4(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    let negate = move |_: &PointerEvent| {
        if result.get().value == 0.0 {
            return;
        }

        if matches!(operator.get(), Operator::None) {
            result.modify().value *= -1.0;
        } else {
            rhs.modify().value *= -1.0;
        }
    };

    let add_point = move |_: &PointerEvent| {
        if let Some(position) = result.get().position {
            if position < 0 {
                return;
            }
        }

        if matches!(operator.get(), Operator::None) {
            result.modify().position = Some(-1);
        } else {
            rhs.modify().position = Some(-1);
        }
    };

    let equals = move |_: &PointerEvent| {
        let mut result = result.modify();
        let mut rhs = rhs.modify();
        let mut operator = operator.modify();
        match *operator {
            Operator::None => {}
            Operator::Add => {
                *result = Number::new(result.value + rhs.value);
            }
            Operator::Subtract => {
                *result = Number::new(result.value - rhs.value);
            }
            Operator::Multiply => {
                *result = Number::new(result.value * rhs.value);
            }
            Operator::Divide => {
                *result = Number::new(result.value / rhs.value);
            }
        }
        *operator = Operator::None;
        *rhs = Number::new(0.0);
    };

    view! {
        <Div class="buttons row">
            <Button on:click=negate>"±"</Button>
            <Button class="number" on:click=add_digit(operator, result, rhs, 0)>"0"</Button>
            <Button on:click=add_point>"."</Button>
            <Button on:click=equals>"="</Button>
        </Div>
    }
}

fn buttons(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) -> View {
    view! {
        <Div class="buttons column">
            { bar0(cx, operator, result, rhs) }
            { bar1(cx, operator, result, rhs) }
            { bar2(cx, operator, result, rhs) }
            { bar3(cx, operator, result, rhs) }
            { bar4(cx, operator, result, rhs) }
        </Div>
    }
}

fn handle_keyboard_event(
    cx: Scope,
    operator: Signal<Operator>,
    result: Signal<Number>,
    rhs: Signal<Number>,
) {
    cx.on(move |event: &KeyboardEvent| {
        if !event.is_press() {
            return;
        }

        if event.is_pressed(Key::Escape) {
            operator.set(Operator::None);
            result.set(Number::new(0.0));
            rhs.set(Number::new(0.0));
        }

        if event.is_pressed(Key::Backspace) {
            if matches!(operator.get(), Operator::None) {
                result.modify().remove_digit();
            } else {
                rhs.modify().remove_digit();
            }
        }

        if event.is_pressed(Key::Enter) {
            let mut result = result.modify();
            let mut rhs = rhs.modify();
            let mut operator = operator.modify();
            match *operator {
                Operator::None => {}
                Operator::Add => {
                    *result = Number::new(result.value + rhs.value);
                }
                Operator::Subtract => {
                    *result = Number::new(result.value - rhs.value);
                }
                Operator::Multiply => {
                    *result = Number::new(result.value * rhs.value);
                }
                Operator::Divide => {
                    *result = Number::new(result.value / rhs.value);
                }
            }
            *operator = Operator::None;
            *rhs = Number::new(0.0);
        }

        if event.is_pressed(Key::Plus) {
            operator.set(Operator::Add);
        }

        if event.is_pressed(Key::Minus) {
            operator.set(Operator::Subtract);
        }

        if event.is_pressed(Key::Asterisk) {
            operator.set(Operator::Multiply);
        }

        if event.is_pressed(Key::Slash) {
            operator.set(Operator::Divide);
        }

        if event.is_pressed(Key::Period) {
            if let Some(position) = result.get().position {
                if position < 0 {
                    return;
                }
            }

            if matches!(operator.get(), Operator::None) {
                result.modify().position = Some(-1);
            } else {
                rhs.modify().position = Some(-1);
            }
        }

        if let Some(digit) = event.key.and_then(Key::as_digit) {
            if matches!(operator.get(), Operator::None) {
                result.modify().add_digit(digit);
            } else {
                rhs.modify().add_digit(digit);
            }
        }
    });
}

fn ui(cx: Scope) -> View {
    let operator = cx.signal(Operator::None);
    let result = cx.signal(Number::new(0.0));
    let rhs = cx.signal(Number::new(0.0));

    handle_keyboard_event(cx, operator, result, rhs);

    view! {
        { result_bar(cx, operator, result, rhs) }
        { buttons(cx, operator, result, rhs) }
    }
}

fn main() {
    App::new(ui)
        .title("Calculator (examples/calculator.rs)")
        .style(style!("examples/style/calculator.css"))
        .resizable(false)
        .transparent()
        .size(300, 400)
        .run();
}
