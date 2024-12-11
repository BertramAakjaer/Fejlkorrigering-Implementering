use std::fs::File;
use std::io::{self, Write};
use std::io::Read;
use rand::Rng;

use crate::data_classes::DataHolder; // Importerer DataHolder structen fra data_classes.rs filen

/*
    FUNKTIONER TIL AT GEMME DATA LOKALT PÅ COMPUTEREN,
    ENTEN SOM REN DATA ELLER MED IMPLEMENTERET FEJLKORRIGERING
*/

// Gemmer en string som en binær fil (bruges til at gemme den originale data, som en .txt fil)
pub fn save_as_binary(data: &str, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

// Benytter paritetsbit til simpel fejlkorrigering, ved at benytte den første byte, som tilføjes
pub fn apply_paritybit(files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    
    let mut new_data = Vec::with_capacity(data.len() + 1);
    new_data.extend_from_slice(&data);
    
    // Tæller antal bits i dataen, og benytter den første byte til gøre antallet af bits lige (den første byte regnes som en bit)
    let bit_count = data.iter().map(|byte| byte.count_ones()).sum::<u32>();
    let parity_byte = if bit_count % 2 == 0 {
        0b0000_0000
    } else {
        0b1111_1111
    };
    
    new_data.insert(0, parity_byte);
    
    // Gemmes på computeren
    let mut file = File::create(format!("output/{}.pab", files.file_name))?;
    file.write_all(&new_data)?;
    
    // Gemmes i dataholder klassen, så det senere kan tilgås
    files.add_err_corr_data(String::from("parity_bit"), format!("output/{}.pab", files.file_name));

    Ok(())
}


// Benytter tre kopier af dataen, til at lave en fejlkorrigering
pub fn apply_three_copies(files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    // Skal bruge 3 gange så meget plads, som den originale data
    let mut new_data = Vec::with_capacity(data.len() * 3);
    
    // Kopierer dataen 3 gange
    for _ in 0..3 {
        new_data.extend_from_slice(&data);
    }
    
    // Gemmer dataen på computeren
    let mut file = File::create(format!("output/{}.3k", files.file_name))?;
    file.write_all(&new_data)?;
    
    // Gemmer dataen i dataholder klassen, så det senere kan tilgås
    files.add_err_corr_data(String::from("three_copies"), format!("output/{}.3k", files.file_name));

    Ok(())
}

// Gør det mere rent i koden at udrenge kontrolsummen adskilt fra hovedet funktionen
fn compute_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc: u8, byte| acc.wrapping_add(*byte))
}

// Benytter en kontrolsum til at lave en fejlkorrigering
pub fn apply_checksum(files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    // Udregner kontrolsummen
    let mut checksum = compute_checksum(&data);
    checksum = !checksum; // Benytter "1's complement", der gør det nemmere at sammenligne kontrolsummen med den udregnede kontrolsum
    
    // Gemmer kontrolsummen i dataen (Den er kun størrelsen af en byte)
    let mut new_data = Vec::with_capacity(data.len() + 1);
    new_data.extend_from_slice(&data);
    new_data.push(checksum);
    
    // Gemmer dataen på computeren
    let mut file = File::create(format!("output/{}.ces", files.file_name))?;
    file.write_all(&new_data)?;
    
    // Gemmer dataen i dataholder klassen, så det senere kan tilgås
    files.add_err_corr_data(String::from("checksum"), format!("output/{}.ces", files.file_name));
    
    Ok(())
}

// Benytter hamming koder til at lave en fejlkorrigering
pub fn apply_hamming_code (files: &mut DataHolder) -> io::Result<()> {
    let mut file = File::open(files.get_data_file_path("txt_file"))?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    /* 
        Hamming koder kan bruge 3 bit til fejlkorrigering i 4 bits
        og da vi har en paritets bit, får vi en byte, per 4 bits,
        så derfor i alt den dobbelte mængde data.
    */ 

    let mut new_data = Vec::with_capacity(data.len() * 2);
    
    let mut byte_cut_counter = 1; // Hjælper med at holde øje med hvilken side af byten der arbejdes mes

    for _ in 0..(data.len() * 2) {
        let left_part: bool;
        let byte_slice: u8;
        let mut new_byte = 0u8;
        
        if !(byte_cut_counter % 2 == 0) { // Sørger for at vi får den rigtige data inkodet
            byte_slice = data[(byte_cut_counter + 1) / 2 - 1];
            
            left_part = true;
        } else {
            byte_slice = data[byte_cut_counter / 2 - 1];
            
            left_part = false
        }
        
        // Skubber bits til den rigtige plads, lige meget hvilke side de var i
        if left_part {
            new_byte |= (byte_slice & 0b1111_0000) >> 1;
        } else {
            new_byte |= (byte_slice & 0b0000_1111) << 3;
        }
        
        // Udregner  E
        if !((new_byte & 0b0111_0000).count_ones() % 2 == 0){
            new_byte |= 0b0000_0100;
        }
        
        // Udregner G
        if !((new_byte & 0b0110_1000).count_ones() % 2 == 0){
            new_byte |= 0b0000_0010;
        }
        
        // Udregner F
        if !((new_byte & 0b0101_1000).count_ones() % 2 == 0){
            new_byte |= 0b0000_0001;
        }
        
        // Parity bit som den første bit
        if !(new_byte.count_ones() % 2 == 0) {
            new_byte |= 0b1000_0000;
        }
        
        new_data.push(new_byte);
        byte_cut_counter += 1; // Da vi gør vidre til næste "slice"
    }
    
    // Gemmer dataen på computeren
    let mut file = File::create(format!("output/{}.hak", files.file_name))?;
    file.write_all(&new_data)?;
    
    // Gemmer dataen i dataholder klassen, så det senere kan tilgås
    files.add_err_corr_data(String::from("hamming_code"), format!("output/{}.hak", files.file_name));
    
    Ok(())
}



/*
    FUNKTION TIL AT LAVE SYNTETISKE FEJL I DATAEN,
    MED ET SPECIFIKT ANTAL ÆNDRINGER, DER SKER I
    TILFÆLDIGE BITS I DATAEN.
*/

pub fn random_bit_flipper(data: &str, changes: u32) -> io::Result<()>{
    let mut file = File::open(data)?;
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)?;

    let mut rng = rand::thread_rng();

    for _ in 0..changes {
        let byte_index = rng.gen_range(0..bytes.len()); // Vælger en tilfældig byte
        let bit_index = rng.gen_range(0..8); // Vælger en tilfældig bit

        bytes[byte_index] ^= 1 << bit_index; // FLipper den bit
    }   
    let mut file = File::create(data)?;

    // Gemmer den nye data med fejl i stedet for den gamle
    file.write_all(&bytes)?;
    
    Ok(())
}



/*
    FUNKTIONER TIL AT GÅ TILBAGE I DATA DER ER INKODET
    MED FEJLKORRIGERING, OG KIGGE PÅ OM DER ER FEJL,
    OG EVENTUELT RETTE DEM.
*/

// Tjekker paritetbit for fejl
pub fn check_paritybit(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");
    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    
    let data_first_byte = data[0]; // Gemmer paritetsbit byten for sig selv
    let data = &data[1..]; // Tager resten af dataen
    
    let temp = String::from_utf8_lossy(&data); // Gemmer dataen som en string, selvom der kan være fejl

    // Viser dataen før fejltjek
    messages_to_user.push(format!("Filens data inden tjek for fejl ({})", temp));

    let mut bit_count_first = data_first_byte.count_ones();

    // Hvis den første byte er modificeret, så vælger den paritetsbiten ud fra dem der er flest af
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

    // Tæller antal bits i resten dataen
    let bit_count = data.iter().map(|byte| byte.count_ones()).sum::<u32>();

    // Addedere paritetsbiten og ser om tallet er lige
    if (bit_count + bit_count_first) % 2 == 0 {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(String::from("Filen er ændret og minimum en fejl er fundet"));
        return messages_to_user;
    }
}


// Tjekker tre kopier for fejl
pub fn check_three_copies(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");
    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    // Dataen er bare tre kopiere, så den kan komprimeres til en tredjedel, når vi kunne læse den
    let data_length = data.len() / 3;


    // De tre kopiere af dataen, adskilt i tre slices
    let data_first_bytes = data[0..data_length].to_vec();
    let data_second_bytes = data[data_length..data_length*2].to_vec();
    let data_third_bytes = data[data_length*2..data_length*3].to_vec();

    // Klargøre en variable til den rettede data
    let mut new_data: Vec<u8> = Vec::with_capacity(data_length);

    // Tæller antal fejl der rettes
    let mut error_count = 0;

    // Går igennem hver byte og sammenligner hver bit i de bytes
    for i in 0..data_length {
        let mut new_byte = 0u8; // Den nye rettede byte

        for bit in 0..8 { // Går over hver bit
            let mask = 1 << bit; // Opretter en maske med en enkelt bit sat til 1 i "bit" positionen

            // Tæller antal bits der er sat til 1 i de tre kopiere ved "bit" positionen
            let count = ((data_first_bytes[i] & mask) != 0) as u8 +
                        ((data_second_bytes[i] & mask) != 0) as u8 +
                        ((data_third_bytes[i] & mask) != 0) as u8;

            if count >= 2 { // Hvis der er to eller tre bits, der er 1, ved vi at det er en 1, der er gennemsnittet ellers vil det være 0
                new_byte |= mask;
            }

            // Hvis der er 1 eller 2 bits er der sket en fejl, da vores tre bits ikke er ens
            if count == 1 || count == 2 {
                error_count += 1;
            }
        }
        new_data.push(new_byte); // Gemmer den rettede byte
    }

    // Konverterer dataen til en string, så vi kan vise det til brugeren
    let temp_correct = String::from_utf8_lossy(&new_data);
    let temp_error = String::from_utf8_lossy(&data_first_bytes); // Retunere et tilfældig slice, for at vise dataen uden rettede fejl

    messages_to_user.push(format!("Filens data før rettelse ({}), efter rettelse ({})", temp_error, temp_correct));

    // Hvis dataen er ens er der ingen fejl
    if data_first_bytes == data_second_bytes && data_second_bytes == data_third_bytes {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(format!("Filen er ændret og {} fejl er fundet", error_count));
        return messages_to_user;
    }
}


// Tjekker kontrolsum for fejl
pub fn check_checksum(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");
    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    let checksum = data.last().unwrap(); // Kontrolsummen var gemt som den sidste byte
    let data = &data[0..data.len()-1];

    // Udregner kontrolsummen
    let computed_checksum = compute_checksum(&data);

    messages_to_user.push(format!("Filens data før tjek for fejl ({})", String::from_utf8_lossy(&data).to_string()));
    messages_to_user.push(format!("Kontrolsum: {:08b}, Udregnet Kontrolsum: {:08b}", checksum, computed_checksum));

    // Da vi brugte "1's complement" til at gemme kontrolsummen, kan vi sammenligne dem og hvis de er 100% forskellige er der ingen fejl
    if (computed_checksum & checksum) == 0b0000_0000 {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(String::from("Filen er ændret og minimum en fejl er fundet"));
        return messages_to_user;
    }
}


// Funktion til at rette en byte med hamming koder, da vi ellers skulle have den to gange i loopet
fn hamming_code_byte_corrector(byte: u8) -> u8 {
    let mut new_byte = 0u8;

    let mut e_even = true;
    let mut f_even = true;
    let mut g_even = true;

    // Hvis E cirklen ikke er lige, så er der sket en fejl
    if !((byte & 0b0111_0100).count_ones() % 2 == 0) {
        e_even = false;
    }

    // Hvis G cirklen ikke er lige, så er der sket en fejl
    if !((byte & 0b0110_1010).count_ones() % 2 == 0) {
        g_even = false;
    }

    // Hvis F cirklen ikke er lige, så er der sket en fejl
    if !((byte & 0b0101_1001).count_ones() % 2 == 0) {
        f_even = false;
    }

    // Hvis der er sket en fejl, så rettes den
    // Kan dog max rette en fejl per byte

    if !e_even && !g_even && !f_even {
        new_byte |= 0b0100_0000;
    } else if !e_even && !g_even {
        new_byte |= 0b0010_0000;
    } else if !e_even && !f_even {
        new_byte |= 0b0001_0000;
    } else if !g_even && !f_even {
        new_byte |= 0b0000_1000;
    } 
    
    // Tekniskset ligegyldigt med resten, da det smides væk !!
    
    /*  else if !e_even {
        new_byte |= 0b0000_0100;
    } else if !g_even {
        new_byte |= 0b0000_0010;
    } else if !f_even {
        new_byte |= 0b0000_0001;
    }
    */

    new_byte ^= byte;
    new_byte // Returnere den rettede byte, hvilket er den ene flippede bit indsat i den originale byte
}


// Tjekker hamming koder for fejl
pub fn check_hamming_code(input_path: &str) -> Vec<String> {
    let mut messages_to_user = Vec::new();

    let file = File::open(input_path).expect("Failed to open file");
    let data: Vec<u8> = file.bytes().map(|byte| byte.unwrap()).collect();

    // Dataen er dobbelt så stor, så vi kommer til at bruge det halve
    let data_length = data.len() / 2;

    // Den uændrede data gemmes her så vi kan vise det i konsollen
    let mut data_before_recovery: Vec<u8> = Vec::with_capacity(data_length);

    // Bruger tælleren til at holde styr på hvilken byte og side vi er i
    let mut byte_counter = 0;

    // Går igennem hver byte og gemmer den urettede data
    for _ in 0..data_length {
        let mut temp_byte = 0u8;

        temp_byte |= (data[byte_counter] & 0b0111_1000) << 1;

        byte_counter += 1;

        temp_byte |= (data[byte_counter] & 0b0111_1000) >> 3;

        byte_counter += 1;

        data_before_recovery.push(temp_byte);
    }
    
    // Gør klar til at gemme den rettede data
    let mut new_data: Vec<u8> = Vec::with_capacity(data_length);
    let mut error_counter = 0;

    // Nulstiller tælleren (Den initialiseres på ny, for at vise at loop'ene er uafhængige)
    let mut byte_counter = 0;

    // Vi går igennem to bytes i det indkodede data for hver iteration, da vi har to bytes per byte i den originale data vi skal rekreere
    for _ in 0..data_length {
        let mut temp_byte = 0u8; // Bliver den nye rettede data


        // Kigger på det der blir til venstre side af denne iteration
        let mut byte_left_side = data[byte_counter];

        if !byte_left_side.count_ones() % 2 == 0 { // Tjekker parietsbiten
            error_counter += 1;
        }

        // Bruger et funktionskald til at rette en bit, hvis der er fejl
        byte_left_side = hamming_code_byte_corrector(byte_left_side);

        byte_counter += 1;


        // Kigger på det der blir til højre side af denne iteration
        let mut byte_right_side = data[byte_counter];

        if !byte_right_side.count_ones() % 2 == 0 { // Tjekker paritetsbiten
            error_counter += 1;
        }

        // Bruger et funktionskald til at rette en bit, hvis der er fejl
        byte_right_side = hamming_code_byte_corrector(byte_right_side);


        // Samler de to bytes til en byte, ved at rykke dataen til den rigtige plads
        temp_byte |= (byte_left_side & 0b0111_1000) << 1;
        temp_byte |= (byte_right_side & 0b0111_1000) >> 3;

        new_data.push(temp_byte); // Gemmer den rettede byte i listen over det rettede data

        byte_counter += 1;
    }

    // Konverterer dataen til en string, så vi kan vise det til brugeren
    let temp_correct = String::from_utf8_lossy(&new_data);
    let temp_error = String::from_utf8_lossy(&data_before_recovery);
    
    messages_to_user.push(format!("Filens data før rettelse ({}), efter rettelse ({})", temp_error, temp_correct));


    if error_counter > 0 { // Udskriver antal fejl fundet via de paritetsbits der var
        messages_to_user.push(format!("Der er fundet {} paritetfejl i filen", error_counter));
    }

    // Hvis dataen er ens er der ikke sket fejl
    if new_data == data_before_recovery {
        messages_to_user.push(String::from("Filen er normal og indeholder ikke fejl."));
        return messages_to_user;
    } else {
        messages_to_user.push(format!("Filen er ændret og fejl er fundet"));
        return messages_to_user;
    }
}