use crate::components::header::Navbar;

use leptos::prelude::*;

#[component]
pub fn Uefi() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <main>
                <Navbar />
                <div class="bg-gradient-to-tl from-indigo-500 to-indigo-500 text-white font-mono flex flex-col min-h-screen">
                    <div class="flex flex-row-reverse flex-wrap m-auto">
                        <script src="https://cdn.jsdelivr.net/npm/@webcomponents/webcomponentsjs@2/webcomponents-loader.min.js"></script>
                        <script type="module" src="https://cdn.jsdelivr.net/gh/zerodevx/zero-md@1/src/zero-md.min.js"></script>
                        <zero-md src="uefi.md"></zero-md>
                    </div>
                </div>
            </main>
        </ErrorBoundary>
    }
}
