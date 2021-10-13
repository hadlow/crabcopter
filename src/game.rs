use crate::copter;

pub struct Game
{
    pub helicopter: copter::Copter,
}

impl Game
{
    pub fn new() -> Self
    {
        let helicopter: copter::Copter = copter::Copter::new();

        Self
        {
            helicopter: helicopter,
        }
    }

    pub fn start(&self)
    {
        loop
        {
            self.update();
            self.render();
        }
    }

    pub fn update(&self)
    {
        self.helicopter.update();
    }

    pub fn render(&self)
    {
        self.helicopter.render();
    }
}
