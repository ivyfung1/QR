//Get registrants information and create QR code
//Read QR code info

use std::io;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use qrcode::QrCode;
use qrcode::render::svg;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;
use std::path::PathBuf;
use serde_json::Value;
use serde_json::from_str;

#[derive(Debug)]
struct Participant {
  name: String,
  email: String,
  contact: u16,
  gender: String,
}

fn update_file_content(filename: &str, counter: u8) -> Result< (), std::io::Error> {
  // Open the file with write access and create it if it doesn't exist
  let mut file = OpenOptions::new()
      .write(true)
      .create(true)
      .open(filename)?;

  // Move the cursor to the beginning of the file and truncate it
  file.seek(io::SeekFrom::Start(0))?;
  file.set_len(0)?;

  // Write the updated content back to the file
  file.write_all(&[counter])?;
  
  println!("Your registration number is {}.", counter);
  
  Ok(())
}

fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();
  let mut input4 = String::new();

  println!("What's your name?");
  io::stdin().read_line(&mut input1).expect("Failed to read line");
  let namex: String = input1.trim().to_string();

  println!("What's your email?");
  io::stdin().read_line(&mut input2).expect("Failed to read line");
  let emailx: String = input2.trim().to_string();

  println!("What's your contact?");
  io::stdin().read_line(&mut input3).expect("Failed to read line");
  let contactx: u16 = input3.parse().unwrap_or_default();

  println!("What's your gender?");
  'input: loop {
    io::stdin().read_line(&mut input4).expect("Failed to read line");
    input4 = input4.trim().to_string().to_uppercase();
    if input4 == ("F") || input4 == ("M") {
    //  break;
    } else {
      println!("Please only enter F or M.");
      input4 = String::new();
      continue 'input;
    }
    break;
  }

  let genderx: String = input4;

  // Define the data you want to encode
  // Check if the file exists
  let _file_exists = fs::metadata("qr.txt").is_ok();

  // Open the file with read and write access, create if it doesn't exist
  let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("qr.txt")
    .expect("Failed to open file");

   // Read the existing counter from the file
   let mut existing_content = Vec::new();
   file.read_to_end(&mut existing_content).expect("Failed to read file");
   let mut counter: u8 = existing_content.get(0).cloned().unwrap_or_default();

   // Increment the counter
   counter += 1;
  let ps = Participant{
    name: namex,
    email: emailx,
    contact: contactx,
    gender: genderx,
  };

  let mut participants: HashMap <u8, Participant> = HashMap::new();
  participants.insert(counter, ps);

  // Update the content of the file
  let _ = update_file_content("qr.txt", counter);

  // Create a QR code
  let code = QrCode::new(format!("{:?}", participants)).expect("Failed to create QR code"); // changed from ps to participants
  
  // Specify rendering options (SVG in this case)
  let svg = code.render::<svg::Color>().build();
  
  // Save the SVG code to a file
  let qr_code_name: String = format!("{:03}", counter);
  std::fs::write(qr_code_name.clone(), svg).expect("Failed to save QR code as SVG");
  
  println!("QR code generated and saved as '{}.svg'", qr_code_name);

  println!("Do you want to read inforamtion from other QR Code? (Y/N)");

  'Read: loop {
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5).expect("Failed to read line");
    input5 = input5.trim().to_string().to_uppercase();
    if input5 == ("Y") {
      println!("Please enter the registration number.");
      let mut input6 = String::new();
      match io::stdin().read_line(&mut input6){
        Ok(_) => {
          let file_path_string = Path::new("/Users/ivyfung/projects/rust/QR/src/").join(input6.trim());
          let file_path = file_path_string.as_os_str();
          
          let svg_content = std::fs::read_to_string(file_path);

          println!("svg_content{:?}", svg_content);
          // Extract encoded data from SVG content
          let encoded_data = svg_content.expect("REASON")
              .lines()
              .filter(|line| line.contains("data:application/qrcode;base64,"))
              .next()
              .map(|line| {
                  line.trim()
                      .replace("data:application/qrcode;base64,", "")
                      .to_string()
              })
              .unwrap_or_default();
      
          // Decode the encoded data (you may need to use a base64 decoding library)
          let decoded_data = base64::decode(encoded_data).expect("Failed to decode base64");
      
          // Assuming the decoded data is a valid UTF-8 string
          let decoded_str = String::from_utf8(decoded_data).expect("Failed to convert to UTF-8");

          // Deserialize the decoded data into a generic JSON Value
          let json_value: Result<Value, _> = from_str(&decoded_str);
          
          match json_value {
            Ok(Value::Object(map)) => {
                // Print the deserialized data
                println!("Deserialized data: {:?}", map);
            }
            Ok(_) => {
                println!("Failed to deserialize into HashMap. Unexpected JSON format.");
            }
            Err(err) => {
                println!("Error deserializing JSON: {}", err);
                panic!();
            }     
          }
        }
        Err(err) => {
          println!("Error reading input: {}", err);
        }
      } 
    } else if input5 == ("N") {
      break;
    } else {
      println!("Please only enter Y or N.");
      continue 'Read;
    }
  }
}
