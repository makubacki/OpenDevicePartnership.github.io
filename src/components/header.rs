use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full h-[160px] px-[120px] bg-white dark:bg-black flex items-center justify-between">
            <div class="flex items-center space-x-6">
                <picture>
                    <source srcset="/images/dark/odplogo.svg" media="(prefers-color-scheme: dark)" />
                    <img
                        src="/images/light/odplogo.svg"
                        alt="ODP Logo"
                        class="w-[149px] h-[51.43px] object-contain"
                    />
                </picture>
            </div>

            <nav class="flex [column-gap:25px]">
                <NavButton href="/getting-started" label="Getting Started"/>
                <NavButton href="/projects" label="Projects"/>
                <NavButton href="/library" label="Library"/>
                <NavButton href="/governance" label="Governance"/>
                <NavButton href="/home" label="Home"/>
            </nav>
        </header>
    }
}

#[component]
fn NavButton(href: &'static str, label: &'static str) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let is_active = move || location.pathname.get().starts_with(href);

    view! {
        <a
            href=href
            class=move || {
                let base = "odp-header-btn odp-header-btn-text";
                if is_active() {
                    format!("{base} odp-header-btn-active odp-header-btn-active-text")
                } else {
                    format!("{base} ")
                }
            }
        >
            {label}
        </a>
    }
}
