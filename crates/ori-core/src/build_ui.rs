use ori_reactive::Scope;

use crate::{IntoView, View};

pub type BoxedBuildUi = Box<dyn FnMut(Scope) -> View + Send + 'static>;

/// Trait for types that can build a UI.
///
/// This is implemented for `impl FnMut(Scope) -> impl IntoView` by default.
pub trait BuildUi<I = View>: Send + 'static {
    /// Builds the UI, in the `scope` provided.
    fn ui(&mut self, scope: Scope) -> View;

    /// Converts this [`BuildUi`] into a [`BoxedBuildUi`].
    fn boxed(mut self) -> BoxedBuildUi
    where
        Self: Sized + 'static,
    {
        Box::new(move |cx| self.ui(cx))
    }
}

impl<F, I: IntoView> BuildUi<I> for F
where
    F: FnMut(Scope) -> I + Send + 'static,
{
    fn ui(&mut self, scope: Scope) -> View {
        self(scope).into_view()
    }
}
