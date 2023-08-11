// Waiting on a fix in: https://github.com/rust-lang/cargo/issues/6659
// use magnus::{RHash, Symbol};
// use mustachers;

// #[test]
// fn it_interpolates_correctly() {
//     let params = RHash::new();
//     params.aset(Symbol::new("top"), String::from("<h1>Page heading</h1>")).unwrap();
//     params.aset(Symbol::new("middle"), String::from("<p>Paragraph of text.</p>")).unwrap();
//     params.aset(Symbol::new("bottom"), String::from("<a href=\"#about\">Page link</a>")).unwrap();
//     params.aset(Symbol::new("lang"), String::from("en-GB")).unwrap();
//     params.aset(Symbol::new("foo"), String::from("bar")).unwrap();

//     let result = mustachers::wrapper(String::from("<h1>{{template}}</h1>"), params).unwrap();
//     assert_eq!(result, "<html lang=\"en-GB\"/><h1>Page heading</h1><p>Paragraph of text.</p><a href=\"#about\">Page link</a>")
// }
