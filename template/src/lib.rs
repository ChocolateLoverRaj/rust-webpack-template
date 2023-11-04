use wasm_bindgen::JsValue;
use wasm_react::{export_components, h, hooks::use_state, Callback, Component, VNode};

struct Counter {
    initial_counter: i32,
}

impl Component for Counter {
    fn render(&self) -> VNode {
        let counter = use_state(|| self.initial_counter);

        let vnode = h!(div).build((
            h!(p).build(("Counter: ", *counter.value())),
            h!(button)
                .on_click(&Callback::new({
                    let mut counter = counter.clone();
                    move |_| counter.set(|c| c + 1)
                }))
                .build("+1"),
            h!(button)
                .on_click(&Callback::new({
                    let mut counter = counter.clone();
                    move |_| counter.set(|c| c - 1)
                }))
                .build("-1"),
        ));
        vnode
    }
}

pub struct App;

impl Component for App {
    fn render(&self) -> VNode {
        Counter { initial_counter: 0 }.build()
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }
