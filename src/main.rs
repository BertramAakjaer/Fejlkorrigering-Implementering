use std::io::{self, Write};
use std::fs::File;

mod data_manipulator;






fn create_file(){

    print!("\nSkriv en sætning der skal gemmes i filen");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let input_string: String = choice.trim().parse().expect("Please enter a string");
    println!();

    match data_manipulator::save_as_binary(&input_string, "1.txt") {
        Ok(_) => println!("Data saved successfully."),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

}




fn choose_errcorr_method() {
    let choices = vec![
        "Duppler",
        "Parity Bit",
        "Checksum",
        "Hamming Kode",
        "CRC",
        "FEC",
        "Reed-Solomon",
        "Viterbi",
        "LDPC",
        "Convolutional Kode",
        "Turbo Kode",
        "BCH",
        "RS",
        "Reed-Muller",
    ];

    loop {
        println!("\nVælg en af følgende metoder (0 for exit):");
        for (i, choice) in choices.iter().enumerate() {
            println!("\t- [{}] {}", i + 1, choice);
        }
        print!("\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");
        println!();

        match choice {
            0 => {
                break;
            }
            1 => {
                
            }
            _ => {
                println!("Valget matchede ikke noget prøv igen.");
            }
        }
    }
}


fn check_for_error() -> bool {
    return false;

}


fn fix_error() {
    if check_for_error() {
        println!("Fejl fundet");
    } else {
        println!("Ingen fejl fundet");
    }
}




fn main() {
    let choices = vec![
        "Opret en fil",
        "Implementer fejlkorrigering",
        "Check for fejl",
        "Check og Ret fejlen"
    ];

    loop {
        println!("\nVælg en af følgende muligheder:");
        for (i, choice) in choices.iter().enumerate() {
            println!("\t- [{}] {}", i + 1, choice);
        }
        print!("\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");
        println!();

        match choice {
            1 => {
                create_file();
            }
            2 => {
                choose_errcorr_method();
            }
            3 => {
                check_for_error();
            }
            4 => {
                fix_error();
            }
            _ => {
                println!("Valget matchede ikke noget prøv igen.");
            }
        }
    }
}