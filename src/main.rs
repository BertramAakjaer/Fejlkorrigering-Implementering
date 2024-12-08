use std::io;
use std::fs;
use figlet_rs::FIGfont; // Til 

mod data_manipulator;
mod data_classes;

/*
    ** De fire fil typer der bliver brugt til fejlkorrigering **

    - Kan Tjekke For Fejl
    Paritets Bit : .pab
    Checksum : .ces

    - Kan Reparere Fejl
    3 Kopier : .3k
    Hamming Koder : .hak
*/

fn create_txt_file(files: &mut Vec<data_classes::DataHolder>, file_name: &str) -> () {

    let mut choice = String::new();

    io::stdin().read_line(&mut choice).expect("Kunne ikke indlæse input");
    let input_string: String = choice.trim().parse().expect("Det skal være bogstaver");
    println!();

    match data_manipulator::save_as_binary(&input_string, format!("output/{}.txt", &file_name).as_str()) {
        Ok(_) => {
            files.push(data_classes::DataHolder::new(format!("{}", &file_name)));
            files.last_mut().unwrap().add_main_file(format!("output/{}.txt", &file_name));
        },
        Err(e) => eprintln!("Denne data kunne ikke gemmes: {}", e),
    }

}

fn clear_all_files() -> () {
    let directory = "output";
    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.is_file() && !path.ends_with(".gitkeep") {
                    fs::remove_file(path).expect("Failed to delete file");
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory: {}", e),
    }
}

fn clear_console() -> () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn print_header() -> () {
    let text = "Fejlkorrigering";
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(text);
    if let Some(figure) = figure {
        println!("{}", figure);
    }
}

fn implement_all_error_correction(all_files: &mut Vec<data_classes::DataHolder>) -> (){
    match data_manipulator::apply_paritybit(all_files.last_mut().unwrap()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }
}

fn main() {
    clear_all_files();
    clear_console();
    print_header();

    loop {
        let mut all_files: Vec<data_classes::DataHolder> = vec![];

        let file_name = format!("data_file[{}]", all_files.len());
        let file_name = file_name.as_str();

        println!("\nSkriv en sætning der skal gemmes i filen :");

        create_txt_file(&mut all_files, &file_name);

    
        implement_all_error_correction(&mut all_files);


        println!("Enter the number of bits to change:");

        let mut bits_to_change = String::new();
        io::stdin().read_line(&mut bits_to_change).expect("Failed to read line");
        let bits_to_change: u32 = bits_to_change.trim().parse().expect("Please enter a valid number");

        println!(" ");



        match data_manipulator::random_bit_flipper(all_files.last_mut().unwrap().get_data_file_path("parity_bit"), bits_to_change) {
            Ok(_) => all_files.last_mut().unwrap().set_data_modified("parity_bit"),
            Err(e) => eprintln!("Failed to save data: {}", e),
        }
        
        println!("Parity bit check:");
        for i in data_manipulator::check_paritybit(all_files.last_mut().unwrap().get_data_file_path("parity_bit")) {
            println!("-\t{}", i);
        }
        break;
    }
}