#![allow(dead_code)]

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
//extern crate num;
extern crate rand;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

mod app;
mod grid;
mod settings;

use grid::*;
use app::*;
use settings::*;


fn main() {
    let opengl = OpenGL::_3_2;

    let win = Window::new(
        WindowSettings::new(
            "Game of Life",
            [WIDTH as u32, HEIGHT as u32]
        )
        .exit_on_esc(true)
    );

    let mut app = App::new(
        opengl,
        Grid::new(XTILES, YTILES, TILESIZE)
    );
    app.grid.glider((5,5));

    for e in win.events() {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(p) = e.press_args() {
            app.key_press(&p);
        }

        if let Some(m) = e.mouse_cursor_args() {
            app.mouse_move(&m);
        }

        if let Some(r) = e.resize_args() {
            app.resize(&r);
        }
    }

}
