use core::fmt::Error;
use std::collections::HashMap;

extern crate mustache;

pub fn render(template: String, params: HashMap<String, String>) -> Result<String, Error> {
    let template = mustache::compile_str(&template).expect("Failed to compile");
    let mut bytes = vec![];

    template
        .render(&mut bytes, &params)
        .expect("Failed to render");

    return Ok(String::from_utf8(bytes).expect("Failed to encode string"));
}

#[cfg(test)]
mod tests {
    use crate::renderer::render;
    use std::collections::HashMap;

    #[test]
    fn it_interpolates_correctly() {
        let template =
            String::from("<html lang=\"{{{lang}}}\"/>{{{ top }}}{{{ middle }}}{{{ bottom }}}");
        let params = HashMap::from([
            (String::from("top"), String::from("<h1>Page heading</h1>")),
            (
                String::from("middle"),
                String::from("<p>Paragraph of text.</p>"),
            ),
            (
                String::from("bottom"),
                String::from("<a href=\"#about\">Page link</a>"),
            ),
            (String::from("lang"), String::from("en-GB")),
            (String::from("foo"), String::from("bar")),
        ]);

        let result = render(template, params).unwrap();
        assert_eq!(result, "<html lang=\"en-GB\"/><h1>Page heading</h1><p>Paragraph of text.</p><a href=\"#about\">Page link</a>");
    }
}
