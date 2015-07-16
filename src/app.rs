use piston::event::*;
use piston::input::Button;
use opengl_graphics::{ GlGraphics, OpenGL };

use grid::Grid;

use settings::*;

pub enum Mode {
    Dot,
    Line,
    SmallCube,
    SmallPlus,

    Glider,
}

pub struct App {
    pub gl: GlGraphics, // opengl drawing backend
    pub grid: Grid,
    pub mouse: [f64; 2],
    pub mode: Mode,
    pub paused: bool,
    pub speed: f64, // ms
    pub next_tick: f64,

    pub width: u32,
    pub height: u32,
}

impl App {
    pub fn new(opengl: OpenGL, grid: Grid) -> App {
        App {
            gl: GlGraphics::new(opengl),
            grid: grid,
            mouse: [0.0; 2],
            mode: Mode::Dot,
            paused: true,
            speed: 0.1,
            next_tick: 0.0,

            width: WIDTH as u32,
            height: HEIGHT as u32,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |_, gl| {
            clear(color::WHITE, gl);
        });
        self.grid.render(&mut self.gl, args);
    }

    pub fn resize_grid(&mut self) {
        let width = self.width as usize / self.grid.tilesize;
        let height = self.height as usize / self.grid.tilesize;

        if width != self.grid.width || height != self.grid.height {
            self.grid.resize(width, height);
        }
    }

    pub fn resize(&mut self, args: [u32; 2]) {
        self.width = args[0];
        self.height = args[1];
        self.resize_grid();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if self.paused { return; }

        self.next_tick -= args.dt;
        if self.next_tick <= 0.0 {
            self.grid.tick();
            self.next_tick = self.speed;
        }
    }

    pub fn key_press(&mut self, args: &Button) {
        use piston::input::Button::{ Keyboard, Mouse };
        use piston::input::keyboard::Key;
        use piston::input::mouse::MouseButton;

        match *args {
            Keyboard(Key::Space)  => {
                self.paused = !self.paused;
            },
            Keyboard(Key::Comma)  => {
                self.speed -= 0.005;
                if self.speed < 0.0 { self.speed = 0.0; }
                println!("Speed: {:.3}", self.speed);
            },
            Keyboard(Key::Period)  => {
                self.speed += 0.005;
                println!("Speed: {:.3}", self.speed);
            },
            Keyboard(Key::M)  => {
                self.grid.tilesize -= 1;
                self.resize_grid();
                println!("Tilesize: {:.3}", self.grid.tilesize);
            },
            Keyboard(Key::N)  => {
                self.grid.tilesize += 1;
                self.resize_grid();
                println!("Tilesize: {:.3}", self.grid.tilesize);
            },
            Keyboard(Key::D1) => {
                self.grid.zoom -= if self.grid.zoom == 1.0 { 0.0 } else { 0.1 };
                println!("Zoom: {:.1}", self.grid.zoom);
            },
            Keyboard(Key::D2) => {
                self.grid.zoom += 0.1;
                println!("Zoom: {:.1}", self.grid.zoom);
            },
            Keyboard(Key::R) => {
                self.grid.randomize();
            },
            Keyboard(Key::C) => {
                self.grid.clear();
            },
            Keyboard(Key::T) => {
                self.grid.tick();
            },
            Keyboard(Key::L) => {
                self.mode = Mode::Line;
                println!("Line mode");
            },
            Keyboard(Key::D) => {
                self.mode = Mode::Dot;
                println!("Dot mode");
            },
            Keyboard(Key::G) => {
                self.mode = Mode::Glider;
                println!("Glider mode");
            },
            Mouse(MouseButton::Left) => {
                self.grid.draw(&self.mouse, &self.mode);
            },

            _ => {}
        }
    }

    pub fn mouse_move(&mut self, args: [f64; 2]) {
        self.mouse = args;
        self.grid.offset = [
            args[0] * (1.0 - self.grid.zoom),
            args[1] * (1.0 - self.grid.zoom)
        ];
        //println!("{:?}", self.mouse);
    }

    pub fn mouse_scroll(&mut self, args: [f64; 2]) {
        println!("{:?}", args);
    }
}
