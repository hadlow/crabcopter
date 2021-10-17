use crate::object;

pub struct Wall
{
    position_x: u16,
    position_y: u16,
    sprite: Vec<Vec<char>>,
}

impl object::Object for Wall
{
    fn get_position_x(&self) -> u16
    {
        self.position_x
    }

    fn get_position_y(&self) -> u16
    {
        self.position_y
    }

    fn get_sprite(&self) -> &Vec<Vec<char>>
    {
        &self.sprite
    }

    fn update(&self)
    {

    }
}

impl Wall
{
    pub fn new() -> Self
    {
        Self
        {
            position_x: 100,
            position_y: 2,
            sprite: Wall::make_sprite(),
        }
    }

    fn make_sprite() -> Vec<Vec<char>>
    {
        vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#'],
        ]
    }
}
