use tauri::{CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

pub fn build_menu() -> Menu {
    let about = CustomMenuItem::new("about".to_string(), "About");
    let submenu = Submenu::new("Help", Menu::new().add_item(about));
    Menu::new().add_submenu(submenu)
}

pub fn menu_event_handler(event: WindowMenuEvent) {
    if event.menu_item_id() == "about" {
        let handle = event.window().app_handle();

        let _ = tauri::WindowBuilder::new(&handle, "about", tauri::WindowUrl::App("/about".into()))
            .menu(Menu::new())
            .inner_size(200., 200.)
            .max_inner_size(200., 400.)
            .resizable(false)
            .build()
            .unwrap();
    }
}
