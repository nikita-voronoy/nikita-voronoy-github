// Copyright (c) 2025 Mykyta Voronyi. Licensed under MIT.

use wasm_bindgen::JsCast;
use web_sys::window;

pub fn download_pdf(filename: &str) {
    let Some(win) = window() else { return };
    let Some(doc) = win.document() else { return };
    let Ok(el) = doc.create_element("a") else { return };

    let anchor: web_sys::HtmlAnchorElement = el.unchecked_into();
    anchor.set_href("resume.pdf");
    anchor.set_download(filename);
    anchor.click();
}
