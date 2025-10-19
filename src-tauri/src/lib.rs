use tauri::menu::{MenuBuilder, SubmenuBuilder};

use tauri::Manager;
use tauri::Emitter;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .text("quit", "Quit")
                .build()?;

            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .text("copy", "Copy")
                .text("paste", "Paste")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_menu, &edit_menu])
                .build()?;

            let window = app.get_webview_window("main").unwrap();
            window.set_menu(menu)?;

            let app_handle = app.handle().clone();

            window.on_menu_event(move |_w, event| {
                let id = event.id().as_ref();
                println!(">>> {} clicked", id);
                app_handle.emit("menu-clicked", id).unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
