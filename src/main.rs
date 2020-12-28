use pinewood_derby::parser::parse;

fn main() {
   println!("{:#?}", parse("tests/test.c"));
}
