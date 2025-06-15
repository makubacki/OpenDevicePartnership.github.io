use leptos::prelude::*;
use crate::components::image_button::ImageButton;

#[component]
pub fn ProjectsComponent() -> impl IntoView {
    view! {
        <section
            style="
                padding: 120px;
                background: #fff;
            "
        >
            <div class="flex flex-row gap-[80px]">
                {/* Left Column */}
                <div class="flex flex-col items-start" style="width: 700px;">
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 140px;
                            font-style: normal;
                            font-weight: 400;
                            line-height: 95%;
                            letter-spacing: -2.8px;
                            margin-bottom: 32px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"System Firmware Domains"}
                    </span>
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 60px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 110%;
                            letter-spacing: -1.2px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Reusable foundations for secure, high-quality device platforms"}
                    </span>
                </div>
                {/* Right Column */}
                <div class="flex flex-col items-start" style="width: 600px;">
                    {/* WHAT label */}
                    <span
                        style="
                            color: #171717;
                            font-family: 'Geist Mono', monospace;
                            font-size: 20px;
                            font-style: normal;
                            font-weight: 600;
                            line-height: 130%;
                            letter-spacing: 0.4px;
                            text-transform: uppercase;
                            margin-bottom: 12px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 35px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 140%;
                            letter-spacing: -0.7px;
                            margin-bottom: 32px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"ODP supports development across three core areas of system firmware. Each domain is designed for modularity, security, and long-term reuse across hardware platforms."}
                    </span>
                    {/* WHY label */}
                    <span
                        style="
                            color: #171717;
                            font-family: 'Geist Mono', monospace;
                            font-size: 20px;
                            font-style: normal;
                            font-weight: 600;
                            line-height: 130%;
                            letter-spacing: 0.4px;
                            text-transform: uppercase;
                            margin-bottom: 12px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        style="
                            color: #171717;
                            font-family: Geist, sans-serif;
                            font-size: 35px;
                            font-style: normal;
                            font-weight: 500;
                            line-height: 140%;
                            letter-spacing: -0.7px;
                            margin-bottom: 32px;
                            display: block;
                            text-align: left;
                        "
                    >
                        {"Modern computing devices need modern solutions that prioritize memory safety and security from the language on up."}
                    </span>
                </div>
            </div>
        </section>

        // Projects Details Section
        <section
            style="
                padding: 120px;
                background: #fff;
            "
        >
            <div class="flex flex-col gap-[60px]">
                {/* Row 1 */}
                <div class="flex flex-row gap-[60px] items-center">
                    {/* Image Button */}
                    <div>
                        <ImageButton href="/boot-firmware" img_src="/images/patina.svg" alt="Boot Firmware" width=600 height=518 />
                    </div>
                    {/* Text Content */}
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 60px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 110%;
                                letter-spacing: -1.2px;
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Boot Firmware (Patina)"}
                        </span>
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 140%;
                                letter-spacing: -0.7px;
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based UEFI implementation"}
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
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Patina provides a modern UEFI firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure boot on client platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/boot-firmware"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the Boot Firmware Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/patina"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ View Patina on GitHub"}
                            </a>
                        </div>
                    </div>
                </div>
                {/* Row 2 */}
                <div class="flex flex-row gap-[60px] items-center">
                    <div>
                        <ImageButton href="/embedded-controller" img_src="/images/ec.svg" alt="Embedded Controller" width=600 height=518 />
                    </div>
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 60px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 110%;
                                letter-spacing: -1.2px;
                                margin-bottom: 18px;
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
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 140%;
                                letter-spacing: -0.7px;
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based EC implementation"}
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
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC provides a modern embedded controller firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure device management."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/embedded-controller"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the EC Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/ec"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ View EC on GitHub"}
                            </a>
                        </div>
                    </div>
                </div>
                {/* Row 3 */}
                <div class="flex flex-row gap-[60px] items-center">
                    <div>
                        <ImageButton href="/windows-ec-services" img_src="/images/ECServices.svg" alt="EC Services" width=600 height=518 />
                    </div>
                    <div class="flex flex-col items-start" style="flex: 1;">
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 60px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 110%;
                                letter-spacing: -1.2px;
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Windows EC Services"}
                        </span>
                        <span
                            style="
                                color: #171717;
                                font-family: Geist, sans-serif;
                                font-size: 35px;
                                font-style: normal;
                                font-weight: 500;
                                line-height: 140%;
                                letter-spacing: -0.7px;
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based EC services implementation"}
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
                                margin-bottom: 18px;
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC Services provides a modern EC services firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure EC services on Windows platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/windows-ec-services"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the EC Services Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/ecservices"
                                style="
                                    color: #171717;
                                    font-family: Geist, sans-serif;
                                    font-size: 25px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 130%;
                                    letter-spacing: -0.25px;
                                    text-decoration-line: underline;
                                    text-decoration-style: solid;
                                    text-decoration-skip-ink: auto;
                                    text-decoration-thickness: auto;
                                    text-underline-offset: auto;
                                    text-underline-position: from-font;
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ View EC Services on GitHub"}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
