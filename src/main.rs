use ggez;
use ggez::Context;
use ggez::ContextBuilder;
use ggez::event;
use ggez::GameResult;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::timer;
use std::vec::Vec;

mod pit;
use pit::Pit;

const WELL_WIDTH: i32 = 10;
const WELL_HEIGHT: i32 = 20;
struct GameState {
    pit: Pit,
    time_counter: u128,
}

impl GameState {
    pub fn new() -> GameResult<GameState> {
        let state = GameState {
            pit: Pit::new( WELL_WIDTH, WELL_HEIGHT ),
            time_counter: 0,
        };
        Ok(state)
    }
}

impl event::EventHandler for GameState {

    fn update(&mut self, ctx:&mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            self.time_counter = 0;
            self.pit.move_piece_down();
        }
        self.time_counter += timer::delta(ctx).as_millis();
        if (self.time_counter > 1000) {
            self.time_counter = 0;
            self.pit.move_piece_down();
        }
        Ok(())
    }

    fn draw(&mut self, ctx:&mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let rect_bounds = _get_pit_bounds(ctx);
        let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect_bounds, graphics::WHITE)?;
        graphics::draw(ctx, &rect, graphics::DrawParam::default());
        for block in &self.pit.active_piece {
            let (x,y) = block;
            draw_block(x, y, graphics::BLACK, ctx, &rect_bounds);
        }
        for block in self.pit.get_solid_blocks() {
            let (x,y) = &block;
            draw_block(x, y, graphics::BLACK, ctx, &rect_bounds);
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymod: event::KeyMods, _repeat: bool) {
        if keycode == event::KeyCode::Right {
            self.pit.move_piece_right();
        }
        if keycode == event::KeyCode::Left {
            self.pit.move_piece_left();
        }
        if keycode == event::KeyCode::Up {
            self.pit.rotate_piece();
        }
        if keycode == event::KeyCode::Escape || keycode == event::KeyCode::Q {
            event::quit(ctx);
        }
    }

}

fn _get_pit_bounds(ctx:&mut Context) -> graphics::Rect {
    // FIXME (26 Aug 2019 sam): When the window is resized, there seems to be some bug
    // and the new pit size is not being calculated correctly.
    let (window_width, window_height) = graphics::drawable_size(&ctx);
    let block_size = window_height * 0.8 / WELL_HEIGHT as f32;
    let height = block_size * WELL_HEIGHT as f32;
    let width = block_size * WELL_WIDTH as f32;
    let x = (window_width / 2.0) - (block_size * WELL_WIDTH as f32/2.0);
    let y = window_height * 0.1;
    graphics::Rect::new(x, y, width, height)
}

fn draw_block(x: &i32, y: &i32, color: graphics::Color, ctx: &mut Context, pit_bounds: &graphics::Rect) {
    let block_size = pit_bounds.w / WELL_WIDTH as f32;
    let piece_x = pit_bounds.x + (*x as f32 * block_size);
    let piece_y = pit_bounds.y + (*y as f32 * block_size);
    let rect_bounds = graphics::Rect::new(piece_x, piece_y, block_size, block_size);
    let rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect_bounds, color).unwrap();
    graphics::draw(ctx, &rect, graphics::DrawParam::default());
}

pub fn main() -> ggez::GameResult {
    let state = &mut GameState::new()?;
    let context_builder = ContextBuilder::new("tetris", "sam");
    let (ctx, event_loop) = &mut context_builder.build()?;
    event::run(ctx, event_loop, state)
}

