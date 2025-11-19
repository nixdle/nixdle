use nixdle::{new, parse_builtin_types, parse_functions};

fn main() {
  let functions = parse_functions(include_str!("../data/data.json")).unwrap();
  println!(
    "{:#?}",
    new(
      functions,
      parse_builtin_types(include_str!("../data/builtins.types.json")).unwrap()
    ),
  );
}
