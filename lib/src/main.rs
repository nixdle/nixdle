use nixdle::{new, parse_functions};

fn main() {
  let functions = parse_functions(include_str!("../data.json")).unwrap();
  println!("{:#?}", new(functions));
}
