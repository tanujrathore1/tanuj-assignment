
use clap::Parser;
use protobuf::Message;
use std::fs::write;
use std::io;
use std::io::prelude::*;
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

use person::Person;

#[derive(Debug, Parser)]
struct UserInput {
    #[clap(long)]
    input: String,

    #[clap(long)]
    output: String,
}

fn main() -> Result<(), io::Error> {
    let file_path = UserInput::parse();
    let input_file_path = file_path.input;
    let output_file_path = file_path.output;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(input_file_path)?;

    let mut data: Vec<Vec<u8>> = Vec::new();

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut binary_file_data = String::new();
    for entries in content.lines() {
        let mut values = entries.split(',');
        let last_name = values.next().expect("Enter last name").to_string();
        let first_name = values.next().expect("Enter first name").to_string();
        let date_of_birth = values.next().expect("Enter date of birth").to_string();

        let mut person = Person::new();
        person.last_name = last_name;
        person.first_name = first_name;
        person.date = date_of_birth;

        let person_in_bytes: Vec<u8> = person.write_to_bytes().unwrap_or(Vec::new());
        let data_length = person_in_bytes.len() as u64;

        let mut buffer = Vec::new();
        protobuf::CodedOutputStream::vec(&mut buffer)
            .write_raw_varint64(data_length)
            .expect("Failed to write varint-encoded u64");

        let binary_data = format!("{:?}{:?}\n", &buffer, person_in_bytes);
        binary_file_data.push_str(&binary_data);

        data.push(person_in_bytes);
    }
    

    write("binary.cf", binary_file_data).expect("file is not accessible");
    
    // writing data in output file
    let mut content = String::new();

    for ele in data {

        let in_msg = Person::parse_from_bytes(&ele).unwrap_or(Person::new());
        let file_data = format!(
            "{:#?},{:#?},{:#?}\n",
            in_msg.last_name, in_msg.first_name, in_msg.date
        );
        content.push_str(&file_data);
    }

    
    write(output_file_path, content)

    // Ok(())
}
