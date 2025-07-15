use leptos::prelude::*;

#[component]
pub fn ImageButton(
    #[prop(into)] href: String,
    #[prop(into)] img_src: String,
    #[prop(into, default = String::from("Button Image"))] alt: String,
    #[prop(default = 350)] width: u32,
    #[prop(default = 320)] height: u32,
) -> impl IntoView {
    view! {
        <a
            href=href
            style="
                display: inline-block;
                border: none;
                background: none;
                padding: 0;
                cursor: pointer;
                text-decoration: none;
            "
        >
            <img
                src=img_src
                alt=alt
                style=format!(
                    "
                        width: {}px;
                        height: {}px;
                        border-radius: 45.7px;
                        object-fit: cover;
                        display: block;
                    ",
                    width, height
                )
            />
        </a>
    }
}
