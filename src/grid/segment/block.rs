use sdl2::rect::Rect;

use crate::{
    traits::{Behaviour, Render},
    world::{World, get_pos}, constants::{SIZE_WORLD, SIZE_RECT_RENDER},
};

/// `Block` - the essence, which is impassable.
#[derive(Debug, Clone, Default)]
pub struct Block {}

impl Render for Block {
    fn render(&self, sdl: &mut crate::app::SDL, idx: usize) {
        let pos = get_pos(idx, SIZE_WORLD[0]);
        let (x, y) = (pos.x as i32, pos.y as i32);
        let canvas = &mut sdl.canvas;
        let rect = Rect::new(
            x * SIZE_RECT_RENDER,
            SIZE_RECT_RENDER * (-y + 2*(SIZE_WORLD[1] as i32) - 1),
            SIZE_RECT_RENDER as u32,
            SIZE_RECT_RENDER as u32,
        );
        
        canvas.set_draw_color((0x64, 0x64, 0x67));
        canvas.fill_rect(rect).unwrap();
    }
}

impl Behaviour for Block {
    fn update(world_read: &World, world: &mut World, idx: usize) {}
}
