use wasm_bindgen::JsValue;
use wasm_react::{export_components, h, Component, VNode};

pub struct App;

impl Component for App {
    fn render(&self) -> VNode {
        h!(div).build("hi")
    }
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(_: JsValue) -> Result<Self, Self::Error> {
        Ok(App)
    }
}

export_components! { App }
