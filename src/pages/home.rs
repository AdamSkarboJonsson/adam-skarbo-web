use leptos::prelude::*;
use crate::components::banner::Banner;

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
