use leptos::{component, view, IntoView, Show};
use leptos_meta::Stylesheet;
use leptos_router::{Redirect, Route, Router, Routes};
use crate::home::Home;
use crate::login_or_registration::LoginOrRegistration;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Router>
            <main>
                <Routes>
                    <Route path="/" view= move || {
                        view! {
                            <Show when=move|| {false} fallback=|| view! {<Redirect path="/register"/>} >
                                <Home />
                            </Show>
                        }
                    } />
                    <Route path="/register" view=LoginOrRegistration />
                </Routes>
            </main>
        </Router>
    }
}

// #[component]
// fn Protected<F, V>(view: F) -> impl IntoView
// where
//     F: Fn() -> V + 'static,
//     V: IntoView,
// {
//     //temporary
//     let is_logged_in = false;
//      view! {
//          if is_logged_in {
//              view()
//          } else {
//              let nav = use_navigate();
//              nav("/register", Default::default());
//          }
//      }
// }