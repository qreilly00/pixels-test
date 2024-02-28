use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder/*, Fullscreen*/},
};

use pixels::{Pixels, SurfaceTexture};

struct Program {
    window_width: f32,
    window_height: f32,
}

impl Program {
    pub fn new() -> Self {
        Self {
            window_width: 1024.0,
            window_height: 1024.0,
        }
    }

    /*pub fn with_values(window_width: f32, window_height: f32) -> Self {
        Self {
            window_width,
            window_height,
        }
    }*/

    pub fn fill_window(&self, frame: &mut [u8]) {
        for (/*i*/_, pixel) in frame.chunks_exact_mut(4).enumerate() {
            /*let x = (i % self.window_width as usize) as i16;
            let y = (i / self.window_height as usize) as i16;*/

            let rgba = [0x00, 0x00, 0x00, 0xff];

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn window_width(&self) -> f32 {self.window_width}
    pub fn window_height(&self) -> f32 {self.window_height}
}

struct Square {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Square {
    /*pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 32.0,
            height: 32.0,
        }
    }*/

    pub fn with_values(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn draw(&self, frame: &mut [u8], window_width: f32, window_height: f32) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pos_x = (i % window_width as usize) as i16;
            let pos_y = (i / window_height as usize) as i16;

            let rgba = [0xff, 0xff, 0xff, 0xff];

            let (tmp_x, tmp_y, tmp_width, tmp_height) = (self.x as i16, self.y as i16, self.width as i16, self.height as i16);

            if (pos_x >= tmp_x && pos_y >= tmp_y) && (pos_x <= tmp_width + tmp_x && pos_y <= tmp_height + tmp_y) {
                pixel.copy_from_slice(&rgba);
            }
        }
    }

    /*pub fn x(&self) -> f32 {self.x}
    pub fn y(&self) -> f32 {self.x}
    pub fn width(&self) -> f32 {self.x}
    pub fn height(&self) -> f32 {self.x}*/
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let program = Program::new();

    let window = {
        let logical_size = LogicalSize::new(program.window_width(), program.window_height());
        //let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);

        WindowBuilder::new()
            .with_title("Test Window")
            //.with_inner_size(scaled_size)
            .with_min_inner_size(logical_size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        //let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(program.window_width() as u32, program.window_height() as u32, &window);
        Pixels::new(program.window_width() as u32, program.window_height() as u32, surface_texture).unwrap()
    };

    let mut squares = Vec::new();

    for i in 0..3 {
        squares.push(Square::with_values(i as f32 * 32.0, 0.0, 32.0, 32.0));
    }

    //squares.push(Square::new());

    let _ = event_loop.run(move |event, elwt| {
        /*match event {
            Event::AboutToWait => {
                // Application update code.
            },

            _ => ()
        }*/

        if let Event::WindowEvent {event, ..} = event {
            match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                },

                WindowEvent::KeyboardInput {event: _, ..} => {

                    elwt.exit();
                },

                _ => ()
            }
        }
        program.fill_window(pixels.frame_mut());

        //square.draw(pixels.frame_mut(), program.window_width(), program.window_height());
        for (_, square) in squares.iter().enumerate() {
            square.draw(pixels.frame_mut(), program.window_width(), program.window_height());
        }

        pixels.render().unwrap();
    }).unwrap();

}
