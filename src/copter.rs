use crate::object;

pub struct Copter
{
    position_x: u16,
    position_y: u16,
    sprite: Vec<Vec<char>>,
}

impl object::Object for Copter
{
    fn get_position_x(&self) -> u16
    {
        self.position_x
    }

    fn get_position_y(&self) -> u16
    {
        self.position_y
    }

    fn get_sprite(&self) -> Vec<Vec<char>>
    {
        self.sprite
    }

    fn update(&self)
    {

    }
}

impl Copter
{
    pub fn new() -> Self
    {
        Self
        {
            position_x: 25,
            position_y: 15,
            sprite: Copter::make_sprite(),
        }
    }

    fn make_sprite() -> Vec<Vec<char>>
    {
        //_________
        //  ,-'-.____()
        // (____.--"""
        // -'--'-

        vec![
            vec![' ', ' ', ' ', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['(', ')', '_', '_', '_', '_', ',', '-', '\'', '-', '.', ' '],
            vec![' ', '"', '"', '"', '-', '-', '.', '_', '_', '_', '_', ')'],
            vec![' ', ' ', ' ', ' ', ' ', ' ', '-', '\'', '-', '-', '\'', '-'],
        ]
    }
}
