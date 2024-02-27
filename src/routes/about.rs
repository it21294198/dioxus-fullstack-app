// use create::about::About;
use dioxus::prelude::*;

pub fn About(cx: Scope) -> Element {
    let name = use_state(cx, || "".to_string());

    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        hr{}
        " An Open Source project dedicated to making Rust UI wonderful."
        hr{}
        button {
            onclick: move |e| name.set("hello".to_string()) ,
            "click me!"
        }
        hr{}
        input {
            // we tell the component what to render
            value: "{name}",
            // and what to do when the value changes
            oninput: move |evt| name.set(evt.value.clone()),
        }
        hr{}
        h1{ (format!("Name is {name}")) }
    }))
}
