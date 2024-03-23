// pub use super::common::Size;

use leptos::{component, view, IntoView};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Year,
    Month,
    Date,
    #[default]
    DateTime,
    Week,
}

#[component]
pub fn DateTimePicker() -> impl IntoView {
    view! {
        <div></div>
    }
}
