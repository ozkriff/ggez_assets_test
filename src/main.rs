extern crate ggez;
 
use ggez::graphics::{Point2, DrawParam, draw_ex};
use ggez::graphics;
use ggez::conf;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::EventHandler;
use ggez::timer;
use ggez::event;
 
struct Assets {
    player_image: graphics::Image
}
impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "./ player.png")?;
 
        Ok(Assets {
            player_image,
        })
    }
}
 
struct MainState {
    assets: Assets,
}
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
 
        let s = MainState {
            assets,
        };
 
        Ok(s)
    }
}
 
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
        }
   
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
 
        let assets = &mut self.assets;
 
        Ok(())
    }
}
 
fn main() {
    let mut cb = ContextBuilder::new("Game", "ggez")
        .window_setup(conf::WindowSetup::default().title("Game Title"))
        .window_mode(conf::WindowMode::default().dimensions(700, 700));
 
    let ctx = &mut cb.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
 
    event::run(ctx, state);
 
}

