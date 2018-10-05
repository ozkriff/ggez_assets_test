extern crate ggez;

use ggez::{
    conf, event,
    event::EventHandler,
    graphics, {Context, ContextBuilder, GameResult},
};

struct Assets {
    player_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        Ok(Assets { player_image })
    }
}

struct MainState {
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        Ok(MainState { assets })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let dst = graphics::Point2::new(20.0, 20.0);
        graphics::draw(ctx, &self.assets.player_image, dst, 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let cb = ContextBuilder::new("Game", "ggez")
        .add_resource_path("resources")
        .window_setup(conf::WindowSetup::default().title("Game Title"))
        .window_mode(conf::WindowMode::default().dimensions(700, 700));
    let ctx = cb.build().unwrap();
    let state = MainState::new(&mut ctx).unwrap();
    event::run(&mut ctx, &mut state).unwrap();
}
