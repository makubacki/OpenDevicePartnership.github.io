use leptos::prelude::*;

#[component]
pub fn GovernanceTeams() -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px;
            "
        >
            {/* Row 1: Header */}
            <div style="margin-bottom: 56px;">
                <span
                    class="p_mono"
                    style="
                        display: block;
                    "
                >
                    {"GOVERNANCE"}
                </span>
            </div>

            {/* Row 2: Two Columns */}
            <div class="flex flex-row gap-[60px]" style="margin-bottom: 80px;">
                <div style="width: 950px;">
                    <span class="h1" style="display: block; text-align: left;">
                        {"How ODP is built by its community"}
                    </span>
                </div>
                <div class="flex flex-col justify-start" style="flex: 1; min-width: 600px; max-width: 900px;">
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod  tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim  veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea  commodo consequat. Duis aute irure dolor in reprehenderit in voluptate  velit esse cillum dolore eu fugiat nulla pariatur."}
                    </span>
                </div>
            </div>

            {/* Row 3: Teams Image and Label */}
            <div class="flex flex-col items-left" style="margin-bottom: 80px;">
                <picture>
                    <source srcset="/images/dark/Teams.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/Teams.svg"
                        alt="Teams"
                        class="icon"
                    />
                </picture>
                <span
                    class="h2"
                    style="
                        display: flex;
                        justify-content: left;
                        align-items: left;
                    "
                >
                    {"Teams"}
                </span>
            </div>

            {/* Row 4: Teams */}
            <div class="flex flex-row items-stretch" style="gap: 123px;">
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Patina"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Patina."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-patina"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Embedded Controller"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Embedded Controller."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-ec"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        class="h3"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"EC Services"}
                    </span>
                    <span
                        class="p2"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Windows EC Services."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="team-ec-services"
                        class="odp-btn odp-btn-text"
                        style="
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
            </div>
        </section>
    }
}