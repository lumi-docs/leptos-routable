// Crate aliases for mock feature - allows same code to work with both
// Re-exported for macro-generated code that uses ::leptos_routable::leptos:: paths
#[cfg(feature = "mock")]
pub extern crate leptos_mock as leptos;
#[cfg(feature = "mock")]
pub extern crate leptos_router_mock as leptos_router;

#[cfg(not(feature = "mock"))]
pub extern crate leptos;
#[cfg(not(feature = "mock"))]
pub extern crate leptos_router;

use std::fmt::Debug;
mod combine_paths;
mod maybe_param;

pub trait Routable {
    fn routes() -> impl leptos::IntoView;

    fn flat_routes() -> impl leptos::IntoView;

    fn fallback() -> impl leptos::IntoView;

    fn parent_route<Path, View>(
        path: Path,
        view: View,
        ssr: leptos_router::SsrMode,
    ) -> impl leptos_router::MatchNestedRoutes + Clone
    where
        Path: Send + Sync + 'static + Clone + Debug + leptos_router::PossibleRouteMatch,
        View: leptos_router::ChooseView;

    fn protected_parent_route<Path, View, ViewFn, ConditionFn, RedirectPathFn, RedirectPath>(
        path: Path,
        view: ViewFn,
        condition: ConditionFn,
        fallback: leptos::children::ViewFn,
        redirect_path: RedirectPathFn,
        ssr: leptos_router::SsrMode,
    ) -> impl leptos_router::MatchNestedRoutes + Clone
    where
        Path: Send + Sync + 'static + Clone + Debug + leptos_router::PossibleRouteMatch,
        ViewFn: Fn() -> View + Send + Clone + 'static,
        View: leptos::IntoView + 'static,
        ConditionFn: Fn() -> Option<bool> + Send + Clone + 'static,
        RedirectPathFn: Fn() -> RedirectPath + Send + Clone + 'static,
        RedirectPath: ::std::fmt::Display + 'static;
}

pub mod prelude {
    pub use super::combine_paths::combine_paths;
    pub use super::Routable;
    pub use crate::maybe_param::*;
    pub use leptos_routable_macro::*;
}
