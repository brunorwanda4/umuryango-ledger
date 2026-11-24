/// WebAssembly entry point for mounting the full GPUI app in the browser.
use wasm_bindgen::prelude::*;

/// Mounts the full GPUI application onto the DOM `<canvas>` specified by `canvas_id`.
///
/// Starts directly into Login/Register or the main Dashboard, rendering 100% of the UI
/// using GPUI's high-performance layout engine and shaders inside WebAssembly.
#[wasm_bindgen]
pub fn mount_gpui_app(canvas_id: &str) -> Result<(), JsValue> {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::RootView;
        use crate::state::AppState;
        use crate::theme::archivo_font_bytes;
        use wasm_bindgen::JsCast;

        let window = web_sys::window()
            .ok_or_else(|| JsValue::from_str("No global window found in browser environment"))?;
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("No DOM document found"))?;
        let canvas = document.get_element_by_id(canvas_id).ok_or_else(|| {
            JsValue::from_str(&format!("Canvas with id '{}' not found in DOM", canvas_id))
        })?;

        let _canvas_elem = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Target element is not an HTMLCanvasElement"))?;

        // Initialize clean app state starting at Auth / Dashboard
        let state = AppState::default();

        let app = gpui_kit::application();
        app.run(move |cx| {
            gpui_kit::init(cx);

            if let Err(e) = cx.text_system().add_fonts(archivo_font_bytes()) {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "Failed to load bundled Archivo fonts into WASM GPUI: {:?}",
                    e
                )));
            }

            let mut options = gpui_kit::WindowOptions::default();
            options.titlebar = None;

            let _ = cx.open_window(options, |_window, cx| cx.new(|_| RootView::new_wasm(state)));
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = canvas_id;
    }

    Ok(())
}
