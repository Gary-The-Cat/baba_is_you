use bevy::prelude::*;
use super::grid_configuration::GridConfiguration;

// TODO, This seems like the concept of a 2d point should be used.
#[derive(Resource)]
pub struct Grid {
    pub configuration: GridConfiguration,
    pub centre: (f32, f32),
}

impl Grid {
    pub fn width(&self) -> f32 {
        self.configuration.cell_size_x * self.configuration.index_size_x as f32
    }
    
    pub fn height(&self) -> f32 {
        self.configuration.cell_size_y * self.configuration.index_size_y as f32
    }
    
    pub fn cell_count(&self) -> u32 {
        self.configuration.index_size_x * self.configuration.index_size_y
    }

    pub fn lower_left(&self) -> (f32, f32) {
        let x = self.centre.0 - self.width() / 2.0;
        let y = self.centre.1 - self.height() / 2.0;

        return (x, y);
    }
    
    pub fn cell_top_left_from_index(&self, (x, y): (u32, u32)) -> Option<(f32, f32)> {
    
        // The cell position is outside of the grid
        if x > self.configuration.index_size_x || y > self.configuration.index_size_y {
            return None
        }

        let lower_left = self.lower_left();
    
        return Some((lower_left.0 + x as f32 * self.configuration.cell_size_x, lower_left.1 + y as f32 * self.configuration.cell_size_y))
    }
    
    pub fn get_cell_from_position(&self, (x, y): (u32, u32)) -> Option<(u32, u32)> {
        let is_above_grid = x >= self.width() as u32 || y >= self.height() as u32;

        if is_above_grid {
            return None;
        }
        else {
            
            let lower_left = self.lower_left();
            return Some((
                (((x as f32 - lower_left.0) / self.width()) * self.configuration.index_size_x as f32) as u32,
                (((y as f32 - lower_left.1) / self.height()) * self.configuration.index_size_y as f32) as u32))
        }
    }
}

