use leptos::prelude::*;

#[component]
pub fn GovernanceTeams() -> impl IntoView {
    view! {
        <section
            style="
                padding: 120px;
                background: #fff;
            "
        >
            {/* Row 1: Header */}
            <div style="margin-bottom: 56px;">
                <span
                    style="
                        color: #171717;
                        font-family: 'Geist Mono', monospace;
                        font-size: 60px;
                        font-style: normal;
                        font-weight: 400;
                        line-height: 110%;
                        letter-spacing: 4.8px;
                        text-transform: uppercase;
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
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
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
                <img
                    src="/images/Teams.svg"
                    alt="Teams"
                    style="
                        display: flex;
                        width: 150px;
                        height: 150px;
                        padding: 18.75px 6.25px;
                        justify-content: left;
                        align-items: left;
                        aspect-ratio: 1/1;
                        margin-bottom: 24px;
                    "
                />
                <span
                    style="
                        color: #171717;
                        font-family: Geist, sans-serif;
                        font-size: 35px;
                        font-style: normal;
                        font-weight: 600;
                        line-height: 120%;
                        letter-spacing: -0.7px;
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
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 35px;
                            font-style: normal;
                            font-weight: 600;
                            line-height: 120%;
                            letter-spacing: -0.7px;
                            margin-bottom: 12px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Patina"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
                            margin-bottom: 24px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Patina."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-patina"
                        style="
                            border-radius: 10px;
                            border: 2px solid #171717;
                            background: #FFF;
                            display: flex;
                            padding: 25px 35px;
                            justify-content: center;
                            align-items: center;
                            gap: 10px;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-weight: 500;
                            color: #171717;
                            cursor: pointer;
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 35px;
                            font-style: normal;
                            font-weight: 600;
                            line-height: 120%;
                            letter-spacing: -0.7px;
                            margin-bottom: 12px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Embedded Controller"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
                            margin-bottom: 24px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Embedded Controller."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="/team-ec"
                        style="
                            border-radius: 10px;
                            border: 2px solid #171717;
                            background: #FFF;
                            display: flex;
                            padding: 25px 35px;
                            justify-content: center;
                            align-items: center;
                            gap: 10px;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-weight: 500;
                            color: #171717;
                            cursor: pointer;
                            margin-top: auto;
                            text-decoration: none;
                        "
                    >
                        {"Members + Contacts"}
                    </a>
                </div>
                <div class="flex flex-col items-start h-full" style="width: 320px; min-height: 350px; justify-content: flex-start;">
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 35px;
                            font-style: normal;
                            font-weight: 600;
                            line-height: 120%;
                            letter-spacing: -0.7px;
                            margin-bottom: 12px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"EC Services"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 130%;
                            letter-spacing: -0.25px;
                            margin-bottom: 24px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Team in charge of Windows EC Services."}
                    </span>
                    <div style="flex: 1 1 auto;"></div>
                    <a
                        href="team-ec-services"
                        style="
                            border-radius: 10px;
                            border: 2px solid #171717;
                            background: #FFF;
                            display: flex;
                            padding: 25px 35px;
                            justify-content: center;
                            align-items: center;
                            gap: 10px;
                            font-family: Geist, sans-serif;
                            font-size: 25px;
                            font-weight: 500;
                            color: #171717;
                            cursor: pointer;
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