use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="container">
                <div>
                    <img src="/images/odplogo.png" />
                </div>
                <div class="navbar-brand">
                    <a class="navbar-title" href="/">
                        "Open Device Partnership"
                    </a>
                    <button class="navbar-burger" aria-label="menu" aria-expanded="true">
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>

                <div class={format!("navbar-menu {}", "is-active")}>
                    <div class="navbar-start">
                        <a class="navbar-item" href="/about">
                            "About"
                        </a>
                        <a class="navbar-item" href="/news-and-blogs">
                            "News & Blogs"
                        </a>
                        <a class="navbar-item" href="/contact">
                            "Contact"
                        </a>
                    </div>
                </div>
            </div>
        </nav>
    }
}