use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed w-screen">
            <div class="grid grid-cols-2 bg-white gap-10 h-22 pt-3 pb-3">
                <a class="flex flex-row place-content-left pl-2" href="/">
                    <div class="place-content-center">
                        <img src="/images/odplogo.jpg" class="h-14 w-14" />
                    </div>
                    <div class="place-content-center">
                        <p class="text-3xl text-left pl-4">Open Device Partnership</p>
                    </div>
                </a>
                <div class="flex flex-row place-content-center">
                    <a href="/about">
                        <p class="text-center p-4">About</p>
                    </a>
                    <a href="/news-and-blogs">
                        <p class="text-center p-4">News & Blogs</p>
                    </a>
                    <a href="/contact">
                        <p class="text-center p-4">Contact</p>
                    </a>
                </div>
            </div>
        </nav>
    }
}