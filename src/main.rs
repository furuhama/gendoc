use gendoc::*;

fn main() {
    let mut option = parser::parse_option();

    option.convert();

    generator::generate(&option);
}
