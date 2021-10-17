use terminal_size::{Width, Height, terminal_size};

use crate::object;

pub struct Canvas
{
    width: usize,
    height: usize,
}

impl Canvas
{
    pub fn new() -> Self
    {
        let size = terminal_size();

        if let Some((Width(width), Height(height))) = size {
            return Self
            {
                width: width.into(),
                height: height.into(),
            };
        }

        Self
        {
            width: 100,
            height: 100,
        }
    }

    pub fn render(&self, objects: &Vec<Box<dyn object::Object>>)
    {
        let mut buffer: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        for object in objects
        {
            buffer[object.get_position_y()][object.get_position_x()] = '1';
            for (r_index, row) in object.get_sprite().iter().enumerate()
            {
                for (c_index, column) in row.iter().enumerate()
                {
                    buffer[object.get_position_y() + r_index][object.get_position_x() + c_index] = *column;
                }
            }
        }

        for row in buffer.iter()
        {
            let s: String = row.into_iter().collect();
            println!("{}", s);
        }
    }

    pub fn clear(&self)
    {
        clearscreen::clear().unwrap();
    }
}
