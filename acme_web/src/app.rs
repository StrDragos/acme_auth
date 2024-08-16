use leptos::{component, IntoView, view};
use leptos_meta::Stylesheet;
use leptos_router::{Route, Router, Routes};

use crate::home::Home;
use crate::login_or_registration::LoginOrRegistration;

#[component]
pub fn App() -> impl IntoView {
    // provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/register" view=LoginOrRegistration />
                </Routes>
            </main>
        </Router>
    }
}