use std::time;
use crate::object;
use crate::copter;
use crate::wall;
use crate::canvas;

pub struct Game
{
    render_clock: time::Instant,
    update_clock: time::Instant,
    render_skip: f32,
    update_skip: f32,
    canvas: canvas::Canvas,
    objects: Vec<Box<dyn object::Object>>,
}

impl Game
{
    pub fn new() -> Self
    {
        let render_clock = time::Instant::now();
        let update_clock = time::Instant::now();
        let render_skip: f32 = 1000. / 10. as f32;
        let update_skip: f32 = 1000. / 10. as f32;
        let canvas: canvas::Canvas = canvas::Canvas::new();
        let mut objects: Vec<Box<dyn object::Object>> = vec![];

        objects.push(Box::new(copter::Copter::new()));
        objects.push(Box::new(wall::Wall::new()));

        Self
        {
            render_clock: render_clock,
            update_clock: update_clock,
            render_skip: render_skip,
            update_skip: update_skip,
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

    fn update(&mut self)
    {
        if self.update_clock.elapsed().as_millis() >= self.update_skip as u128
        {
            for object in self.objects.iter_mut()
            {
                object.update();
            }

            self.update_clock = time::Instant::now();
        }
    }

    fn render(&mut self)
    {
        if self.render_clock.elapsed().as_millis() >= self.render_skip as u128
        {
            self.canvas.clear();
            self.canvas.render(&self.objects);
            self.render_clock = time::Instant::now();
        }
    }
}
