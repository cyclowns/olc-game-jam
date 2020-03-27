mod entities;
mod systems;
mod components;
mod util;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::{nalgebra as na, timer};
use ggez::input::keyboard;
use ggez::graphics::{self, Text, Rect, Mesh};
use ggez::conf::{WindowMode, WindowSetup};
use log::{info, warn, error};

fn main() {
    util::setup_logger().unwrap();
    info!("Game started!");
    let (mut ctx, mut event_loop) =
        ContextBuilder::new("costcow-defense", "cyclowns")
            .window_setup(WindowSetup::default()
                          .title("CostCow Defense"))
            .window_mode(WindowMode::default()
                          .dimensions(800.0, 600.0)
                          .borderless(true)
                          .resizable(false))
            .build()
            .unwrap();

    let mut my_game = OLCGameJam::new(&mut ctx);
    
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => info!("Exited cleanly."),
        Err(e) => error!("Error: {}", e)
    }
}

struct OLCGameJam {

}

impl OLCGameJam {
    pub fn new(_ctx: &mut Context) -> OLCGameJam {
        OLCGameJam {}
    }
}

impl EventHandler for OLCGameJam {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.8, 0.1, 1.0].into());

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}