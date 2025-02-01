use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-container" style="height: 30px; position: relative;">
                <div style="float: left;">
                    <img src="/images/odplogo.png" style="width: 40px; height: 40px;" />
                </div>
                <div class="navbar-brand" style="float: left;">
                    <a class="navbar-title">
                        "Open Device Partnership"
                    </a>
                </div>
                <div class="navbar-menu {}" style="float: right;">
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