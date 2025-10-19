use leptos::task::spawn_local;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
struct MenuPayload {
    event: String,
    payload: String,
    id: u32,
}

#[component]
pub fn App() -> impl IntoView {
    let (msg, set_msg) = signal(String::new());

    spawn_local(async move {
        let closure = Closure::wrap(Box::new(move |event: JsValue| {
            // 直接提取 event.payload.paths
            let event: MenuPayload = serde_wasm_bindgen::from_value(event).unwrap();
            leptos::logging::log!("event = {:?}", event);
            let MenuPayload { event: _, payload, id: _ } = event;
            set_msg.set(payload);
        }) as Box<dyn FnMut(JsValue)>);

        let _ = listen("menu-clicked", closure.as_ref().into()).await;
        closure.forget();
    });


    view! {
        <main class="container">
            <p>{ move || msg.get() }</p>
        </main>
    }
}
