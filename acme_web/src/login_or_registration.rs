use leptos::{component, create_signal, event_target_value, IntoView, SignalGet, SignalSet, view};
use regex::Regex;


#[component]
pub fn LoginOrRegistration() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-screen">
            <div class="flex flex-col justify-center items-stretch items-center w-1/5">
                <h2 class="text-xl pb-6 text-center">Log in or sign up</h2>
                <div class="flex flex-col items-stretch space-y-2">
                    <SocialLoginOrRegisterBtn
                        src_path=String::from("/static/icons/apple.svg") auth_provider=String::from("Apple") />
                    <SocialLoginOrRegisterBtn
                        src_path=String::from("/static/icons/google.svg") auth_provider=String::from("Google") />
                    <SocialLoginOrRegisterBtn
                        src_path=String::from("/static/icons/facebook.svg") auth_provider=String::from("Facebook") />
                </div>
                <div class="text-slate-300 text-center">or</div>
                <EnterWithEmailLink />
            </div>

        </div>
    }
}

#[component]
fn SocialLoginOrRegisterBtn(
    src_path: String,
    auth_provider: String,
) -> impl IntoView {
    let button_label = format!("Continue with {}", auth_provider);
    view! {
        <button class="flex items-center justify-center space-x-2 bg-zinc-100 px-4 py-2 rounded-lg w-full">
            <img class="w-8 h-auto pr-2" src=src_path />
            <div class="text-center text-sm">{button_label}</div>
        </button>
    }
}

#[component]
fn EnterWithEmailLink() -> impl IntoView{
    let (name, set_name) = create_signal("".to_string());
    let is_email = move || -> bool {
        let email_regex = Regex::new(r"^[\w\.-]+@[a-zA-Z\d\.-]+\.[a-zA-Z]{2,}$").unwrap();
        let value = &name.get();
        email_regex.is_match(&value)
    };
    view! {
        <div class="flex flex-col items-stretch justify-center items-center space-y-3">
            <input class="border p-2 rounded hover:border-cyan-900" placeholder="Email" type="text"
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
                class=("border-red-400", move || !is_email())
            />
            <button class="bg-gray-500 rounded-lg p-2">
                <span class="text-slate-200">Continue</span>
            </button>
        </div>
    }
}
