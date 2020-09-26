use gendoc::*;

fn main() {
    let mut document_option = parser::parse_option();

    document_option.convert();

    generator::generate(&document_option);
}
