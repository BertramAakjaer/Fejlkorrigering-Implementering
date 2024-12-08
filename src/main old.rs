use std::io;
use std::fs;
use std::path::Path;


mod data_manipulator;
mod data_classes;

/*
    3 Kopier : .3k
    Paritets Bit : .pab
    Checksum : .ces
    Hamming Kode : .hak
*/

fn create_file(files: &mut Vec<data_classes::DataHolder>) -> () {

    println!("\nSkriv en sætning der skal gemmes i filen :");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let input_string: String = choice.trim().parse().expect("Please enter a string");
    println!();

    match data_manipulator::save_as_binary(&input_string, format!("output/{}.txt", &input_string.chars().take(8).collect::<String>()).as_str()) {
        Ok(_) => {
            println!("Data saved successfully.");
            files.push(data_classes::DataHolder::new(format!("{}", &input_string.chars().take(8).collect::<String>())));
            files.last_mut().unwrap().add_main_file(format!("output/{}.txt", &input_string.chars().take(8).collect::<String>()));
        },
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

}


fn choose_errcorr_tool(file: &str) {
    let choices = vec![
        "3 Kopier",
        "Paritets Bit",
        "Checksum",
        "Hamming Kode",
    ];

    /*
        "CRC",
        "FEC",
        "Reed-Solomon",
        "Viterbi",
        "LDPC",
        "Convolutional Kode",
        "Turbo Kode",
        "BCH",
        "RS",
    */

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


fn choose_method(file: &str) {
    let choices = vec![
        "Implementer fejlkorrigering",
        "Check for fejl",
        "Check og Ret fejlen"
    ];

    loop {
        print!("Fil valgt: {}", file);
        println!("\nVælg en af værktøjer (0 for exit):");
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
                choose_errcorr_tool(file);
            }
            2 => {
                check_for_error();
            }
            3 => {
                fix_error();
            }
            _ => {
                println!("Valget matchede ikke noget prøv igen.");
            }
        }
    }
}


fn get_user_input() -> () {
    let choices = vec![
        "Opret en fil",
        "Vis filer + deres data",
        "Vælg en fil og manipuler den",
    ];

    let mut all_data: Vec<data_classes::DataHolder> = vec![];

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
                create_file(&mut all_data); 
            }
            2 => {
                //choose_errcorr_method();
            }
            3 => {
                println!("\nVælg en af følgende filer:");
                let files = get_files_in_directory("output").unwrap();
                for (i, files) in files.iter().enumerate() {
                    println!("\t- [{}] {}", i + 1, files);
                }
                print!("\n");

                let mut index_files = String::new();
                io::stdin().read_line(&mut index_files).expect("Failed to read line");
                let index_files: u32 = index_files.trim().parse().expect("Please enter a number");
                println!();

                let file: &String = files.get(index_files as usize - 1).unwrap();

                choose_method(file);

            }
            _ => {
                println!("Valget matchede ikke noget prøv igen.");
            }
        }
    }
}

fn get_files_in_directory(directory: &str) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();
    let path = Path::new(directory);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.to_str() {
                    files.push(file_name.to_string());
                }
            }
        }
    }
    Ok(files)
}


fn main() {
    return ();
    /*
    let directory = "output";
    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(path).expect("Failed to delete file");
                }
            }
            println!("All files in the directory have been deleted.");
        }
        Err(e) => eprintln!("Failed to read directory: {}", e),
    }

    get_user_input();

    */

    let a = "output/as.txt";
    let b = "output/as.pab";


    match data_manipulator::apply_paritybit(a, b) {
        Ok(_) => {
            println!("Data saved successfully.");
        },
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::bit_manipulator(b, 1) {
        Ok(_) => {
            println!("Data modified succesfully.");
        },
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    println!("{}", data_manipulator::check_paritybit(b));
}