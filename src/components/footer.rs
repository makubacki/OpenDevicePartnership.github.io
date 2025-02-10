use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 bg-gray-100 h-32">
            <a class="place-content-center" href="/about">
                <p class="text-center font-bold">"About"</p>
            </a>
            <a class="place-content-center" href="/contact">
                <p class="text-center font-bold">"Contact Us"</p>
            </a>
        </div>
    }
}
