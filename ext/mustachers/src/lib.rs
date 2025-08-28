use magnus::{function, prelude::*, r_hash::ForEach, Error, RHash, RModule, Ruby, Symbol, Value};
use std::collections::HashMap;

mod renderer;

pub fn wrapper(ruby: &Ruby, template: String, params: RHash) -> Result<String, Error> {
    let mut data: HashMap<String, String> = HashMap::new();

    params.foreach(|key: Symbol, value: Value| {
        data.insert(key.to_string(), value.to_string());

        return Ok(ForEach::Continue);
    })?;

    return renderer::render(template, data)
        .map_err(|e: std::fmt::Error| Error::new(ruby.exception_runtime_error(), e.to_string()));
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let mustachers_module: RModule = ruby.define_module("Mustachers")?;
    let renderer_module: RModule = mustachers_module.define_module("Renderer")?;

    renderer_module.define_singleton_method("render", function!(wrapper, 2))?;

    Ok(())
}
