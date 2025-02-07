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
            <div>
                <div class="grid grid-cols-4 bg-white gap-10 h-24 pl-24 pr-24">
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/boot-firmware">
                        <p class="text-center font-bold p-4">Boot Firmware</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/embedded-controller">
                        <p class="text-center font-bold p-4">Embedded Controller Firmware</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/windows-ec-services">
                        <p class="text-center font-bold p-4">Windows-EC Services</p>
                    </a>
                    <a class="place-content-center bg-blue-200 rounded-md ring-4 ring-indigo-300" href="/mptf">
                        <p class="text-center font-bold p-4">Modern Power&Thermal Framework</p>
                    </a>
                </div>
            </div>
            <div class="bg-white h-2"> </div>
            <div class="bg-white">
                <div class="text-base px-28">
                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Introducing the Open Device Partnership Project</span></p>

                    <p class="pb-4 text-lg">ODP is a groundbreaking initiative designed to transform the Windows device ecosystem. By focusing on 
                    robust security, superior device fundamentals, and accelerated product development, ODP sets a new standard for security, 
                    performance, battery life, and reliability. </p>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Key Components</span></p>
                    <ul class="pb-4 text-lg">
                        <li class="list-disc list-inside"><i>Fast Boot Firmware</i>: ensures a secure and efficient boot process for Windows devices</li>
                        <li class="list-disc list-inside"><i>Secure Embedded Controller (EC)</i>: standardizes and secures the embeddedcontroller firmware across x86 and ARM</li>
                        <li class="list-disc list-inside"><i>Modern Power & Thermal Framework (MPTF)</i>: addresses performance and thermal management challenges to improve battery life across x86 and ARM</li>
                    </ul>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Value Proposition</span></p>
                    <ul class="pb-4 text-lg">
                        <li class="list-disc list-inside"><i>Enhanced Security</i>: secure boot process and hardened firmware, protecting devices from vulnerabilities</li>
                        <li class="list-disc list-inside"><i>Improved Performance</i>: optimizes performance and thermal management, MPTF enables top notch fundamentals</li>
                        <li class="list-disc list-inside"><i>Accelerated Development</i>: open-source collaboration with partners, enables faster and more efficient product development</li>
                    </ul> 

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Partner-Oriented Vision</span></p>
                    <p class="pb-4 text-lg">ODP is built on a foundation of collaboration and innovation. We are committed to working closely with our ecosystem partners,
                    including OEMs, IHVs, IBVs, and developers, to build a secure and hardened Windows device with solid fundamentals. We invite partners to 
                    contribute to and benefit from a marketplace of options.</p>

                    <p class="pb-4"><span class="font-bold text-xl whitespace-normal">Get Involved</span></p>
                    <p class="pb-4 text-lg">Our goal is to create a foundation that not only meets but exceeds current state of the art, driving innovation and raising 
                    the bar for Windows devices. For more information about the ODP project and partnership opportunities, please consult the documentation and 
                    issue your first pull request.</p>
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
