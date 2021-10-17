use crate::object;

pub struct Copter
{
    position_x: usize,
    position_y: usize,
    sprite: Vec<Vec<char>>,
}

impl object::Object for Copter
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
            vec![' ', ' ', ' ', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
            vec!['(', ')', '_', '_', '_', '_', ',', '-', '\'', '-', '.', ' '],
            vec![' ', '"', '"', '"', '-', '-', '.', '_', '_', '_', '_', ')'],
            vec![' ', ' ', ' ', ' ', ' ', ' ', '-', '\'', '-', '-', '\'', '-'],
        ]
    }
}
