use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());

    cx.render(rsx!(p {

        h1{
            "Home page"
        }
    }))
}
