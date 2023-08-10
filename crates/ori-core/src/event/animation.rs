use ori_reactive::WeakCallback;

#[derive(Clone, Debug)]
pub struct RequestAnimationFrame {
    callback: WeakCallback,
}

impl RequestAnimationFrame {
    pub fn new(callback: WeakCallback) -> Self {
        Self { callback }
    }

    pub fn callback(&self) -> &WeakCallback {
        &self.callback
    }
}
