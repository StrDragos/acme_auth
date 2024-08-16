use leptos::{component, IntoView, view};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center h-screen">
            <div>Welcome</div>
        </div>
    }
}