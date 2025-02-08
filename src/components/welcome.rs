use crate::components::subprojects::SubProjects;

use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div class="pt-16 grid grid-cols-1 gap-4 bg-white">
            <div class="grid grid-cols-2 bg-gray-100">
                <div class="place-content-center ...">
                    <p class="text-4xl place-self-center p-20">Enhancing device security, strengthening fundamentals, and accelerating the delivery of high quality devices</p>
                </div>
                <div>
                    <img src="/images/laptop.jpg" class="p-9" />
                </div>
            </div>
            <div class="bg-white h-2"> </div>
            <SubProjects />
            <div class="bg-white h-2"> </div>
            <div class="bg-white">
                <div class="text-base px-28">
                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Introducing the Open Device Partnership (ODP)</span></p>

                    <p class="pb-4 text-lg">ODP is an initiative designed to transform device fundamentals in the Windows ecosystem. By providing 
                    high quality and easy to reuse components, ODP strives to set new standards for security, performance, battery life, and reliability.</p>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">ODP Projects</span></p>
                    <ul class="pb-4 text-lg">
                        <li class="list-disc list-inside"><i>Fast and Minimal Boot Firmware</i> - a secure and efficient boot firmware for Windows devices</li>
                        <li class="list-disc list-inside"><i>Hardened Embedded Controller (EC)</i> - standardizes and secures the embedded controller firmware across the Windows ecosystem</li>
                        <li class="list-disc list-inside"><i>Modern Power & Thermal Framework (MPTF)</i> - a silicon agnostic performance and thermal management framework to deliver superior battery life</li>
                    </ul>
                    <p class="pb-4"><span class="text-lg">We are also interested in adding new projects to the partnership that align with our shared goals (consult our governance documentation for more information).</span></p>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Value Proposition</span></p>
                    <ul class="pb-4 text-lg">
                        <li class="list-disc list-inside"><i>Enhanced Security</i> - secures the boot process and hardened EC firmware, protecting devices from vulnerabilities</li>
                        <li class="list-disc list-inside"><i>Improved Performance</i> - optimizing performance and thermal management, MPTF addresses inconsistent battery life</li>
                        <li class="list-disc list-inside"><i>Accelerated Development</i> - open-source collaboration with partners, enabling faster and more efficient product development</li>
                        <li class="list-disc list-inside"><i>Standardized Solutions</i> - designed to work across different architectures (x86 & Arm) and operating systems</li>
                    </ul> 

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Partner-Oriented Vision</span></p>
                    <p class="pb-4 text-lg">ODP is built on a foundation of collaboration and innovation. We are committed to creating an ecosystem of partners, 
                    including OEMs, silicon providers, and independent developers, to build secure and high-quality devices.</p>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Get Involved</span></p>
                    <p class="pb-4 text-lg">Our goal is to create a foundation that not only meets but exceeds current state of the art, driving innovation and 
                    raising the bar for Windows devices. We are also open to collaborating with partners building other categories of devices. For more information 
                    about the ODP project and partnership opportunities, please consult the documentation and issue your first pull request.</p>
                </div>
            </div>
            <div>
                <div class="grid grid-cols-3 bg-gray-100 h-32">
                    <a class="place-content-center" href="/news-and-blogs">
                        <p class="text-center font-bold">What is New?</p>
                    </a>
                    <a class="place-content-center" href="/about">
                        <p class="text-center font-bold">About</p>
                    </a>
                    <a class="place-content-center" href="/contact">
                        <p class="text-center font-bold">Contact Us</p>
                    </a>
                </div>
            </div>
        </div>
    }
}
