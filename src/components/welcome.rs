use leptos::prelude::*;

#[component]
pub fn Welcome() -> impl IntoView {
    view! {
        <div class="pt-16 grid grid-cols-1 gap-4 bg-green-200">
            <div class="grid grid-cols-2 bg-gray-100">
                <div class="place-content-center ...">
                    <p class="text-3xl place-self-center p-20 ...">Enhancing device security, strengthening fundamentals, and accelerating the delivery of high-quality devices</p>
                </div>
                <div>
                    <img src="/images/laptop.jpg" class="p-9" />
                </div>
            </div>
            <div>3</div>
            <div>4</div>
            <div>5</div>
        </div>
    }
}
