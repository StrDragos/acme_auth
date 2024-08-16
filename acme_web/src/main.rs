use leptos::{logging, mount_to_body, view};

mod app;
mod login_or_registration;
mod home;

use crate::app::App;
fn main() {
    console_error_panic_hook::set_once();
    logging::log!("csr mode -mounting to body");
    mount_to_body(|| view! { <App /> });
}