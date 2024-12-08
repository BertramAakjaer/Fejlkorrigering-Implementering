pub struct DataHolder {

    pub file_name: String,

    txt_file: DataType,
    three_copies: DataType,
    parity_bit: DataType,
    checksum: DataType,
    hamming_code: DataType,
}

impl DataHolder {

    pub fn new(file_name: String) -> DataHolder {

        DataHolder {
            file_name,

            txt_file: DataType::new(),
            three_copies: DataType::new(),
            parity_bit: DataType::new(),
            checksum: DataType::new(),
            hamming_code: DataType::new(),
        }
    }

    pub fn add_main_file(&mut self, path: String) {
        self.txt_file.populate_data("txt_file", path.as_str());
    }

    pub fn add_err_corr_data(&mut self, key: String, path: String) {
        match key.as_str() {
            "three_copies" => self.three_copies.populate_data(&key, &path),
            "parity_bit" => self.parity_bit.populate_data(&key, &path),
            "checksum" => self.checksum.populate_data(&key, &path),
            "hamming_code" => self.hamming_code.populate_data(&key, &path),
            _ => (),
        }
    }

    pub fn get_data_file_path(&self, key: &str) -> &str {
        match key {
            "txt_file" => &self.txt_file.get_file_path(),
            "three_copies" => self.three_copies.get_file_path(),
            "parity_bit" => self.parity_bit.get_file_path(),
            "checksum" => self.checksum.get_file_path(),
            "hamming_code" => self.hamming_code.get_file_path(),
            _ => "",
        }
    }

    pub fn set_data_modified(&mut self, key: &str) {
        match key {
            "txt_file" => self.txt_file.set_error_applied(),
            "three_copies" => self.three_copies.set_error_applied(),
            "parity_bit" => self.parity_bit.set_error_applied(),
            "checksum" => self.checksum.set_error_applied(),
            "hamming_code" => self.hamming_code.set_error_applied(),
            _ => (),
        }
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
}


pub struct DataType {

    file_type: String,
    file_path: String,
    error_applied: bool
}

impl DataType {
    pub fn new() -> DataType {
        DataType {
            file_type: String::new(),
            file_path: String::new(),
            error_applied: false,
        }
    }

    pub fn populate_data(&mut self, file_type: &str, file_path: &str) {
        self.file_type = file_type.to_string();
        self.file_path = file_path.to_string();
    }

    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn set_error_applied(&mut self) {
        self.error_applied = true;
    }

}