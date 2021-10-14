pub struct Copter
{
    pub position: u16,
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

    pub fn update(&self)
    {

    }

    pub fn render(&self)
    {
        println!("########");
        println!("########");
        println!("########");
        println!("########");
    }
}
