use std::time;
use crate::object;
use crate::copter;
use crate::canvas;

pub struct Game
{
    clock: time::Instant,
    skip: f32,
    canvas: canvas::Canvas,
    objects: Vec<Box<dyn object::Object>>,
}

impl Game
{
    pub fn new() -> Self
    {
        let fps: f32 = 1.;
        let clock = time::Instant::now();
        let skip: f32 = (1. / fps as f32) * 1_000_000_000.;
        let canvas: canvas::Canvas = canvas::Canvas::new();
        let objects: Vec<Box<dyn object::Object>> = vec![Box::new(copter::Copter::new())];

        Self
        {
            clock: clock,
            skip: skip,
            canvas: canvas,
            objects: objects,
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
        for object in &self.objects
        {
            object.update();
        }
    }

    pub fn render(&mut self)
    {
        let t = self.clock.elapsed();
        let total_skip = t.as_secs() * 1_000 + t.subsec_nanos() as u64;
        let diff = self.skip - (total_skip as f32);

        if diff > 0.
        {
            self.canvas.render(&self.objects);
            self.canvas.clear();
            self.clock = time::Instant::now();
        }
    }
}
