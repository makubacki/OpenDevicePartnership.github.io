use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="w-full h-[160px] px-[120px] bg-white flex items-center justify-between">
            <div class="flex items-center space-x-6">
                <img
                    src="/images/odplogo.svg"
                    alt="ODP Logo"
                    class="w-[149px] h-[51.43px] object-contain"
                />

            </div>

            <nav class="flex [column-gap:25px]">
                <NavButton href="/getting-started" label="Getting Started"/>
                <NavButton href="/projects" label="Projects"/>
                <NavButton href="/library" label="Library"/>
                <NavButton href="/governance" label="Governance"/>
                <NavButton href="/non-technical" label="Non-Technical"/>
                <NavButton href="/team" label="Team"/>
            </nav>
        </header>
    }
}

#[component]
fn NavButton(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <a
            href=href
            class="px-[24px] py-[19px] h-[62px] flex items-center justify-center bg-white 
            border-2 border-black rounded-[10px] opacity-100 
            hover:bg-gray-100 transition 
            font-geist text-[20px] font-semibold not-italic"
        >
            {label}
        </a>
    }
}