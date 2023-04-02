use std::fs::File;
use std::io::{ self, BufRead, BufReader };
use std::path::PathBuf;

pub fn read_lines_as_buffer(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
}

pub fn read_lines(file_path: String) -> Vec<String> {
    
    let mut output: Vec<String> = Vec::new();

    let lines = read_lines_as_buffer(file_path);

    for line in lines{
        output.push(line.unwrap());
    } 

    return output;
}

pub fn get_project_root() -> String{
    return PathBuf::from(env!("CARGO_MANIFEST_DIR")).display().to_string();
}

pub fn get_asset_path(asset_name: String) -> String{
    let mut path = get_project_root();
    path.push('\\');
    path.push_str("assets");
    path.push('\\');
    path.push_str(asset_name.as_str());
    return path;
}

pub fn get_level_path(level_name: String) -> String{
    let mut path = get_project_root();
    path.push('\\');
    path.push_str("levels");
    path.push('\\');
    path.push_str(level_name.as_str());
    return path;
}