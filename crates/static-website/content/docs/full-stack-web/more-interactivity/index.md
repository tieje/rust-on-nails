# More Interactivity (Webassembly)

If you are not able to get all the interactivity your application needs with Htmx then we can deploy some rust in the browser.

To do this we'll follow the guidelines in [Reasonable System for JavaScript Structure](https://ricostacruz.com/rsjs/) and apply those to Rust.

## Using a counter component as an example.

To follow the RSJS guidelines, we will use data attributes, that is, HTML attributes that begin with data-, a standard feature of HTML, to indicate that our HTML is a counter component. We will then write Rust to use an attribute selector that looks for the data-counter attribute as the root element in our counter component and wires in the appropriate event handlers and logic. Additionally, let’s rework the code to use query_selector() and add the counter functionality to all counter components found on the page. (You never know how many counters you might want!)

```html
<section class="counter" data-counter> <1>
  <output id="my-output" data-counter-output>0</output> <2>
  <button class="increment-btn" data-counter-increment>Increment</button>
</section>
```

## A new web-csr (client side rendering) crate

```sh
cargo new --lib crates/web-csr
```

## Cargo.toml

```toml
[package]
name = "web-csr"
version = "0.1.0"
edition = "2021"


[lib]
crate-type = ["cdylib"]

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.84"
web-sys = { version = "0.3.76", features = ['Document', 'Element', 'Event', 'HtmlElement', 'Node', 'Window', 'console'] }

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.7", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.34"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

## Our first component

This component attaches to any element with the attribute `data-alert` and shows an alert if the element is clicked.

```rust
use wasm_bindgen::prelude::*;
use web_sys::{window, Document, Element, HtmlElement};

// A simple helper function to get the document from the global window.
fn document() -> Document {
    window().expect("no global `window` exists").document().expect("should have a document")
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let doc = document();
    // Query for the button element with `data-alert` attribute.
    let button: Option<Element> = doc.query_selector("[data-alert]").ok().flatten();

    if let Some(btn_el) = button {
        // Convert to an HtmlElement to attach event listener easily.
        let btn = btn_el.dyn_into::<HtmlElement>()?;

        // Closure for the event listener.
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
            // Show the alert when button is clicked.
            window().unwrap().alert_with_message("Alert triggered!").unwrap();
        });

        // Add the click event listener to the button.
        btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget(); // Keep the closure alive for the lifetime of the program.
    } else {
        web_sys::console::log_1(&"No element with data-alert found!".into());
    }

    Ok(())
}
```

## Update our Justfile

Add an entry `wasm`

```justfile
wasm:
    cd /workspace/crates/web-csr && wasm-pack build --target web --out-dir dist
```

Now run `just wasm` and a wasm file will be created in the `dist` folder.

## Todo

- [ ] Update the layout
- [ ] Serve the assets