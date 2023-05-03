use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input_level_path = "C:/dev/babalevels/level.csv".to_string();
    let output_level_path = "C:/dev/babalevels/level.json".to_string();

    let lines = backend::application::file_system::read_lines(input_level_path);
    let level_data_result = backend::data_structures::level_data::LevelData::create_from_lines(lines);
    
    let serialised = match level_data_result {
        Err(error) => {
            print!("{}", error);
            panic!()
        },
        Ok(level_data) => serde_json::to_string_pretty(&level_data).unwrap()
    };

    let mut file = File::create(output_level_path).unwrap();
    file.write_all(serialised.as_bytes());
}
