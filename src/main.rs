use std::{fs::*, io::{BufReader, BufWriter, Read, Write}};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    room_x: u16,
    room_y: u16,
    room_number: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coords {
    positions: Vec<String>
}
fn main() {
    let name: String = "test".to_string();
    let mut file = File::create(name + ".timesliplvl").expect("Failed to create file");
    let magic_number = "TIMESLP".as_bytes();
    let version: i8 = 1;
    let mut buffer: Vec<u8> = Vec::new();

    // magic number
    for ch in magic_number.iter() {
        buffer.push(*ch)
    }
    buffer.push(version.to_le_bytes()[0]);
    buffer.push(0u16.to_le_bytes()[0]);

    buffer.append(&mut header());

    buffer.push(0u16.to_le_bytes()[0]);
    buffer.push(0u16.to_le_bytes()[0]);

    buffer.append(&mut sect(1));

    buffer.append(&mut coordinates_data("src/config/entities.json", "ENTY"));

    buffer.push(0u16.to_le_bytes()[0]);
    buffer.push(0u16.to_le_bytes()[0]);

    buffer.append(&mut sect(2));

    buffer.append(&mut coordinates_data("src/config/decorations.json", "DECO"));

    buffer.push(0u16.to_le_bytes()[0]);
    buffer.push(0u16.to_le_bytes()[0]);

    buffer.append(&mut sect(3));
    buffer.append(&mut coordinates_data("src/config/level_layout.json", "LAYO"));

    for ch in buffer.iter() {
        file.write_all(&[*ch]).expect("Failed to write to file")
    }
}

fn sect(number: u8) -> Vec<u8> {
    let mut vec:Vec<u8> = Vec::new();

    for ch in "SECT".chars(){
        vec.push(ch as u8)
    }
    vec.push(number.to_le_bytes()[0]);
    return vec;
}

fn header() -> Vec<u8>{
    let file = File::open("src/config/header.json").expect("Failed to read: config/header.json");
    let reader = BufReader::new(file);
    let header: Header = serde_json::from_reader(reader).expect("Failed to Deserialize header");

    println!("{:?}", header);
    let mut vec:Vec<u8> = Vec::new();

    for ch in header.room_x.to_le_bytes(){
        vec.push(ch)
    }
    vec.push('!' as u8);
    for ch in header.room_y.to_le_bytes(){
        vec.push(ch)
    }

    vec.push(0u16.to_le_bytes()[0]);
    vec.push(0u16.to_le_bytes()[0]);

    for ch in header.room_number.to_le_bytes(){
        vec.push(ch)
    }

    return vec;
}

fn coordinates_data(path: &str, sect_name: &str) -> Vec<u8>{
    let file = File::open(path).expect("Failed to read: {path}");
    let reader = BufReader::new(file);
    let mut entities: Coords = serde_json::from_reader(reader).expect("Failed to Deserialize path");

    println!("{:?}", entities);
    let mut vec:Vec<u8> = Vec::new();

    vec.push(0u16.to_le_bytes()[0]);

    for ch in sect_name.chars(){
        vec.push(ch as u8)
    }

    vec.push(0u16.to_le_bytes()[0]);
    vec.push(0u16.to_le_bytes()[0]);

    vec.push('[' as u8);

    for entity in entities.positions.iter_mut() {
        println!("Entity:{entity}");
        let mut entity_type = entity.split_off(5);
        println!("entity_type: {entity_type}");
        println!("coords: {entity}");

        let (x, y) = entity.split_at(2);
        let (v, y) = y.split_at(1);
        vec.push('(' as u8);
        drop(v);
        let x:u16 = x.parse().expect("Failed to parse x coordinates", );
        let y:u16 = y.parse().expect("Failed to parse y coordinates", );
        for ch in x.to_le_bytes(){
            vec.push(ch)
        }
        vec.push('!' as u8);
        for ch in y.to_le_bytes(){
            vec.push(ch)
        }

        let entity_type = entity_type.split_off(1);

        vec.push('/' as u8);
        
        for ch in entity_type.parse::<u8>().expect("Failed to parse the type").to_le_bytes(){
            vec.push(ch)
        }

        
        vec.push(')' as u8);
        vec.push(',' as u8);
    }
    vec.push(']' as u8);
    return vec;
}
