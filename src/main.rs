#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let arr = vec!["q", "w", "e"];
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            "style":"background-color:gray;",
            "Hello, world!",
            div{
                hidden:false,
                background:false,
                "style": "background-color: red;",
                "Hello"
            }
            arr.iter().map(|i| rsx!{div{i}})
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    })
}
