extern crate spirv_cross;
extern crate examples;
use spirv_cross::{msl, spirv};
use examples::words_from_bytes;

fn main() {
    let module = spirv::Module::from_words(words_from_bytes(include_bytes!("../vertex.spv")));

    // Parse a SPIR-V module
    let ast = spirv::Ast::<msl::Target>::parse(&module).unwrap();

    // List all entry points
    for entry_point in &ast.get_entry_points().unwrap() {
        println!("{:?}", entry_point);
    }

    // Compile to MSL
    let shader = ast.compile().unwrap();
    println!("{}", shader);
}
