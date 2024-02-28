// use create::about::About;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

pub fn About(cx: Scope) -> Element {
    let _name = use_state(cx, || "".to_string()); // Prefixing with underscore

    cx.render(rsx!(p {
        b {"Dioxus Labs"}
        hr{}
        " An Open Source project dedicated to making Rust UI wonderful."
        hr{}
        button {
            onclick: move |_e| _name.set("hello".to_string()) , // Using _name
            "click me!"
        }
        hr{}
        input {
            // we tell the component what to render
            value: "{_name}",
            // and what to do when the value changes
            oninput: move |evt| _name.set(evt.value.clone()),
        }
        hr{}
        h1{ (format!("Name is {_name}")) } // Using _name
        div{
            h2{
                Link { to: "/", "Go home!" }
                // Link { to: "/about", "Go about!" }
            }
        }
    }))
}
