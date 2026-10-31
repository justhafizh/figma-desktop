use std::sync::atomic::{AtomicUsize, Ordering}; // BARU
use tauri::webview::NewWindowResponse; // BARU
use tauri::{WebviewUrl, WebviewWindowBuilder};

static POPUP_ID: AtomicUsize = AtomicUsize::new(0); // BARU

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      let handle = app.handle().clone(); // BARU

      let window = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External("https://www.figma.com".parse().unwrap()),
      )
      .title("Figma Desktops")
      .inner_size(1024.0, 600.0)
      //.user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36")
      // BARU: popup (mis. login Google) dibuat sebagai jendela yang tersambung ke jendela utama
      .on_new_window(move |url, features| {
        let n = POPUP_ID.fetch_add(1, Ordering::Relaxed);
        let popup = WebviewWindowBuilder::new(
          &handle,
          format!("popup-{n}"),
          WebviewUrl::External("about:blank".parse().unwrap()),
        )
        .window_features(features)
        .title(url.as_str())
        .build()
        .unwrap();
        NewWindowResponse::Create { window: popup }
      })
      .build()?;

      window.set_zoom(0.9)?;

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}