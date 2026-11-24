use std::sync::{Arc, Mutex};

use gpui_kit::AppContext;
use umuryango_app::RootView;
use umuryango_app::assets::AppAssets;
use umuryango_app::db_facade::LocalDatabase;
use umuryango_app::state::AppState;
use umuryango_app::theme::archivo_font_bytes;

fn main() {
    println!("Initializing Umuryango Ledger GPUI desktop/mobile application...");
    let db =
        LocalDatabase::open_default().unwrap_or_else(|_| LocalDatabase::open_in_memory().unwrap());
    let state = AppState::default();
    let db_arc = Arc::new(Mutex::new(db));

    println!(
        "Application ready. API: {}, Locale: {:?}",
        state.api_base_url, state.locale
    );

    let app = gpui_kit::application().with_assets(AppAssets);
    app.run(move |cx| {
        gpui_kit::init(cx);

        if let Err(e) = cx.text_system().add_fonts(archivo_font_bytes()) {
            eprintln!("Warning: Failed to load bundled Archivo fonts: {:?}", e);
        }

        let options = gpui_kit::WindowOptions {
            titlebar: Some(gpui_kit::TitlebarOptions {
                title: Some("Umuryango Ledger".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let db_clone = Arc::clone(&db_arc);
        let _ = gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| RootView::new(state, Some(db_clone), window, cx))
        });
    });
}
