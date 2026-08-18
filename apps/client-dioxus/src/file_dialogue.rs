// File Path: apps/client-dioxus/src/file_dialogue.rs

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Url, Blob, BlobPropertyBag};
use serde_json::json;

#[wasm_bindgen]
pub struct WasmFileDialogueBridge;

impl WasmFileDialogueBridge {
    /// Triggers an asynchronous browser file download dialogue containing your encrypted, compressed .soma payload packet
    pub fn trigger_browser_hardware_download_save(soma_encrypted_ciphertext: &str) -> Result<(), JsValue> {
        let window = web_sys::window().ok_or("System exception: Global window context is unreachable.")?;
        let document = window.document().ok_or("System exception: Document structural layer context is unreachable.")?;

        // 1. Pack your string data into a browser-native binary Blob block layout matrix
        let mut properties = BlobPropertyBag::new();
        properties.type_("application/octet-stream");
        
        let javascript_array = js_sys::Array::new();
        javascript_array.push(&JsValue::from_str(soma_encrypted_ciphertext));
        
        let file_blob = Blob::new_with_str_sequence_and_options(&javascript_array, &properties)?;

        // 2. Generate a secure, temporary local blob URL allocation path layout mapping
        let download_url = Url::create_object_url_with_blob(&file_blob)?;

        // 3. Programmatically instantiate a phantom anchor link element to force browser file-save behaviors
        let phantom_anchor = document.create_element("a")?.dyn_into::<web_sys::HtmlAnchorElement>()?;
        
        let current_timestamp = js_sys::Date::now();
        phantom_anchor.set_href(&download_url);
        phantom_anchor.set_download(&format!("soma_health_vault_export_{}.soma", current_timestamp));
        
        // 4. Fire simulated user click events to deploy download windows instantly onto the screen interface
        phantom_anchor.click();
        
        // Clean up memory allocations by revoking the temporary pointer address string
        Url::revoke_object_url(&download_url)?;
        
        println!("🔒 Local user device triggered browser file dialogue download packaging successfully.");
        Ok(())
    }

    /// Configures an upload event hook handler to ingest an external .soma backup file directly back into memory frames
    pub fn trigger_browser_import_dialogue<F>(mut on_file_loaded_callback: F) -> Result<(), JsValue>
    where
        F: FnMut(String) + 'static,
    {
        let window = web_sys::window().ok_or("Global window context layer is missing.")?;
        let document = window.document().ok_or("Document node structure context layer is missing.")?;

        // Create a hidden interactive input file selector button widget dynamically inside memory
        let file_input = document.create_element("input")?.dyn_into::<HtmlInputElement>()?;
        file_input.set_type("file");
        file_input.set_accept(".soma");

        // Set up async event listener mappings to capture the input event
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::Event| {
            let target = event.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            if let Some(files) = target.files() {
                if let Some(file) = files.get(0) {
                    let file_reader = web_sys::FileReader::new().unwrap();
                    let file_reader_clone = file_reader.clone();
                    
                    let mut callback_wrapper = Box::new(on_file_loaded_callback) as Box<dyn FnMut(String)>;

                    let onload_closure = Closure::<dyn FnMut()>::new(move || {
                        if let Ok(result_value) = file_reader_clone.result() {
                            if let Some(plaintext_content) = result_value.as_string() {
                                println!("🔒 File payload safely uploaded and buffered inside WASM space memory bounds.");
                                (callback_wrapper)(plaintext_content);
                            }
                        }
                    });

                    file_reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                    onload_closure.forget();
                    file_reader.read_as_text(&file).unwrap();
                }
            }
        });

        file_input.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();

        // Launch the file explorer view overlay window screen dynamically
        file_input.click();
        Ok(())
    }
}
