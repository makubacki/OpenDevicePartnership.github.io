use crate::components::header::Header;
use crate::components::footer::Footer;
use crate::components::team_grid::{TeamMember, TeamGrid};

use leptos::prelude::*;

fn create_team() -> Vec<TeamMember> {

    vec![
        TeamMember {
            first_name: "Michael",
            last_name: "Kubacki",
            role: "Team leader",
            github_username: "makubacki",
            github_url: "https://github.com/makubacki",
            image_url: "https://github.com/makubacki.png?size=200",
        },
        TeamMember {
            first_name: "Ron",
            last_name: "Gurr",
            role: "Team leader",
            github_username: "rogurr",
            github_url: "https://github.com/rogurr",
            image_url: "https://github.com/rogurr.png?size=200",
        },
        TeamMember {
            first_name: "Kat",
            last_name: "Perez",
            role: "",
            github_username: "kat-perez",
            github_url: "https://github.com/kat-perez",
            image_url: "https://github.com/kat-perez.png?size=200",
        },
    ]
}

#[component]
pub fn TeamPatina() -> impl IntoView {

    let team = create_team();

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
                                {"Patina team"}
                            </span>
                            <span
                                class="p1"
                                style="
                                    display: block;
                                    text-align: left;
                                "
                            >
                                {"Developing and managing development of a new modern UEFI"}
                            </span>
                        </div>
                    </div>
                </section>
                <TeamGrid members=team />
                <Footer />
            </div>
            
        </ErrorBoundary>
    }
}