// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
    Emitter,
};

mod project;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // --- Application menu (macOS menu bar) -------------------------
            // The app menu (bold, uses the product name on macOS).
            let app_menu = SubmenuBuilder::new(app, "Bib")
                .about(None)
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .quit()
                .build()?;

            // File menu — open a project and save the active file. The Rust
            // side owns the accelerators and forwards the action to the webview
            // (same pattern as the View-menu zoom items below).
            let open_project = MenuItemBuilder::with_id("open-project", "Open Project…")
                .accelerator("CmdOrCtrl+O")
                .build(app)?;
            let save_file = MenuItemBuilder::with_id("save-file", "Save")
                .accelerator("CmdOrCtrl+S")
                .build(app)?;
            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&open_project)
                .separator()
                .item(&save_file)
                .build()?;

            // Standard Edit menu so copy/paste/undo keep working.
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .undo()
                .redo()
                .separator()
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;

            // View menu — zoom + full screen, editor style.
            let zoom_in = MenuItemBuilder::with_id("zoom-in", "Zoom In")
                .accelerator("CmdOrCtrl+=")
                .build(app)?;
            let zoom_out = MenuItemBuilder::with_id("zoom-out", "Zoom Out")
                .accelerator("CmdOrCtrl+-")
                .build(app)?;
            let reset_zoom = MenuItemBuilder::with_id("reset-zoom", "Reset Zoom")
                .accelerator("CmdOrCtrl+0")
                .build(app)?;
            let reset_all_zoom =
                MenuItemBuilder::with_id("reset-all-zoom", "Reset All Zoom").build(app)?;
            let fullscreen = PredefinedMenuItem::fullscreen(app, None)?;

            let view_menu = SubmenuBuilder::new(app, "View")
                .item(&zoom_in)
                .item(&zoom_out)
                .item(&reset_zoom)
                .item(&reset_all_zoom)
                .separator()
                .item(&fullscreen)
                .build()?;

            // Window menu.
            let window_menu = SubmenuBuilder::new(app, "Window")
                .minimize()
                .maximize()
                .separator()
                .close_window()
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&app_menu, &file_menu, &edit_menu, &view_menu, &window_menu])
                .build()?;

            app.set_menu(menu)?;

            // Zoom is owned by the frontend (CSS zoom + persistence), so the
            // menu just tells the webview which action to run.
            app.on_menu_event(move |app, event| match event.id().as_ref() {
                "open-project" => {
                    let _ = app.emit("menu:open-project", ());
                }
                "save-file" => {
                    let _ = app.emit("menu:save", ());
                }
                "zoom-in" => {
                    let _ = app.emit("menu:zoom", "in");
                }
                "zoom-out" => {
                    let _ = app.emit("menu:zoom", "out");
                }
                "reset-zoom" => {
                    let _ = app.emit("menu:zoom", "reset");
                }
                "reset-all-zoom" => {
                    let _ = app.emit("menu:zoom", "reset-all");
                }
                _ => {}
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            project::read_project_tree,
            project::read_text_file,
            project::write_text_file,
            project::search_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_includes_the_name() {
        let msg = greet("Ada");
        assert!(msg.contains("Ada"), "expected the name in: {msg}");
        assert!(msg.starts_with("Hello, Ada!"));
    }
}
