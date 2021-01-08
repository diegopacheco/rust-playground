fn main() {
    let language = "verilog";
    let package = format!("tree-sitter-{}", language);
    let source_directory = format!("{}/src", package);
    let source_file = format!("{}/parser.c", source_directory);

    println!("cargo:rerun-if-changed={}", source_file); // <1>

    cc::Build::new()
        .file(source_file)
        .include(source_directory)
        .compile(&package); // <2>
}