use std::io::{self,Write};
use std::fs::OpenOptions;


#[derive(Clone)]
pub struct Grid {
    pub grid_vec: Vec<Vec<String>>,
}

impl Grid {
    pub fn _height(&self) -> usize {
        return self.grid_vec.len();
    }

    pub fn _width(&self) -> usize {
        return self.grid_vec[0].len();
    }

    pub fn grid_vec(&self) -> Vec<Vec<String>> {
        return self.grid_vec.clone();
    }

    pub fn _elem(&self, x: i32, y: i32) -> String {
        let point_char = ".";

        if x < 0 || y < 0 {
            return point_char.to_string();
        }

        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;

        if x_usize > (self._width() -1) || y_usize > (self._height()-1) {
            return point_char.to_string();
        }

        return self.grid_vec[y_usize][x_usize].clone();
    }

    pub fn _up(&self, x :i32,y:i32) -> String {
        self._elem(x,y-1)
    }

    pub fn _up_up(&self, x :i32,y:i32) -> String {
        self._elem(x,y-2)
    }

    pub fn _down(&self, x :i32,y:i32) -> String {
        self._elem(x,y+1)
    }
    
    pub fn _down_down(&self, x :i32,y:i32) -> String {
        self._elem(x,y+2)
    } 
    
    pub fn _left(&self, x :i32,y:i32) -> String {
        self._elem(x-1,y)
    }

    pub fn _left_left(&self, x :i32,y:i32) -> String {
        self._elem(x-2, y)
    }

    pub fn _right(&self, x :i32,y:i32) -> String {
        self._elem(x+1,y)
    }

    pub fn _right_right(&self, x :i32,y:i32) -> String {
        self._elem(x+2,y)
    }
    pub fn _set(&mut self, x: i32, y: i32, value: i32) {
        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;
        self.grid_vec[y_usize][x_usize] = value.to_string();
    }

    pub fn _set_str(&mut self, x: i32, y: i32, value: String) {
        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;
        self.grid_vec[y_usize][x_usize] = value;
    }

    pub fn _is_within_grid(&self, x:i32, y:i32) -> bool {
        x < self._width() as i32 && x >= 0 && y < self._height() as i32 && y >= 0
    }

    pub fn _print(&self) {
        for grid in &self.grid_vec {
            for val in grid {
                print!("{}", val);
            }
            println!();
        }
    }

    pub fn _print_special(&self, it:String, count:i32)-> io::Result<()> {

        let mut file = OpenOptions::new()
        .append(true)       // Open for appending
        .create(true)       // Create the file if it doesn't exist
        .open("example.txt")?; // Open the file
        
        writeln!(file, "Move {} count: {}",it,count)?;
        for grid in &self.grid_vec {
                let result: String = grid.iter().map(|num| num.to_string()).collect();
                // fs::write("output.txt",result);
                writeln!(file, "{}", result)?;
            }
        writeln!(file, "")?;
        Ok(())
    }
    
    pub fn _create_visiter_grid(&self) -> Gridi32{
        let rows = self._width(); 
        let cols = self._height();
        Gridi32 { grid_vec: vec![vec![0; rows]; cols]}
    }
}

#[derive(Clone)]
pub struct Gridi32 {
    pub grid_vec: Vec<Vec<i32>>,
}

impl Gridi32 {
    pub fn _height(&self) -> usize {
        return self.grid_vec.len();
    }

    pub fn _width(&self) -> usize {
        return self.grid_vec[0].len();
    }

    pub fn _elem(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;

        if x_usize > (self._width() -1) || y_usize > (self._height()-1) {
            return 0;
        }

        return self.grid_vec[y_usize][x_usize].clone();
    }

    pub fn _set(&mut self, x: i32, y: i32, value: i32) {
        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;
        self.grid_vec[y_usize][x_usize] = value;
    }

    pub fn _print(&self) {
        for grid in &self.grid_vec {
            for val in grid {
                print!("{:7}", val);
            }
            println!();
        }
    }

    pub fn _print_special(&self)-> io::Result<()> {
        let mut file = OpenOptions::new()
        .append(true)       // Open for appending
        .create(true)       // Create the file if it doesn't exist
        .open("example.txt")?; // Open the file
        
        
        writeln!(file)?;
        for grid in &self.grid_vec {
                let result: String = grid.iter().map(|num| num.to_string()).collect();
                // fs::write("output.txt",result);
                writeln!(file, "{:7}", result)?;
            }
        writeln!(file, "")?;
        Ok(())
    }

    pub fn _max(&self) -> Option<i32> {
        let max_value = self.grid_vec
        .iter()
        .flatten()
        .max()
        .cloned();
        
        return max_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elem() {
        let rows = 4;
        let cols = 4;
        let vec = vec![vec![9; cols]; rows];
        let gridi32 = Gridi32{grid_vec:vec};
        assert_eq!(gridi32._elem(1, 2),9);
    }

    #[test]
    fn set_elem() {
        let rows = 4;
        let cols = 4;
        let vec = vec![vec![9; cols]; rows];
        let mut gridi32 = Gridi32{grid_vec:vec};
        gridi32._set(0, 3, 23);
        assert_eq!(gridi32._elem(0, 3), 23);
    }
}