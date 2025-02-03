use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div class="pt-16 grid grid-cols-1 gap-4 bg-white">
            <div class="grid grid-cols-2 bg-gray-100">
                <div class="place-content-center ...">
                    <p class="text-4xl place-self-center p-20 ...">Enhancing device security, strengthening fundamentals, and accelerating the delivery of high-quality devices</p>
                </div>
                <div>
                    <img src="/images/laptop.jpg" class="p-9" />
                </div>
            </div>
            <div class="bg-white h-2"> </div>
            <div>
                <div class="grid grid-cols-4 bg-white gap-10 h-24 pl-24 pr-24">
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/boot-firmware">
                        <p class="text-center font-bold p-4">Boot Firmware</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/embedded-controller">
                        <p class="text-center font-bold p-4">Embedded Controller</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/windows-ec-services">
                        <p class="text-center font-bold p-4">Windows-EC Services</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/mptf">
                        <p class="text-center font-bold p-4">Modern Power&Thermal</p>
                    </a>
                </div>
            </div>
            <div class="bg-white h-2"> </div>
            <div class="bg-white">
                <div class="text-base px-28">
                    <p class="font-bold pb-4">Introducing the Open Device Partnership (ODP) Project</p>

                    <p class="pb-4">The ODP project is a groundbreaking initiative designed to transform the Windows device 
                    ecosystem. By focusing on robust security, superior device fundamentals, and accelerated 
                    product development, the ODP project sets a new standard for security, performance, battery life, 
                    and reliability.</p>


                    <p class="font-bold pb-4">Key Components</p>
                    <ul class="pb-4">
                        <li>* Fast Boot Firmware - Ensures a secure and efficient boot process for Windows devices.</li>
                        <li>* Secure Embedded Controller (EC) - Standardizes and secures the embedded controller firmware across the Windows ecosystem.</li>
                        <li>* Modern Power & Thermal Framework (MPTF) - Addresses performance and thermal management to improve battery life.</li>
                    </ul>

                    <p class="font-bold pb-4">Value Proposition</p>

                    <ul class="pb-4">
                        <li>* Enhanced Security - ensures a secure boot process and hardened EC firmware, protecting devices from vulnerabilities.</li>
                        <li>* Improved Performance - optimizing performance and thermal management, MPTF addresses inconsistent battery life</li>
                        <li>* Accelerated Development - opensource collaboration with partners, enabling faster and more efficient product development.</li>
                    </ul> 

                    <p class="font-bold pb-4">Partner-Oriented Vision</p>

                    <p class="pb-4">ODP project is built on a foundation of collaboration and innovation. We are committed to working closely with our ecosystem partners, including OEMs, silicon providers, 
                    and developers, to build a secure and innovative Windows platform. We invite partners to contribute to and benefit from a marketplace of options for building secure CoPilot+ PCs with strong fundamentals.</p>

                    <p class="font-bold pb-4">Get Involved</p>
                    
                    <p class="pb-4">Our goal is to create a foundation that not only meets but exceeds market needs, driving innovation and raising the bar for Windows devices.
                    For more information about the ODP project and partnership opportunities, please [TBD]</p>
                </div>
            </div>
            <div>
                <div class="grid grid-cols-3 bg-gray-100 h-32">
                    <a class="place-content-center" href="/news-and-blogs">
                        <p class="text-center font-bold">What is New</p>
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
