use std::sync::{Arc, Mutex};

use gpui_kit::AppContext;
use umuryango_app::RootView;
use umuryango_app::db_facade::LocalDatabase;
use umuryango_app::state::AppState;
use umuryango_app::theme::archivo_font_bytes;

fn main() {
    println!("Initializing Umuryango Ledger GPUI desktop/mobile application...");
    let db =
        LocalDatabase::open_default().unwrap_or_else(|_| LocalDatabase::open_in_memory().unwrap());
    let mut state = AppState::default();
    state.load_from_db(&db);

    let db_arc = Arc::new(Mutex::new(db));

    println!(
        "Application ready. Active scope: {:?}, Locale: {:?}",
        state.active_scope, state.locale
    );

    let app = gpui_kit::application();
    app.run(move |cx| {
        gpui_kit::init(cx);

        if let Err(e) = cx.text_system().add_fonts(archivo_font_bytes()) {
            eprintln!("Warning: Failed to load bundled Archivo fonts: {:?}", e);
        }

        let mut options = gpui_kit::WindowOptions::default();
        options.titlebar = Some(gpui_kit::TitlebarOptions {
            title: Some("Umuryango Ledger".into()),
            ..Default::default()
        });
        let db_clone = Arc::clone(&db_arc);
        let _ = cx.open_window(options, |_window, cx| {
            cx.new(|_| RootView::new(state, Some(db_clone)))
        });
    });
}
