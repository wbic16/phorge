use std::collections::HashSet;
use std::fs;
use std::env;
use sha2::{Sha512, Digest};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};

// buffer_hash_62 intentionally mutates /, +, and = characters to ensure that hashes fit within base-62
fn buffer_hash_62(buffer: String) -> String {
  let mut hasher = Sha512::new();
  hasher.update(buffer);
  let result = hasher.finalize();
  return STANDARD_NO_PAD.encode(result).replace("/", "e").replace("+","a").replace("=","");
}

fn left_handed_count(test: &str) -> usize {
  let left: HashSet<char> = "arbjwitdmzngq".chars().collect();
  test.chars().filter(|c| left.contains(&c)).count()
}

fn right_handed_count(test: &str) -> usize {
  let right: HashSet<char> = "esckxolfpuyhv".chars().collect();
  test.chars().filter(|c| right.contains(&c)).count()
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
  let lines = buffer.split("\n").collect::<Vec<&str>>();
  println!("Read {} lines and {} bytes from {}.", lines.len(), buffer.len(), file);

  let nova_fox_version: String = buffer_hash_62(lines[0].into());
  let user_email: String = buffer_hash_62(lines[1].into());
  let user_seed: String = buffer_hash_62(lines[2].into());
  let current_date: String = buffer_hash_62(lines[3].into());
  let parent_room: String = buffer_hash_62(lines[4].into());
  let ancestor_room: String = buffer_hash_62(lines[5].into());
  let new_coordinate: String = buffer_hash_62(lines[6].into());
  let buffer_length = buffer_hash_62(buffer.len().to_string());
  let left_handed = buffer_hash_62(left_handed_count(&buffer).to_string());
  let right_handed = buffer_hash_62(right_handed_count(&buffer).to_string());
  let digest = buffer_hash_62(buffer);
  let mut main_hash = "".to_owned();
  main_hash += &nova_fox_version;
  main_hash += &user_email;
  main_hash += &user_seed;
  main_hash += &current_date;
  main_hash += &parent_room;
  main_hash += &ancestor_room;
  main_hash += &new_coordinate;
  main_hash += &buffer_length;
  main_hash += &left_handed;
  main_hash += &right_handed;
  main_hash += &digest;
  let final_hash = buffer_hash_62(main_hash.to_owned());
  main_hash += &final_hash;

  if cfg!(debug_assertions) {
    println!("{}", nova_fox_version); 
    println!("{}", user_email);       
    println!("{}", user_seed);        
    println!("{}", current_date);     
    println!("{}", parent_room);      
    println!("{}", ancestor_room);    
    println!("{}", new_coordinate);   
    println!("{}", buffer_length);    
    println!("{}", left_handed);      
    println!("{}", right_handed);     
    println!("{}", digest);           
    println!("{}", final_hash);       
  } else {
    println!("{}", main_hash);
  }
}