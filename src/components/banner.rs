use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div id="banner">
            <div class="banner-content">
                <div class="banner-title">
                    <A href="/">
                        <h1>
                            "Adam Skärbo Jonsson" 
                            <br/> 
                            <span class="subtitle">"Machine Learning Engineer" </span>
                        </h1>
                    </A>
                </div>
                <div class="banner-contacts">
                    <a href="mailto:adskarbo@gmail.com" class="banner-contact-item">
                        <span class="banner-contact-icon">"✉"</span>
                        <span>"Email"</span>
                    </a>
                    <a href="https://www.linkedin.com/in/adam-sk%C3%A4rbo-jonsson-70847a1a3/" target="_blank" class="banner-contact-item">
                        <span class="banner-contact-icon">"💼"</span>
                        <span>"LinkedIn"</span>
                    </a>
                    <a href="https://github.com/AdamSkarboJonsson" target="_blank" class="banner-contact-item">
                        <span class="banner-contact-icon">"🔗"</span>
                        <span>"GitHub"</span>
                    </a>
                </div>
            </div>
        </div>
    }
}