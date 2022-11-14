mod fire;
mod color;

use std::borrow::Borrow;
use ggez::*;
use ggez::graphics::*;
use lerp::num_traits::ToPrimitive;
use crate::fire::Fire;

static STEPS: usize = 4;
static PIXEL_SIZE: f32 = 5.0;
struct State {
    doom_fire: Fire
}


impl event::EventHandler<GameError> for State{
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        _ctx.gfx.set_window_title(&*_ctx.time.fps().to_string());

        while _ctx.time.check_update_time(50){
            self.doom_fire.burn_frame();
        }
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {

        let mb = &mut MeshBuilder::new();
        let mut idx = 0;
        let mut idy = 0;
        for column in self.doom_fire.bonfire.iter(){
            for pixel in column.iter(){
                let color_pixel = &self.doom_fire.palette[*pixel as usize];
                if color_pixel != color::NEGRO.borrow() {
                    //let rect = ;
                    mb.rectangle(
                        DrawMode::fill(),
                        Rect::new(
                            (idx as f32) * PIXEL_SIZE,
                            (idy as f32) * PIXEL_SIZE,
                            PIXEL_SIZE,
                            PIXEL_SIZE
                        ),
                        color_pixel.to_ggez_color()
                    )?;
                }
                idy +=1;
            }
            idx +=1;
            idy=0
        }
        let mut canvas = Canvas::from_frame(_ctx, Color::BLACK);
        let mesh = Mesh::from_data(_ctx, mb.build());
        canvas.draw(mesh.borrow(), DrawParam::new());
        canvas.finish(_ctx)?;
        Ok(())
    }


}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.width=1024.0;
    c.window_mode.height=768.0;

    let estado = State {
        doom_fire: Fire::new(
            STEPS,
            (c.window_mode.width / PIXEL_SIZE).floor().to_u8().unwrap(),
            (c.window_mode.height / PIXEL_SIZE).floor().to_u8().unwrap()
    )};


    let (ctx, event_loop) = ContextBuilder::new("Hello GGEZ", "Alfonsoak")
        .default_conf(c)
        .build()
        .unwrap();


    event::run(ctx,event_loop,estado);

}
