use crate::components::header::Header;
use crate::components::footer::Footer;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn TeamECServices() -> impl IntoView {
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
        
            <div class="w-full min-h-screen overflow-x-hidden">
                <Header />
                <Footer />
            </div>
            
        </ErrorBoundary>
    }
}
