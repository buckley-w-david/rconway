use bracket_terminal::prelude::*;

use std::collections::HashSet;

pub trait Around {
    fn around(&self) -> Vec<Point>;
}

impl Around for Point {
    fn around(&self) -> Vec<Point> {
        vec![
            Point {
                x: self.x - 1,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x + 1,
                y: self.y - 1,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y + 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Running,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Debug {
    Normal,
    Debug(u8),
}

#[derive(Debug)]
struct Game {
    board: HashSet<Point>,
    reference_point: (i32, i32),
    cursor_point: (i32, i32),
    mode: Mode,
    debug: Debug,
    single_step: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            board: HashSet::new(),
            reference_point: (0, 0),
            cursor_point: (0, 0),
            mode: Mode::Waiting,
            debug: Debug::Normal,
            single_step: false,
        }
    }

    fn neighbours(&self, point: &Point) -> u8 {
        point
            .around()
            .iter()
            .filter(|p| self.board.contains(p))
            .count() as u8
    }

    fn step(&mut self) {
        let mut edits = Vec::<(Point, bool)>::new();
        for p in self.board.union(
            &self
                .board
                .iter()
                .flat_map(|p| p.around())
                .collect::<HashSet<Point>>(),
        ) {
            let alive = self.board.contains(&p);
            let count = self.neighbours(&p);

            if alive && count > 3 {
                edits.push((*p, false));
            } else if alive && count < 2 {
                edits.push((*p, false));
            } else if !alive && count == 3 {
                edits.push((*p, true));
            }
        }

        for (p, alive) in edits {
            if !alive {
                self.board.remove(&p);
            } else {
                self.board.insert(p);
            }
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.cls();
        match ctx.key {
            Some(VirtualKeyCode::P) => {
                self.mode = match self.mode {
                    Mode::Waiting => Mode::Running,
                    Mode::Running => Mode::Waiting,
                }
            }
            Some(VirtualKeyCode::D) => {
                self.debug = match self.debug {
                    Debug::Normal => Debug::Debug(0),
                    Debug::Debug(0) => Debug::Debug(1),
                    _ => Debug::Normal,
                }
            }
            Some(VirtualKeyCode::S) => {
                self.single_step = true;
            }
            Some(VirtualKeyCode::M) => {
                self.board.insert(Point::new(
                    self.cursor_point.0 + self.reference_point.0,
                    self.cursor_point.1 + self.reference_point.1,
                ));
            }
            // Shift viewport
            Some(VirtualKeyCode::Left) => {
                self.reference_point = (self.reference_point.0 - 1, self.reference_point.1)
            }
            Some(VirtualKeyCode::Right) => {
                self.reference_point = (self.reference_point.0 + 1, self.reference_point.1)
            }
            Some(VirtualKeyCode::Up) => {
                self.reference_point = (self.reference_point.0, self.reference_point.1 - 1)
            }
            Some(VirtualKeyCode::Down) => {
                self.reference_point = (self.reference_point.0, self.reference_point.1 + 1)
            }
            // Shift Cursor
            Some(VirtualKeyCode::H) => {
                self.cursor_point = (self.cursor_point.0 - 1, self.cursor_point.1)
            }
            Some(VirtualKeyCode::J) => {
                self.cursor_point = (self.cursor_point.0, self.cursor_point.1 + 1)
            }
            Some(VirtualKeyCode::K) => {
                self.cursor_point = (self.cursor_point.0, self.cursor_point.1 - 1)
            }
            Some(VirtualKeyCode::L) => {
                self.cursor_point = (self.cursor_point.0 + 1, self.cursor_point.1)
            }
            _ => (),
        }

        if self.mode == Mode::Running || self.single_step {
            self.single_step = false;
            self.step();
        }

        for y in 0..51 {
            for x in 0..81 {
                match self.debug {
                    Debug::Normal => {
                        let drawp = Point::new(x, y);
                        let boardp = Point::new(
                            drawp.x + self.reference_point.0,
                            drawp.y + self.reference_point.1,
                        );
                        if let Some(_) = self.board.get(&boardp) {
                            draw_batch.print_color(
                                drawp,
                                "#",
                                ColorPair::new(RGBA::from_u8(12, 201, 6, 255), RGBA::new()),
                            );
                        }
                    }
                    Debug::Debug(level) => {
                        let drawp = Point::new(x, y);
                        let boardp = Point::new(
                            drawp.x + self.reference_point.0,
                            drawp.y + self.reference_point.1,
                        );
                        let count = self.neighbours(&boardp);
                        let cp = match count {
                            0 => ColorPair::new(RGBA::from_u8(0, 0, 255, 255), RGBA::new()),
                            _ => ColorPair::new(RGBA::from_u8(247, 121, 24, 255), RGBA::new()),
                        };

                        if level > 0 {
                            draw_batch.print_color(drawp, format!("{}", count), cp);
                        } else if let Some(_) = self.board.get(&boardp) {
                            draw_batch.print_color(
                                drawp,
                                "#",
                                ColorPair::new(RGBA::from_u8(12, 201, 6, 255), RGBA::new()),
                            );
                        } else {
                            draw_batch.print_color(drawp, format!("{}", count), cp);
                        }
                    }
                }
            }
        }
        draw_batch.print_color(
            Point::new(self.cursor_point.0, self.cursor_point.1),
            "@",
            ColorPair::new(RGBA::from_u8(12, 201, 6, 255), RGBA::new()),
        );
        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    // BTerm's builder interface offers a number of helpers to get you up and running quickly.
    // Here, we are using the `simple80x50()` helper, which builds an 80-wide by 50-tall console,
    // with the baked-in 8x8 terminal font.
    let context = BTermBuilder::simple80x50()
        .with_title("Conway's Game of Life")
        .with_fps_cap(30.0)
        .build()?;

    let mut game = Game::new();
    // Seed the board with a glider
    game.board.insert(Point { x: 10, y: 11 });
    game.board.insert(Point { x: 11, y: 12 });
    game.board.insert(Point { x: 12, y: 12 });
    game.board.insert(Point { x: 12, y: 11 });
    game.board.insert(Point { x: 12, y: 10 });

    // Call into BTerm to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, game)
}
