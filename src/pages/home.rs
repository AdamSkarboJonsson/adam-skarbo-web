use crate::components::banner::Banner;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Banner />
        <main>
        <article>
            <h2>"Hello!"</h2>
            <p>"This website is under construction."</p>
            </article>
        </main>
    }
}
