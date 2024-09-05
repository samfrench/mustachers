use core::fmt::Error;
use std::collections::HashMap;
use mustache::{compile_str, MapBuilder};
use std::sync::OnceLock;

static CACHED_TEMPLATE: OnceLock<mustache::Template> = OnceLock::new();

pub fn render(template: &str, params: &HashMap<String, String>) -> Result<String, Error> {
    let template = CACHED_TEMPLATE.get_or_init(|| compile_str(template).expect("Failed to compile"));

    let mut builder = MapBuilder::new();
    for (key, value) in params {
        builder = builder.insert_str(key, value);
    }
    let data = builder.build();

    let mut buffer = Vec::with_capacity(4096);
    template.render_data(&mut buffer, &data).expect("Failed to render");

    Ok(String::from_utf8(buffer).expect("Failed to encode string"))
}


#[cfg(test)]
mod tests {
    use super::render;
    use std::collections::HashMap;

    #[test]
    fn it_interpolates_correctly() {
        let template = "<html lang=\"{{{lang}}}\"/>{{{ top }}}{{{ middle }}}{{{ bottom }}}";
        let params = HashMap::from([
            ("top", "<h1>Page heading</h1>"),
            ("middle", "<p>Paragraph of text.</p>"),
            ("bottom", "<a href=\"#about\">Page link</a>"),
            ("lang", "en-GB"),
            ("foo", "bar"),
        ]);

        let result = render(template, &params).unwrap();
        assert_eq!(result, "<html lang=\"en-GB\"/><h1>Page heading</h1><p>Paragraph of text.</p><a href=\"#about\">Page link</a>");
    }
}
