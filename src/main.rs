mod map;
mod player;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub use crate::map::*;
    pub use crate::player::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player
}

impl State {
    fn new() -> Self {
        State {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
    
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Dungen Crawler").with_fps_cap(30.0).build()?;

    main_loop(context, State::new())
}
