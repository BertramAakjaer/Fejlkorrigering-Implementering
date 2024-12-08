use std::fs::File;
use std::io::{self, Write};
use std::io::Read;
use rand::Rng;

use crate::data_classes::DataHolder;

pub fn save_as_binary(data: &str, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn random_bit_flipper(data: &str, changes: u32) -> io::Result<()>{
    let mut file = File::open(data)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let mut rng = rand::thread_rng();
    for _ in 0..changes {
        let byte_index = rng.gen_range(0..bytes.len());
        let bit_index = rng.gen_range(0..8);
        // println!("Before: {:08b}", bytes[byte_index]);
        bytes[byte_index] ^= 1 << bit_index; // Flip a single bit in the byte
        // println!("After: {:08b}", bytes[byte_index]);
    }   
    let mut file = File::create(data)?;

    file.write_all(&bytes)?;
    
    Ok(())
}



pub fn apply_paritybit(files: &mut DataHolder) -> io::Result<()> {

    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // println!("{:?}", data);

    
    let mut new_data = Vec::with_capacity(data.len() + 1);
    new_data.extend_from_slice(&data);
    
    let bit_count = data.iter().map(|byte| byte.count_ones()).sum::<u32>();
    let parity_byte = if bit_count % 2 == 0 {
        0b0000_0000
    } else {
        0b1111_1111
    };

    new_data.insert(0, parity_byte);


    // println!("{:?}", new_data);

    let mut file = File::create(format!("output/{}.pab", files.file_name))?;
    file.write_all(&new_data)?;

    files.add_err_corr_data(String::from("parity_bit"), format!("output/{}.pab", files.file_name));

    Ok(())
}


pub fn check_paritybit(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");

    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    let data_first_byte = data[0];
    let data = &data[1..];

    let mut bit_count_first = data_first_byte.count_ones();

    if bit_count_first == 0 || bit_count_first == 8 {
        messages_to_user.push(String::from("Parity is valid"));
        if bit_count_first == 8 { bit_count_first = 1; } else { bit_count_first = 0; }
    } else {
        if bit_count_first >= 4 {
            messages_to_user.push(String::from("Parity is invalid, but fixed to 1"));
            bit_count_first = 1;
        } else {
            messages_to_user.push(String::from("Parity is invalid, but fixed to 0"));
            bit_count_first = 0;
        }
    }

    let bit_count = data.iter().map(|byte| byte.count_ones()).sum::<u32>();

    if (bit_count + bit_count_first) % 2 == 0 {
        messages_to_user.push(String::from("Data is normal and not modified."));
        return messages_to_user;
    } else {
        messages_to_user.push(String::from("Data is modified and an error has been detected"));
        return messages_to_user;
    }
}