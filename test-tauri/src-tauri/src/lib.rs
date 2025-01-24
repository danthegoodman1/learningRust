use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let tray = tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(|tray, event| match event {
                    tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } => {
                        println!("left click pressed and released");
                        // in this example, let's show and focus the main window when the tray is clicked
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        println!("unhandled event {event:?}");
                    }
                })
                .build(app)?;
            let webviews = app.webview_windows();
            for (label, window) in webviews {
                println!("label: {}", label);
                println!("window: {:?}", window.url().unwrap());
            }

            // let window = tauri::webview::WebviewWindowBuilder::new(app, "ee", tauri::WebviewUrl::External(tauri::Url::parse("https://www.google.com").?unwrap()))
            let window =
                tauri::webview::WebviewWindowBuilder::new(app, "ee", tauri::WebviewUrl::default())
                    // .always_on_top(true)
                    .position(100_f64, 100_f64)
                    .inner_size(1800_f64, 1200_f64)
                    .closable(false)
                    .minimizable(false)
                    .resizable(false)
                    .title_bar_style(tauri::TitleBarStyle::Overlay)
                    .title("")
                    .disable_drag_drop_handler()
                    .on_page_load(|window, _| {
                        println!("page loaded");
                    })
                    .build()
                    .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
