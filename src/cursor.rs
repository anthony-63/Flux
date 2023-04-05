use nannou::{prelude::*, winit::dpi::{PhysicalPosition}};

pub struct FluxCursor {
    pub x: f32,
    pub y: f32,
    pub textures: Vec<wgpu::Texture>,
    size: f32,
}

impl FluxCursor {
    pub fn new(cursor_size: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            size: cursor_size,
            textures: vec![],
        }
    }
    pub fn draw(&self, draw: Draw, index: usize) {
        draw.texture(&self.textures[index]).x_y(self.x, self.y).w_h(self.size, self.size);
    }
    pub fn cursor_move(&mut self, mp: Point2, sens: f32) {
        self.x = mp.x * sens;
        self.y = mp.y * sens;
    }
    pub fn change_cursor_size(&mut self, size: f32) {
        self.size = size;
    }

    pub fn lock_cursor_to_play_area(&mut self, pw: f32, ph: f32) {
        self.x = self.x.clamp(-(pw/2.0), pw/2.0);
        self.y = self.y.clamp(-(ph/2.0),ph/2.0);
    }

    pub fn lock_real_cursor_to_play_area(&mut self, app: &App, window: WindowId, sens: f32, edge_buffer: f32, pw: f32, ph: f32) {
        let w = app.window(window).unwrap();
        let x = app.mouse.x + (app.window_rect().w() as f32 / 2.0);
        let y = (app.window_rect().h() as f32 / 2.0) - app.mouse.y;
        if app.mouse.x > ((pw / 2.0) / sens) + edge_buffer * 1.5 {
            w.winit_window().set_cursor_position(PhysicalPosition::new(x - 10.0, y)).unwrap();
        }else if app.mouse.x < -((pw / 2.0) / sens) - edge_buffer {
            w.winit_window().set_cursor_position(PhysicalPosition::new(x + 10.0, y)).unwrap();
        } else if app.mouse.y > ((ph / 2.0) / sens) + edge_buffer * 1.5 {
            w.winit_window().set_cursor_position(PhysicalPosition::new(x, y + 10.0)).unwrap();
        } else if app.mouse.y < -((ph / 2.0) / sens) - edge_buffer {
            w.winit_window().set_cursor_position(PhysicalPosition::new(x, y - 10.0)).unwrap();
        }
    }
}