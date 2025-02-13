use crate::components::welcome::Welcome;
use leptos::prelude::*;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main class="custom-bg-white custom-text-gray-800">
            <Welcome />
        </main>
    }
}