use tauri::{
    // image::Image,
    menu::{CheckMenuItemBuilder, IconMenuItemBuilder, MenuBuilder, SubmenuBuilder},
};

use tauri::Manager;
use tauri::Emitter;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let menu = MenuBuilder::new(app)
                .text("open", "Open")
                .text("close", "Close")
                .build()?;

            let window = app.get_webview_window("main").unwrap();
            window.set_menu(menu)?;

            let app_handle = app.handle().clone();

            window.on_menu_event(move |_w, event| {
                match event.id().as_ref() {
                    "open"  => {
                        println!(">>> Open clicked");
                        app_handle.emit("menu-clicked", "open");
                    },
                    "close" => {
                        println!(">>> Close clicked");
                        app_handle.emit("menu-clicked", "close");
                        // _w.close().unwrap();
                    }
                    _ => {}
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!());
}
