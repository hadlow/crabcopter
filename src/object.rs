pub trait Object
{
    fn get_position_x(&self) -> u16;
    fn get_position_y(&self) -> u16;
    fn get_sprite(&self) -> &Vec<Vec<char>>;
    fn update(&self);
}
