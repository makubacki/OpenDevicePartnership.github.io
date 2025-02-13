use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed w-screen custom-bg-white shadow-md z-50">
            <div class="flex flex-col md:flex-row justify-between items-center p-4 border-b-4 border-gray-300">
                <a class="flex items-center mb-4 md:mb-0" href="/">
                    <img src="/images/odplogo.png" class="h-14 w-14" alt="ODP Logo" />
                    <p class="text-2xl md:text-3xl custom-text-gray-800 pl-4">Open Device Partnership</p>
                </a>
                <div class="flex-grow md:hidden"></div>
                <div class="flex space-x-8 ml-4">
                    <a href="/about" class="custom-text-gray-800 hover:custom-text-blue-600 transition duration-300">
                        About
                    </a>
                    <a href="/documentation" class="custom-text-gray-800 hover:custom-text-blue-600 transition duration-300">
                        Documentation
                    </a>
                    <a href="https://github.com/OpenDevicePartnership" class="custom-text-gray-800 hover:custom-text-blue-600 transition duration-300">
                        Repositories
                    </a>
                    <a href="/contact" class="custom-text-gray-800 hover:custom-text-blue-600 transition duration-300">
                        Contact
                    </a>
                </div>
            </div>
        </nav>
    }
}