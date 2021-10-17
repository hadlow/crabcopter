pub trait Object
{
    fn get_position_x(&self) -> usize;
    fn get_position_y(&self) -> usize;
    fn get_sprite(&self) -> &Vec<Vec<char>>;
    fn update(&self);
}
