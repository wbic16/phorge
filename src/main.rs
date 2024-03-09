use std::fs;
use std::env;
use sha2::{Sha512, Digest};

fn bufferHash(buffer: String) -> String {
  let mut hasher = Sha512::new();
  hasher.update(buffer);
  let result = hasher.finalize();
  let result = format!("{:x}", result);
  return result;
}

fn main() {

  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: phorge <file> <output>");
    return;
  }

  let file = &args[1];
  let mut output = "novafox.dat";
  if args.len() >= 3 {
    output = &args[2];
  }

  println!("Reviewing {} for gameplay data...", file);

  if fs::metadata(output).is_ok() {
    fs::remove_file(output).expect("unable to clear output file");
    return;
  }

  if !fs::metadata(file).is_ok() {
    eprintln!("Error: Unable to locate {}", file);
    return;
  }

  let message = "Unable to find ".to_owned() + file;
  let buffer:String = fs::read_to_string(file).expect(&message);

  println!("Read {} bytes from {}.", buffer.len(), file);

  let digest = bufferHash(buffer);  

  println!("Generated {} for {}.", digest, output);
}