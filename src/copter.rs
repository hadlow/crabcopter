pub struct Copter
{
    pub position: i16,
}

impl Copter
{
    pub fn new() -> Self
    {
        Self
        {
            position: 5,
        }
    }

    pub fn print(&self)
    {
        println!("########");
        println!("########");
        println!("########");
        println!("########");
    }
}
