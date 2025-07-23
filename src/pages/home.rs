use crate::components::banner::Banner;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Banner />
        <main>
            <h2>"Hello!"</h2>
            <p>"This website is still being built. "</p>
        </main>
    }
}
