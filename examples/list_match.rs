#[cfg(feature = "use-serde")]
use std::{fs, io};

#[cfg(feature = "use-serde")]
fn main() -> Result<(), io::Error> {
  use cirru_parser::{format, Cirru, CirruWriterOptions};
  // use std::collections::hash_map::DefaultHasher;
  use cirru_parser::from_json_str;

  let files = vec!["list-match"];

  for file in files {
    println!("testing file: {}", file);
    let json_str = fs::read_to_string(format!("./tests/writer_data/{}.json", file))?;
    let cirru_str = fs::read_to_string(format!("./tests/writer_cirru/{}.cirru", file))?;

    let writer_options = CirruWriterOptions { use_inline: false };
    match from_json_str(&json_str) {
      Ok(tree) => {
        if let Cirru::List(xs) = tree {
          assert_eq!(cirru_str, format(&xs, writer_options).unwrap());
        } else {
          panic!("unexpected leaf here")
        }
      }
      Err(e) => {
        println!("{:?}", e);
        panic!("failed to load edn data from json");
      }
    }
  }
  Ok(())
}

#[cfg(not(feature = "use-serde"))]
fn main() {
  println!("this example requires feature `use-serde`");
}
