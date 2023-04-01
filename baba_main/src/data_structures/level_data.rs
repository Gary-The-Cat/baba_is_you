use crate::data_structures::grid_configuration::GridConfiguration;
use crate::data_structures::enums::node::Node;

pub struct LevelData{
    pub grid: GridConfiguration,
    pub nodes: Vec<Node>,
}

impl LevelData{
    pub fn create_from_lines(lines: Vec<String>) ->  Result<LevelData, String> {
        if lines.len() == 0 {
            return Err("No lines were provided in the level file.".to_string());
        }
        else {
            let configuration_line = &lines[0];

            if let Ok(config) = try_get_width_height_from_line(configuration_line){
                let mut level = LevelData{
                    grid: GridConfiguration{
                        cell_size_x: 64.0,
                        cell_size_y: 64.0,
                        index_size_x: config.0,
                        index_size_y: config.1,
                    },
                    nodes: Vec::new(),
                };

                for (index, line) in lines.iter().skip(1).enumerate(){
                    let line_components: Vec<&str> = line.split(",").collect();

                    if line_components.len() != 4
                    {
                        return Err(format!("An error occurred while reading line {line}, please ensure it is in the correct format and try again.").to_string());
                    }

                    if let (Ok(x), Ok(y)) = (line_components[0].parse::<u32>(), line_components[1].parse::<u32>()){
                        if x >= config.0 || y >= config.1 {
                            return Err(format!("An error occurred while reading line {index}, please ensure your x and y are within the defined level grid size.").to_string());
                        }
                    }
                    else {
                    }
                }
                
            return Err("Not sure".to_string());
            }
            else
            {
                return Err("The configuration provided was invalid.".to_string());
            }
        }
    }
}

fn try_get_width_height_from_line(configuration_line: &String) -> Result<(u32, u32), String> {
    let components: Vec<&str> = configuration_line.split(",").collect();

    if (components.len() == 0)
    {
        return Err("No configuration was was provided. Please check file structure and try again.".to_string());
    }
    else if (components.len() != 2)
    {
        return Err("The configuration line must only consist of 'width,height'. Please check file structure and try again.".to_string());
    }
    else
    {
        if let (Ok(width), Ok(height)) = (components[0].parse::<u32>(), components[1].parse::<u32>()) {
            return Ok((width, height));
        }
        else {
            return Err("The values provided for 'width,height' must both be positive integers. Please check file structure and try again.".to_string());
        }
    }
}