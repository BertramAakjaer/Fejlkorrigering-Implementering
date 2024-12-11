// Klasse der holder styr på de filer med fejlkorrigering der hænger sammen
pub struct DataHolder {

    pub file_name: String, // Navnet kan nemt tilgås

    // Primary variabler
    txt_file: DataType,
    three_copies: DataType,
    parity_bit: DataType,
    checksum: DataType,
    hamming_code: DataType,
}

impl DataHolder { // Funktioner til DataHolder klassen

    pub fn new(file_name: String) -> DataHolder { // Kontruktør
        DataHolder {
            file_name,

            txt_file: DataType::new(),
            three_copies: DataType::new(),
            parity_bit: DataType::new(),
            checksum: DataType::new(),
            hamming_code: DataType::new(),
        }
    }

    // TIlføj en ny hovedet fil
    pub fn add_main_file(&mut self, path: String) {
        self.txt_file.populate_data("txt_file", path.as_str());
    }

    // Tilføj en ny fil med fejlkorrigerin implementeret og gem den som en klasse med navn og sti
    pub fn add_err_corr_data(&mut self, key: String, path: String) {

        match key.as_str() { // Tager en string som input og matcher den med en fejlkorrigerings type
            "three_copies" => self.three_copies.populate_data(&key, &path),
            "parity_bit" => self.parity_bit.populate_data(&key, &path),
            "checksum" => self.checksum.populate_data(&key, &path),
            "hamming_code" => self.hamming_code.populate_data(&key, &path),

            _ => (),
        }
    }

    // Hent stien til en fil, ud fra typen
    pub fn get_data_file_path(&self, key: &str) -> &str {

        match key { // Tager en string som input og matcher den med en fejlkorrigerings type
            "txt_file" => &self.txt_file.get_file_path(),
            "three_copies" => self.three_copies.get_file_path(),
            "parity_bit" => self.parity_bit.get_file_path(),
            "checksum" => self.checksum.get_file_path(),
            "hamming_code" => self.hamming_code.get_file_path(),

            _ => "",
        }
    }

    // Ændre variablen error_applied til true, så vi ved at der er blevet ændret i filen
    pub fn set_data_modified(&mut self, key: &str) {

        match key { // Tager en string som input og matcher den med en fejlkorrigerings type
            "txt_file" => self.txt_file.set_error_applied(),
            "three_copies" => self.three_copies.set_error_applied(),
            "parity_bit" => self.parity_bit.set_error_applied(),
            "checksum" => self.checksum.set_error_applied(),
            "hamming_code" => self.hamming_code.set_error_applied(),

            _ => (),
        }
    }
}


// Klasse der holder typen af fil, som fx "parity_bit" og stien til filen, samt om den er blevet ændret ved at flippe bits
pub struct DataType {
    // Indeholder kun private variabler
    file_type: String,
    file_path: String,
    error_applied: bool
}

impl DataType { // Funktioner til DataType klassen

    pub fn new() -> DataType {

        DataType { // Konstruktør
            file_type: String::new(),
            file_path: String::new(),
            error_applied: false,
        }
    }

    // Opsætter en ny fil med dens fejlkorrektionstype og sti
    pub fn populate_data(&mut self, file_type: &str, file_path: &str) {
        self.file_type = file_type.to_string();
        self.file_path = file_path.to_string();
    }

    // Retunerer stien til filen
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    // Sætter error_applied til true
    pub fn set_error_applied(&mut self) {
        self.error_applied = true;
    }
}