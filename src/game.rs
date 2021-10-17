use std::time;
use crate::object;
use crate::copter;
use crate::wall;
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
        let fps: f32 = 2.;
        let clock = time::Instant::now();
        let skip: f32 = (1000. / fps as f32);
        let canvas: canvas::Canvas = canvas::Canvas::new();
        let mut objects: Vec<Box<dyn object::Object>> = vec![];

        objects.push(Box::new(copter::Copter::new()));
        objects.push(Box::new(wall::Wall::new()));

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

        if t.as_millis() >= self.skip as u128
        {
            self.canvas.clear();
            self.canvas.render(&self.objects);
            self.clock = time::Instant::now();
        }
    }
}
