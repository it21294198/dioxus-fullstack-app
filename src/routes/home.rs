use dioxus::prelude::*;
use dioxus_router::prelude::Link;

pub fn Home(cx: Scope) -> Element {
    let _name = use_state(cx, || "".to_string());

    cx.render(rsx!(p {

        div{
            "Home page"
            h2{
                // Link { to: "/", "Go home!" }
                Link { to: "/about", "Go about!" }
            }
        }
    }))
}
