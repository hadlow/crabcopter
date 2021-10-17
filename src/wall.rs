use crate::object;

pub struct Wall
{
    position_x: usize,
    position_y: usize,
    sprite: Vec<Vec<char>>,
}

impl object::Object for Wall
{
    fn get_position_x(&self) -> usize
    {
        self.position_x
    }

    fn get_position_y(&self) -> usize
    {
        self.position_y
    }

    fn get_sprite(&self) -> &Vec<Vec<char>>
    {
        &self.sprite
    }

    fn update(&mut self)
    {
        self.position_x = self.position_x + 1;
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
