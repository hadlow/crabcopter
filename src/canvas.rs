use terminal_size::{Width, Height, terminal_size};

use crate::object;

pub struct Canvas
{
    width: u16,
    height: u16,
}

impl Canvas
{
    pub fn new() -> Self
    {
        let size = terminal_size();

        if let Some((Width(width), Height(height))) = size {
            return Self
            {
                width: width,
                height: height,
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
        let mut buffer: Vec<Vec<char>> = vec![
            vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd',],
        ];

        for object in objects
        {
            object.get_position_x();
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
