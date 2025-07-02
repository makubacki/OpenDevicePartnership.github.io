use crate::components::header::Header;
use crate::components::footer::Footer;

use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn TeamEC() -> impl IntoView {
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
                <a
                    href="javascript:history.back()"
                    class="block"
                    style="margin: 0; padding: 0;"
                >
                    <picture>
                        <source srcset="/images/dark/backbutton.svg" media="(prefers-color-scheme: dark)" />
                        <img
                            src="/images/light/backbutton.svg"
                            alt="Back"
                            style="margin: 0; padding: 0; display: block;"
                        />
                    </picture>
                </a>
                <section
                    class="background_primary"
                    style="
                        padding: 120px;
                        margin-top: -80px;
                    "
                >
                    <div class="flex flex-row gap-[80px]">
                        {/* Left Column */}
                        <div class="flex flex-col items-start" style="width: 700px;">
                            <span
                                class="h1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Meet the team"}
                            </span>
                        </div>
                        {/* Right Column */}
                        <div class="flex flex-col items-start" style="width: 600px;">
                            {/* Team Introduction */}
                            <span
                                class="mono"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"The Secure EC team ..."}
                            </span>
                        </div>
                    </div>
                </section>
                <Footer />
            </div>
            
        </ErrorBoundary>
    }
}
