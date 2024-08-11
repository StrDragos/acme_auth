use leptos::{component, IntoView, view};
use leptos_meta::Stylesheet;
use leptos_router::{Route, Router, Routes};

// use crate::register::Register;

#[component]
pub fn App() -> impl IntoView {
    // provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    // <Route path="/register" view=Register />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    // let (count, set_count) = create_signal(0);
    view! {
        <div class="flex justify-center items-center h-screen">
            <div class="flex flex-col justify-center items-center w-2/5 bg-red-400">
                <h2>Login or signup</h2>
                <div class="flex flex-col">
                    <SocialLoginOrRegisterBtn />
                    <SocialLoginOrRegisterBtn />
                    <SocialLoginOrRegisterBtn />
                </div>
            </div>

        </div>
    }
}

#[component]
fn SocialLoginOrRegisterBtn() -> impl IntoView {
    view! {
        <button>
            <div class="flex flex-row">
                <img src="icons/google.svg" />
                <div>Continue with Google</div>
            </div>
        </button>
    }
}

