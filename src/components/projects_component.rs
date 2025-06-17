use leptos::prelude::*;
use crate::components::image_button::ImageButton;

#[component]
pub fn ProjectsComponent() -> impl IntoView {
    view! {
        <section
            class="background_primary"
            style="
                padding: 120px;
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
                        {"System Firmware Domains"}
                    </span>
                    <span
                        class="h2"
                        style="
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
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHAT"}
                    </span>
                    {/* WHAT description */}
                    <span
                        class="p1"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"ODP supports development across three core areas of system firmware. Each domain is designed for modularity, security, and long-term reuse across hardware platforms."}
                    </span>
                    {/* WHY label */}
                    <span
                        class="mono"
                        style="
                            display: block;
                            text-align: left;
                        "
                    >
                        {"WHY"}
                    </span>
                    {/* WHY description */}
                    <span
                        class="p1"
                        style="
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
            class="background_primary"
            style="
                padding: 120px;
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
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Boot Firmware (Patina)"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based UEFI implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Patina provides a modern UEFI firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure boot on client platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/boot-firmware"
                                class="link"
                                style="
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the Boot Firmware Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/patina"
                                class="link"
                                style="
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
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Embedded Controller"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based EC implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC provides a modern embedded controller firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure device management."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/embedded-controller"
                                class="link"
                                style="
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the EC Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/ec"
                                class="link"
                                style="
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
                            class="h2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"Windows EC Services"}
                        </span>
                        <span
                            class="p1"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"A secure Rust-based EC services implementation"}
                        </span>
                        <span
                            class="p2"
                            style="
                                display: block;
                                text-align: left;
                            "
                        >
                            {"ODP EC Services provides a modern EC services firmware written in Rust. Designed for safety and composability, it serves as a foundational layer for secure EC services on Windows platforms."}
                        </span>
                        <div class="flex flex-col gap-[8px]">
                            <a
                                href="https://docs.odp.example.com/windows-ec-services"
                                class="link"
                                style="
                                    display: block;
                                "
                                target="_blank"
                            >
                                {"→ Read the EC Services Guide"}
                            </a>
                            <a
                                href="https://github.com/microsoft/ecservices"
                                class="link"
                                style="
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
