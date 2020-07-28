use std::path::PathBuf;
use tree_sitter::{Language, Parser, TreeCursor, Node};

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
    let node = tree.root_node();
    pretty_print(node, 0);
}

fn pretty_print(root: Node, level: usize ) {
    let kind = root.kind();
    let start = root.start_position();
    let end = root.end_position();
    println!("{}{} [{}, {}] - [{}, {}]"," ".repeat(level * 2), kind, start.row, start.column, end.row, end.column);
    for i in 0..root.named_child_count() {
        let node = root.named_child(i).unwrap();
        pretty_print(node, level + 1);
    }
}