use std::time;

use crate::copter;

pub struct Game
{
    pub clock: time::Instant,
    pub skip: f32,
    pub helicopter: copter::Copter,
}

impl Game
{
    pub fn new() -> Self
    {
        let fps: f32 = 1.;
        let clock = time::Instant::now();
        let skip: f32 = (1. / fps as f32) * 1_000_000_000.;
        let helicopter: copter::Copter = copter::Copter::new();

        Self
        {
            clock: clock,
            skip: skip,
            helicopter: helicopter,
        }
    }

    pub fn start(&mut self)
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

    pub fn render(&mut self)
    {
        let t = self.clock.elapsed();
        let total_skip = t.as_secs() * 1_000 + t.subsec_nanos() as u64;
        let diff = self.skip - (total_skip as f32);

        if diff > 0.
        {
            println!("{}", total_skip);

            self.helicopter.render();
            clearscreen::clear().unwrap();

            self.clock = time::Instant::now();
        }
    }
}
