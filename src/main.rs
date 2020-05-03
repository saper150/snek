use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::Text;
use ggez::{event, graphics, Context, GameResult};

use std::time::{Duration, Instant};

use rpds;

mod grid;
mod path;
mod snek;
use grid::GridPosition;
use std::collections::HashMap;


const UPDATES_PER_SECOND: f32 = 36.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;


struct Food {
    pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos }
    }
}

const SCREEN_SIZE: (f32, f32) = (25.0 * 32.0, 25.0 * 32.0);

const SIZE: i32 = 10;

const GRID_SIZE: (i32, i32) = (SIZE, SIZE);

const GRID_CELL_SIZE: (i32, i32) = (
    (SCREEN_SIZE.0 / (GRID_SIZE.0 as f32)) as i32,
    (SCREEN_SIZE.1 / (GRID_SIZE.1 as f32)) as i32,
);

struct GameState {
    snek: snek::Snek,
    food: Food,
    scores: HashMap<GridPosition, i32>,
    last_update: Instant,
    grid: grid::Grid,
    follow_path: rpds::List<snek::Snek>,
    run: bool,
}

impl GameState {
    pub fn new() -> Self {
        let gridd = grid::Grid::new(GRID_SIZE.0, GRID_SIZE.1);

        let snake_pos = grid::GridPosition {
            x: gridd.size_x / 4,
            y: gridd.size_y / 2,
        };
        let food_pos = gridd.random_position();
        let snek = snek::Snek::new(snake_pos);
        let (path, scores) = path::find_path(snek.clone(), &gridd, food_pos);

        GameState {
            snek: snek,
            food: Food::new(food_pos),
            follow_path: path,
            scores: scores,
            grid: gridd,
            last_update: Instant::now(),
            run: false,
        }
    }

    fn generate_food(&mut self) {
        let new_food = self.grid.random_position();
        if self.snek.is_occupied(&new_food) {
            self.generate_food()
        } else {
            self.food = Food { pos: new_food }
        }
    }
}

fn draw_snek(state: &GameState, ctx: &mut Context) -> GameResult<()> {
    for seg in state.snek.body.iter() {
        draw_position(ctx, seg.pos, [0.3, 0.3, 0.0, 1.0].into())?;
    }

    draw_position(ctx, state.snek.get_head(), [1.0, 0.5, 0.0, 1.0].into())?;
    Ok(())
}

fn draw_food(state: &GameState, ctx: &mut Context) -> GameResult<()> {
    draw_position(ctx, state.food.pos, [0.0, 0.0, 1.0, 1.0].into())?;
    Ok(())
}

fn draw_scores(state: &GameState, ctx: &mut Context) -> GameResult<()> {
    draw_position(ctx, state.food.pos, [0.0, 0.0, 1.0, 1.0].into())?;

    for (position, score) in state.scores.iter() {
        let text = Text::new(score.to_string());
        graphics::draw(
            ctx,
            &text,
            (ggez::mint::Point2 {
                x: (position.x * GRID_CELL_SIZE.0) as f32,
                y: (position.y * GRID_CELL_SIZE.1) as f32,
            },),
        )?;
    }

    Ok(())
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !(Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE)) {
            return Ok(());
        }

        if !self.run {
            return Ok(());
        }

        match self.follow_path.first() {
            Some(s) => {
                self.snek = (*s).clone();
                self.follow_path = self.follow_path.drop_first().unwrap();
            }
            None => {
                // let new_snek = self.snek.eat(self.food.pos);
                // self.snek = new_snek;
                self.generate_food();
                let (path, scores) = path::find_path(self.snek.clone(), &self.grid, self.food.pos);
                self.follow_path = path;
                self.scores = scores;
            }
        }

        self.last_update = Instant::now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(
            ctx,
            [37.0 / 255.0, 116.0 / 255.0, 169.0 / 255.0, 1.0].into(),
        );
        draw_snek(self, ctx)?;
        draw_food(self, ctx)?;
        draw_scores(self, ctx)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.run = false
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Space {
            self.run = true
        }
    }
}

fn main() -> GameResult {


    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "Gray Olson")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new();
    event::run(ctx, events_loop, state)
}

fn draw_position(ctx: &mut Context, pos: GridPosition, color: graphics::Color) -> GameResult<()> {
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), pos.into(), color)?;

    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}
