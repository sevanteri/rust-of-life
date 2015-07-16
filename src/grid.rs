extern crate rand;

use piston::event::*;
use opengl_graphics::GlGraphics;

use app::Mode;

pub type Cell = bool;
pub type Col = Box<[Cell]>;
pub type Rows = Box<[Col]>;
//pub type Rows = [[Cell; YTILES]; XTILES];

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub tilesize: usize,
    pub zoom: f64,
    pub offset: [f64; 2],
    cells: Rows,
    buffer: Rows,
}

impl Grid {
    pub fn new(width: usize, height: usize, tilesize: usize) -> Grid {
        Grid {
            width: width,
            height: height,
            tilesize: tilesize,
            zoom: 1.0,
            offset: [0.0, 0.0],
            cells: Grid::new_empty_cells(width, height),
            buffer: Grid::new_empty_cells(width, height),
            //cells: [[false; YTILES]; XTILES],
            //buffer: [[false; YTILES]; XTILES],
        }
    }

    fn new_empty_cells(width: usize, height: usize) -> Rows {
        use std::iter::*;
        let mut rows = Vec::with_capacity(width + 1);
        for _ in 0..width {
            rows.push(
                Vec::from_iter(repeat(false).take(height)).into_boxed_slice()
            );
        }

        rows.into_boxed_slice()
    }

    pub fn randomize(&mut self) {
        for x in self.cells.iter_mut() {
            for y in x.iter_mut() {
                *y = rand::random();
            }
        }
        //println!("{:?}", self.cells);
    }

    pub fn tick(&mut self) {

        for x in 0..self.width {
            for y in 0..self.height {
                let n = self.neighbors(x, y);
                let c = &self.cells[x][y];
                let b = &mut self.buffer[x][y];

                if n == 3 && *c == false {
                    *b = true;
                }
                else if (n < 2 || n > 3) && *c == true {
                    *b = false;
                }
                else if n > 3 && *c == false {
                    *b = false;
                }
                else if n == 2 || n == 3 {
                    *b = *c;
                } else {
                    *b = false;
                }
            }
        }

        {
            use std::mem::swap;
            swap(&mut self.cells, &mut self.buffer);
        }

    }

    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        for (x, xv) in self.cells.iter().enumerate() {
            for (y, yv) in xv.iter().enumerate() {
                yv.render(x, y,
                          &self,
                          gl,
                          args);
            }
        }
        //for (x, xv) in self.buffer.iter_mut().enumerate() {
            //for (y, yv) in xv.iter_mut().enumerate() {
                //yv.render_red(x, y, self.tilesize, gl, args);
            //}
        //}

        self.draw_grid(gl, args);
    }

    // modulo
    fn m(a: isize, b: isize) -> usize {
        //use num::integer::mod_floor;
        //mod_floor(a, b) as usize
        ((a%b + b) % b) as usize
    }

    fn neighbors(&self, xv: usize, yv: usize) -> u8 {
        let c = &self.cells;

        let x = xv as isize;
        let y = yv as isize;
        let xcap = self.width as isize;
        let ycap = self.height as isize;

        let xp1 = Grid::m(x + 1, xcap);
        let xm1 = Grid::m(x - 1, xcap);
        let yp1 = Grid::m(y + 1, ycap);
        let ym1 = Grid::m(y - 1, ycap);

        // right, left, up, down
        return c[xp1][yv] as u8
             + c[xm1][yv] as u8
             + c[xv][ym1] as u8
             + c[xv][yp1] as u8

             /* nw, ne, se, sw */
             + c[xm1][ym1] as u8
             + c[xm1][yp1] as u8
             + c[xp1][yp1] as u8
             + c[xp1][ym1] as u8
    }

    fn draw_grid(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            let line = line::Line::new(color::grey(0.9), 0.5);
            let g = grid::Grid {
                rows: self.height as u32,
                cols: self.width as u32,
                units: self.tilesize as f64 * self.zoom
            };
            g.draw(&line, &c.draw_state, c.transform.trans(self.offset[0], self.offset[1]), gl);
        });
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        use std::cmp::min;
        let mut new_cells = Grid::new_empty_cells(width, height);
        let mut new_buffer = Grid::new_empty_cells(width, height);

        let xs = min(self.cells.len(), new_cells.len());
        let ys = min(self.cells[0].len(), new_cells[0].len());

        for x in 0..xs {
            for y in 0..ys {
                new_cells[x][y] = self.cells[x][y];
                new_buffer[x][y] = self.buffer[x][y];
            }
        }

        self.width = width;
        self.height = height;
        self.cells = new_cells;
        self.buffer = new_buffer;
    }

    pub fn clear(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.cells[x][y] = false;
                self.buffer[x][y] = false;
            }
        }
    }

    pub fn draw(&mut self, mouse: &[f64; 2], mode: &Mode) {
        let x = ((mouse[0] - self.offset[0]) / self.tilesize as f64 / self.zoom) as usize;
        let y = ((mouse[1] - self.offset[1]) / self.tilesize as f64 / self.zoom) as usize;
        //println!("{}, {}", x,y);

        match *mode {
            Mode::Dot => self.cells[x][y] = !self.cells[x][y],
            Mode::Glider => self.glider((x, y)),
            Mode::Line => self.line((x, y)),
            _ => {}
        }

    }

    pub fn glider(&mut self, (x, y): (usize, usize)) {
        let xp1 = Grid::m((x + 1) as isize, self.width as isize);
        let xp2 = Grid::m((x + 2) as isize, self.width as isize);
        let yp1 = Grid::m((y + 1) as isize, self.height as isize);
        let yp2 = Grid::m((y + 2) as isize, self.height as isize);

        self.cells[x][y] = true;
        self.cells[xp2][y] = true;
        self.cells[xp2][yp1] = true;
        self.cells[xp1][yp1] = true;
        self.cells[xp1][yp2] = true;
    }

    pub fn line(&mut self, (x, y): (usize, usize)) {
        self.cells[x - 1][y].toggle();
        self.cells[x][y].toggle();
        self.cells[x + 1][y].toggle();
    }
}

trait Tile {
    fn render(&self,
              x: usize,
              y: usize,
              grid: &Grid,
              gl: &mut GlGraphics,
              args: &RenderArgs);
    fn render_red(&self,
                  x: usize,
                  y: usize,
                  grid: &Grid,
                  gl: &mut GlGraphics,
                  args: &RenderArgs);
    fn toggle(&mut self);
}

impl Tile for Cell {
    fn render(&self,
              x: usize,
              y: usize,
              grid: &Grid,
              gl: &mut GlGraphics,
              args: &RenderArgs)
    {
        use graphics::*;

        if !*self { return; }

        let square = rectangle::square(
            ((x * grid.tilesize) as f64 * grid.zoom) + grid.offset[0],
            ((y * grid.tilesize) as f64 * grid.zoom) + grid.offset[1],
            grid.tilesize as f64 * grid.zoom
        );

        gl.draw(args.viewport(), |c, gl| {
            rectangle(color::BLACK, square, c.transform, gl);
        });
    }

    fn render_red(&self,
                  x: usize,
                  y: usize,
                  grid: &Grid,
                  gl: &mut GlGraphics,
                  args: &RenderArgs)
    {
        use graphics::*;

        if !*self { return; }

        let square = rectangle::square(
            (x * grid.tilesize + grid.tilesize/4) as f64,
            (y * grid.tilesize + grid.tilesize/4) as f64,
            (grid.tilesize/2) as f64
        );

        gl.draw(args.viewport(), |c, gl| {
            rectangle(color::hex("CC0000"), square, c.transform, gl);
        });
    }

    fn toggle(&mut self) {
        *self = !*self;
    }
}
