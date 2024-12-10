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


pub fn apply_three_copies(files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut new_data = Vec::with_capacity(data.len() * 3);

    for _ in 0..3 {
        new_data.extend_from_slice(&data);
    }

    let mut file = File::create(format!("output/{}.3k", files.file_name))?;
    file.write_all(&new_data)?;

    files.add_err_corr_data(String::from("three_copies"), format!("output/{}.3k", files.file_name));

    Ok(())
}

fn compute_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc: u8, byte| acc.wrapping_add(*byte))
}

pub fn apply_checksum(files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let mut checksum = compute_checksum(&data);
    checksum = !checksum;

    let mut new_data = Vec::with_capacity(data.len() + 1);
    new_data.extend_from_slice(&data);
    new_data.push(checksum);

    let mut file = File::create(format!("output/{}.ces", files.file_name))?;
    file.write_all(&new_data)?;

    files.add_err_corr_data(String::from("checksum"), format!("output/{}.ces", files.file_name));

    Ok(())
}













pub fn check_paritybit(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");

    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    
    let data_first_byte = data[0];
    let data = &data[1..];
    
    let temp = String::from_utf8_lossy(&data);

    messages_to_user.push(format!("Filens data inden tjek for fejl ({})", temp));

    let mut bit_count_first = data_first_byte.count_ones();

    if bit_count_first == 0 || bit_count_first == 8 {
        messages_to_user.push(String::from("Paritet er gyldig"));
        if bit_count_first == 8 { bit_count_first = 1; } else { bit_count_first = 0; }
    } else {
        if bit_count_first >= 4 {
            messages_to_user.push(String::from("Paritet er ikke gyldig, men sat til 1"));
            bit_count_first = 1;
        } else {
            messages_to_user.push(String::from("Paritet er ikke gyldig, men sat til 0"));
            bit_count_first = 0;
        }
    }

    let bit_count = data.iter().map(|byte| byte.count_ones()).sum::<u32>();

    if (bit_count + bit_count_first) % 2 == 0 {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(String::from("Filen er ændret og minimum en fejl er fundet"));
        return messages_to_user;
    }
}


pub fn check_three_copies(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");

    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    let data_length = data.len() / 3;

    let data_first_bytes = data[0..data_length].to_vec();
    let data_second_bytes = data[data_length..data_length*2].to_vec();
    let data_third_bytes = data[data_length*2..data_length*3].to_vec();

    let mut new_data: Vec<u8> = Vec::with_capacity(data_length);

    let mut error_count = 0;

    for i in 0..data_length {
        let mut new_byte = 0u8;
        for bit in 0..8 {
            let mask = 1 << bit;
            let count = ((data_first_bytes[i] & mask) != 0) as u8 +
                        ((data_second_bytes[i] & mask) != 0) as u8 +
                        ((data_third_bytes[i] & mask) != 0) as u8;
            if count >= 2 {
                new_byte |= mask;
            }

            if count == 2 || count == 1 {
                error_count += 1;
            }
        }
        new_data.push(new_byte);
    }

    let temp_correct = String::from_utf8_lossy(&new_data);
    let temp_error = String::from_utf8_lossy(&data_first_bytes); // Retunere et tilfældig slice, for at vise dataen uden rettede fejl

    messages_to_user.push(format!("Filens data før rettelse ({}), efter rettelse ({})", temp_error, temp_correct));


    if data_first_bytes == data_second_bytes && data_second_bytes == data_third_bytes {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(format!("Filen er ændret og {} fejl er fundet", error_count));
        return messages_to_user;
    }
}

pub fn check_checksum(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");

    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    let checksum = data.last().unwrap();
    let data = &data[0..data.len()-1];

    let computed_checksum = compute_checksum(&data);

    messages_to_user.push(format!("Filens data før tjek for fejl ({})", String::from_utf8_lossy(&data).to_string()));
    messages_to_user.push(format!("Kontrolsum: {:08b}, Udregnet Kontrolsum: {:08b}", checksum, computed_checksum));

    if (computed_checksum & checksum) == 0b0000_0000 {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(String::from("Filen er ændret og minimum en fejl er fundet"));
        return messages_to_user;
    }
}
