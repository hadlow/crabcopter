use terminal_size::{Width, Height, terminal_size};

use crate::object;
use crate::copter;

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
        
    }

    pub fn clear(&self)
    {
        clearscreen::clear().unwrap();
    }
}
