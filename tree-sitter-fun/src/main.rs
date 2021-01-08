use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_verilog() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_verilog() };
    parser.set_language(language).unwrap();

    let source_code = "module mymodule(); endmodule";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    assert_eq!(root_node.kind(), "source_file");
    assert_eq!(root_node.start_position().column, 0);
}