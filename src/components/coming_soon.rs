use leptos::prelude::*;

/// A simple Coming Soon component
#[component]
pub fn ComingSoon() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
            <div class="flex flex-row-reverse flex-wrap m-auto">
                <h1 class="text-9xl font-bold font-sans">Coming soon...</h1>
            </div>
        </div>
    }
}
