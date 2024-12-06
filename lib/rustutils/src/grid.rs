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

    pub fn _print(&self) {
        for grid in &self.grid_vec {
            println!("{:?}",grid);
        }
        println!("{}","");
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
            println!("{:?}",grid);
        }
        println!("{}","");
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