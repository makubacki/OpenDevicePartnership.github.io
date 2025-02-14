use leptos::prelude::*;

#[component]
pub fn IntroMessage() -> impl IntoView {
    view! {
        <div class="text-base px-4 sm:px-8 md:px-16 lg:px-24 xl:px-32 py-8 custom-bg-gray-100 custom-text-gray-800">
            <p class="pb-4">
                <span class="font-bold text-2xl custom-text-gray-400">
                    "Introducing the Open Device Partnership (ODP)"
                </span>
            </p>

            <p class="pb-4 text-lg">
                "ODP is an open initiative aimed at making it easier to build secure Windows devices with solid
                fundamentals. Integral to this vision is the idea of leveraging industry standards to build system software that spans 
                the full range of silicon options available and to simplify and accelerate development of high-quality devices. ODP strives 
                to set new standards for security, performance, battery life, and reliability while maximizing code re-use."
            </p>

            <p class="pb-4">
                <span class="font-bold text-2xl custom-text-gray-400">ODP Projects</span>
            </p>
            <ul class="pb-4 text-lg list-disc list-inside">
                <li>
                    <b class="">Fast and Minimal Boot Firmware</b>
                    " - A secure and efficient boot firmware for Windows devices"
                </li>
                <li>
                    <b class="">Hardened Embedded Controller Firmware</b>
                    " - An extensible, MCU-agnostic, secure embedded controller firmware"
                </li>
                <li>
                    <b class="">Standardized Embedded Controller Services</b>
                    " - A common method for interfacing embedded controller services with Windows"
                </li>
            </ul>
            <p class="pb-4 text-lg">
                "While we have ambitious goals for these first projects, we are also interested in
                adding new projects to the partnership that align with our shared goals. More information about partnership governance 
                and how to start new projects will be provided shortly. In the meantime, please reach out to "
                <a href="mailto:odpadmin@microsoft.com" class="underline custom-text-blue-600">
                    ODP Administrators
                </a>
                " with questions or comments."
            </p>

            <p class="pb-4">
                <span class="font-bold text-2xl custom-text-gray-400">Value Proposition</span>
            </p>
            <ul class="pb-4 text-lg list-disc list-inside">
                <li class="pl-4">
                    <b class="">Enhanced Security</b>
                    " - As security threats continue to evolve, it is critical we take
                    bold steps to protect devices from vulnerabilities by reducing the attack surface area, using secure hardware features, 
                    using modern programming languages to reduce human error-induced problems, and generally thinking about security first 
                    in every piece of code we design and author."
                </li>
                <li class="pl-4">
                    <b class="">Standardization</b>
                    " - While it is critical for device partners to differentiate in features and
                    capabilities, unfortunately a large fraction of device software is often simply the infrastructure and plumbing 
                    necessary to pull everything together. Developing and maintaining this software is a development tax to be paid 
                    and worse, often paid for each device and each architecture (x86 & ARM). Developing and building on top of industry 
                    standards offers one option to simplify and maximize re-use and it is a centerpiece of the ODP strategy."
                </li>
                <li class="pl-4">
                    <b class="">Accelerated Development</b>
                    " - ODP is built on open-source collaboration with partners,
                    sharing common goals and solutions, thus enabling faster and more efficient product development."
                </li>
            </ul>

            <p class="pb-4">
                <span class="font-bold text-2xl custom-text-gray-400">Partner-Oriented Vision</span>
            </p>
            <p class="pb-4 text-lg">
                "We are committed to creating an inclusive ecosystem of partners, including device OEM/ODMs,
                IHVs, Silicon Vendors, independent developers, security researchers, and anyone interested in helping to build secure 
                and high-quality devices."
            </p>

            <p class="pb-4">
                <span class="font-bold text-2xl custom-text-gray-400">Get Involved</span>
            </p>
            <p class="pb-4 text-lg">
                "For more information about ODP and partnership opportunities, please consult the documentation,
                clone source code from our growing list of public repositories and issue your first pull request!"
            </p>
        </div>
    }
}
