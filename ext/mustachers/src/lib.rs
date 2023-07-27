extern crate mustache;

use std::collections::HashMap;
use magnus::{define_module, prelude::*, Error, function, RHash, Symbol, r_hash::ForEach};

fn render(template: String, params: RHash) -> Result<String, Error> {
    let template = mustache::compile_str(&template).expect("Failed to compile");
    let mut bytes = vec![];
    let mut data: HashMap<String, String> = HashMap::new();

    params.foreach(|key: Symbol, value: String| {
        data.insert(key.to_string(), value);

        return Ok(ForEach::Continue);
    })?;

    template.render(&mut bytes, &data).expect("Failed to render");

    return Ok(String::from_utf8(bytes).expect("Failed to encode string"));
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let mustachers_module = define_module("Mustachers")?;
    let renderer_module = mustachers_module.define_module("Renderer")?;
    renderer_module.define_singleton_method("render", function!(render, 2))?;

    Ok(())
}
