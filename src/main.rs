use std::io;
use std::fs;
use figlet_rs::FIGfont; // Til overskrift der vises i konsollen

// De andre filer indlæses
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

// Funktion til at oprette den originale fil, der holder uændret data
fn create_txt_file(files: &mut Vec<data_classes::DataHolder>, file_name: &str) -> () {
    let mut choice = String::new();

    // Læser input fra brugeren, til at gemme i filen
    io::stdin().read_line(&mut choice).expect("Kunne ikke indlæse input");
    let input_string: String = choice.trim().parse().expect("Det skal være bogstaver");
    println!();

    // Laver en ny objekt af DataHolder og tilføjer den til listen
    match data_manipulator::save_as_binary(&input_string, format!("output/{}.txt", &file_name).as_str()) {
        Ok(_) => {
            files.push(data_classes::DataHolder::new(format!("{}", &file_name)));
            files.last_mut().unwrap().add_main_file(format!("output/{}.txt", &file_name));
        },
        Err(e) => eprintln!("Denne data kunne ikke gemmes: {}", e),
    }
}

// Sletter alle filer i output mappen
fn clear_all_files() -> () {
    let directory = "output";

    match fs::read_dir(directory) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.expect("Filen kunne ikke læses");
                let path = entry.path();
                if path.is_file() && !path.ends_with(".gitkeep") {
                    fs::remove_file(path).expect("Filen kunne ikke slettes");
                }
            }
        }
        Err(e) => eprintln!("Kunne ikke læse mappen: {}", e),
    }
}

// Rydder konsollen
fn clear_console() -> () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

// Printer overskriften, ved brug af figlet
fn print_header() -> () {
    let text = "Fejlkorrigering";
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(text);
    if let Some(figure) = figure {
        println!("{}", figure);
    }
}


// Samler alle funktionkald til at implementere fejlkorrigering 
fn implement_all_error_correction(all_files: &mut Vec<data_classes::DataHolder>) -> (){
    match data_manipulator::apply_paritybit(all_files.last_mut().unwrap()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::apply_three_copies(all_files.last_mut().unwrap()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::apply_checksum(all_files.last_mut().unwrap()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::apply_hamming_code(all_files.last_mut().unwrap()) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }
}


// Samler alle funktionkald til at flippe bits i fejlkorrigerings filerne
fn flip_bits(all_files: &mut Vec<data_classes::DataHolder>, bits_to_change: u32) -> () {
    match data_manipulator::random_bit_flipper(all_files.last_mut().unwrap().get_data_file_path("parity_bit"), bits_to_change) {
        Ok(_) => all_files.last_mut().unwrap().set_data_modified("parity_bit"),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::random_bit_flipper(all_files.last_mut().unwrap().get_data_file_path("three_copies"), bits_to_change) {
        Ok(_) => all_files.last_mut().unwrap().set_data_modified("three_copies"),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::random_bit_flipper(all_files.last_mut().unwrap().get_data_file_path("checksum"), bits_to_change) {
        Ok(_) => all_files.last_mut().unwrap().set_data_modified("checksum"),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }

    match data_manipulator::random_bit_flipper(all_files.last_mut().unwrap().get_data_file_path("hamming_code"), bits_to_change) {
        Ok(_) => all_files.last_mut().unwrap().set_data_modified("hamming_code"),
        Err(e) => eprintln!("Failed to save data: {}", e),
    }
}


/*
    Fire funktioner der tjekker om fejlkorrigeringen har virket,
    ved at udnytte det der blev implementeret tidligere, her 
    kalder funktioner i data_manipulator.rs
*/

fn show_and_check_paritybit(all_files: &mut Vec<data_classes::DataHolder>) -> () {
    println!("Paritets bit tjek :");
    for i in data_manipulator::check_paritybit(all_files.last_mut().unwrap().get_data_file_path("parity_bit")) {
        println!("-\t{}", i);
    }
}

fn show_and_check_three_copies(all_files: &mut Vec<data_classes::DataHolder>) -> () {
    println!("Tre kopier tjek :");
    for i in data_manipulator::check_three_copies(all_files.last_mut().unwrap().get_data_file_path("three_copies")) {
        println!("-\t{}", i);
    }
}

fn show_and_check_checksum(all_files: &mut Vec<data_classes::DataHolder>) -> () {
    println!("Kontrolsum tjek :");
    for i in data_manipulator::check_checksum(all_files.last_mut().unwrap().get_data_file_path("checksum")) {
        println!("-\t{}", i);
    }
}

fn show_and_check_hamming_code(all_files: &mut Vec<data_classes::DataHolder>) -> () {
    println!("Hamming kode tjek :");
    for i in data_manipulator::check_hamming_code(all_files.last_mut().unwrap().get_data_file_path("hamming_code")) {
        println!("-\t{}", i);
    }
}


// Main funktionen, der looper til det bliver stopper
fn main() {

    // Rydder konsollen og sletter tidligere filer
    clear_all_files();
    clear_console();
    print_header();

    let mut iteration_count = 0;


    // Loop der kører programmet
    loop {
        let mut all_files: Vec<data_classes::DataHolder> = vec![];

        let file_name = format!("data_file[{}]", iteration_count);
        let file_name = file_name.as_str();

        println!("\nSkriv en sætning der skal gemmes i filen :");

        create_txt_file(&mut all_files, &file_name);

    
        implement_all_error_correction(&mut all_files);

        


        println!("Skriv det antal bits der skal ændres :");

        let mut bits_to_change = String::new();
        io::stdin().read_line(&mut bits_to_change).expect("Failed to read line");
        let bits_to_change: u32 = bits_to_change.trim().parse().expect("Please enter a valid number");

        println!(" ");


        println!("###########################################################");
        println!("##  Benytter fejlkorrigering på data efter flippet bits  ##");
        println!("###########################################################");
        println!(" ");


        // flipper tilfældige bits i de forskellige filer
        flip_bits(&mut all_files, bits_to_change);


        // Viser alle fejlkorrigeringernes resultater

        show_and_check_paritybit(&mut all_files);

        println!(" ");

        show_and_check_checksum(&mut all_files);
        
        println!(" ");
        
        show_and_check_three_copies(&mut all_files);

        println!(" ");

        show_and_check_hamming_code(&mut all_files);



        println!("\n**Vil du fortsætte (Y/n)** :"); // Spørger bruger om loop'et skal stoppes

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: String = choice.trim().parse().expect("Please enter a valid number");

        if choice.to_lowercase() == "n" {
            break;
        }

        // Bruges til at holde styr på iterationer, så de ældre filer også gemmes og ikke overskrives
        iteration_count += 1;

        clear_console();
    }
}