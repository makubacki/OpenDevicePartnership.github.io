use crate::components::welcome::Welcome;

use leptos::prelude::*;

#[component]
pub fn Main() -> impl IntoView {
    view! {
        <main>
            <Welcome />
        </main>
    }
}
