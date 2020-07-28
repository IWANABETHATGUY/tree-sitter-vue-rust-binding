use std::path::PathBuf;
use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_vue() -> Language;
}
fn main() {
    let language = unsafe { tree_sitter_vue() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    let source_code = r#"
<template>
  <p>
    Hello, <a :[key]="url">{{ name }}</a>!
  </p>
</template>
"#;

    let tree = parser.parse(source_code, None).unwrap();
    println!("{:?}", tree.root_node());
}
