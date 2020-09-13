extern crate ggez;

use graphics::DrawParam;
use ggez::ContextBuilder;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::event::*;
use std::env;
use std::path;
use std::time;

const SPEED: f32 = 8.0;

struct Dog {
    xpos: f32,
    ypos: f32,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    space_pressed: bool,
    state: &'static str,
    walk_1: graphics::spritebatch::SpriteBatch,
    walk_2: graphics::spritebatch::SpriteBatch,
    walk_3: graphics::spritebatch::SpriteBatch,
    walk_4: graphics::spritebatch::SpriteBatch,
    pee: graphics::spritebatch::SpriteBatch,
}

// First we make a structure to contain the game's state
struct MainState {
    hydrants: graphics::spritebatch::SpriteBatch,
    roads: graphics::spritebatch::SpriteBatch,
    grass: graphics::spritebatch::SpriteBatch,
    twoo: graphics::spritebatch::SpriteBatch,
    dog: Dog,
    frames: usize,
    offset: f32,
    points: isize,
    hydrant_pos: Vec<f32>,
    last_start: time::Duration,
    state: &'static str,
    last_score: isize,
}

impl Dog {
    fn new(ctx: &mut Context) -> GameResult<Dog> { 
        let walk1 = graphics::Image::new(ctx, "/dog_walk1.png")?;
        let walk1_sb = graphics::spritebatch::SpriteBatch::new(walk1.clone());
        let walk2 = graphics::Image::new(ctx, "/dog_walk2.png")?;
        let walk2_sb = graphics::spritebatch::SpriteBatch::new(walk2.clone());
        let walk3 = graphics::Image::new(ctx, "/dog_walk3.png")?;
        let walk3_sb = graphics::spritebatch::SpriteBatch::new(walk3.clone());
        let walk4 = graphics::Image::new(ctx, "/dog_walk4.png")?;
        let walk4_sb = graphics::spritebatch::SpriteBatch::new(walk4.clone());
        let pee = graphics::Image::new(ctx, "/dog_pee.png")?;
        let pee_sb = graphics::spritebatch::SpriteBatch::new(pee.clone());
        Ok(Dog {
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            space_pressed: false,
            xpos: 200.0,
            ypos: 400.0,
            state: "stopped",
            walk_1: walk1_sb,
            walk_2: walk2_sb,
            walk_3: walk3_sb,
            walk_4: walk4_sb,
            pee: pee_sb,
        })
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl MainState {
    fn new(mut ctx: &mut Context) -> GameResult<MainState> {
        let hydrant = graphics::Image::new(ctx, "/hydrant.png")?;
        let hydrantbatch = graphics::spritebatch::SpriteBatch::new(hydrant);
        let road = graphics::Image::new(ctx, "/road.png")?;
        let roadbatch = graphics::spritebatch::SpriteBatch::new(road);
        let grass = graphics::Image::new(ctx, "/grass.png")?;
        let grassbatch = graphics::spritebatch::SpriteBatch::new(grass);
        let twoo = graphics::Image::new(ctx, "/twoo.png")?;
        let twoo_sb = graphics::spritebatch::SpriteBatch::new(twoo);

        let s = MainState {
            hydrants: hydrantbatch,
            roads: roadbatch,
            grass: grassbatch,
            twoo: twoo_sb,
            dog: Dog::new(&mut ctx)?,
            frames: 0,
            offset: 0.0,
            points: 0,
            hydrant_pos: Vec::new(),
            last_start: time::Duration::from_secs(0),
            state: "menu",
            last_score: 0,
        };
        Ok(s)
    }
}


impl event::EventHandler for MainState {

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode,  _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Return => {
                self.state = "play";
                self.dog.xpos = -1.0*self.offset+200.0;
                self.last_score = self.points;
                self.points = 0;
            }
            KeyCode::Up => {
                self.dog.up_pressed = true;
                self.dog.state = "walking";
            }
            KeyCode::Down => {
                self.dog.down_pressed = true;
                self.dog.state = "walking";
            }
            KeyCode::Left => {
                self.dog.left_pressed = true;
                self.dog.state = "walking";
            }
            KeyCode::Right => {
                self.dog.right_pressed = true;
                self.dog.state = "walking";
            }
            KeyCode::Space => {
                self.dog.space_pressed = true;
                self.dog.state = "peeing";
            }
            _ => (), // Do nothing
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => {
                self.dog.up_pressed = false;
                self.dog.state = "stopped";
            }
            KeyCode::Down => {
                self.dog.down_pressed = false;
                self.dog.state = "stopped";
            }
            KeyCode::Left => {
                self.dog.left_pressed = false;
                self.dog.state = "stopped";
            }
            KeyCode::Right => {
                self.dog.right_pressed = false;
                self.dog.state = "stopped";
            }
            KeyCode::Space => {
                self.dog.space_pressed = false;
                self.dog.state = "stopped";
            }
            _ => (), // Do nothing
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut elapsed_time =ggez::timer::duration_to_f64(ggez::timer::time_since_start(ctx)) as f32; 
        elapsed_time -= ggez::timer::duration_to_f64(self.last_start) as f32;
        self.offset -= SPEED*(((ggez::timer::delta(ctx)).subsec_nanos() as f32)/1e8)+elapsed_time*(1.0/24.0); 

        if self.state != "menu" {
            if self.dog.space_pressed {
                self.dog.up_pressed = false;
                self.dog.down_pressed = false;
                self.dog.left_pressed = false;
                self.dog.right_pressed = false;

                if self.dog.ypos < 200.0 {
                    for hydrant in self.hydrant_pos.clone().into_iter() {
                        if self.dog.xpos > hydrant-130.0 && self.dog.xpos < hydrant-70.0 {
                            self.points += 1;
                        }
                    } 
                }
            }
            if self.dog.up_pressed {
                if self.dog.ypos > 170.0 {
                    self.dog.ypos -= 15.0*((ggez::timer::delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
            if self.dog.down_pressed {
                if self.dog.ypos < 470.0 {
                    self.dog.ypos += 15.0*((ggez::timer::delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
            if self.dog.left_pressed {
                if self.dog.xpos > -1.0*self.offset+100.0 {
                    self.dog.xpos -= 30.0*((ggez::timer::delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
            if self.dog.right_pressed {
                if self.dog.xpos < -1.0*self.offset+550.0 {
                    self.dog.xpos += 30.0*((ggez::timer::delta(ctx).subsec_nanos() as f32/1e8));
                }
            }
        }
        else {
            self.last_start = ggez::timer::time_since_start(ctx);
        }

        if self.dog.xpos < -1.0*self.offset-20.0 {
            self.state = "menu";
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        // Drawables are drawn from their center.
      
        { // grass
            let first_grass = -1.0*(self.offset-(self.offset%-640.0));
            for i in 0..5 {
                let p = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[first_grass+(640.0*i as f32),100.0]),
                    scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                    rotation: 0.0,
                    ..Default::default()
                };
                self.grass.add(p);
            }

            let param = graphics::DrawParam {
                //dest: ggez::mint::Point2::from_slice(self.offset/2.0,0.0),
                dest: ggez::mint::Point2::from_slice(&[self.offset,0.0]),
                scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                rotation: 0.0,
                offset: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                ..Default::default()
            };
            graphics::draw(ctx, &self.grass, param)?;
            self.grass.clear();
        }
        { // road
            let first_road = -1.0*(self.offset-(self.offset%-501.0));
            for i in 0..5 {
                let p = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[first_road+(501.0*i as f32),265.0]),
                    scale: ggez::mint::Vector2::from_slice(&[0.5,0.5]),
                    rotation: 0.0,
                    ..Default::default()
                };
                self.roads.add(p);
            }

            let param = graphics::DrawParam {
                dest: ggez::mint::Point2::from_slice(&[self.offset,0.0]),
                scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                rotation: 0.0,
                offset: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                ..Default::default()
            };
            graphics::draw(ctx, &self.roads, param)?;
            self.roads.clear();
        }
        { // hydrants
            let first_hydrant = -1.0*(self.offset-(self.offset%-512.0));
            self.hydrant_pos = Vec::new();
            for i in 0..100 {
                self.hydrant_pos.push(first_hydrant+(512.0*i as f32));
                let p = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[first_hydrant+(512.0*i as f32),190.0]),
                    scale: ggez::mint::Vector2::from_slice(&[3.0,3.0]),
                    rotation: 0.0,
                    ..Default::default()
                };
                self.hydrants.add(p);
            }

            let param = graphics::DrawParam {
                dest: ggez::mint::Point2::from_slice(&[self.offset,0.0]),
                scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                rotation: 0.0,
                offset: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                ..Default::default()
            };
            graphics::draw(ctx, &self.hydrants, param)?;
            self.hydrants.clear();
        }
        if self.state != "menu" {
            { // dog
                let p = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[self.dog.xpos, self.dog.ypos]),
                    scale: ggez::mint::Vector2::from_slice(&[4.0,4.0]),
                    rotation: 0.0,
                    ..Default::default()
                };
                let param = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[self.offset,0.0]),
                    scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                    rotation: 0.0,
                    offset: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                    ..Default::default()
                };
                if self.dog.state == "peeing" {
                        self.dog.pee.add(p);
                        graphics::draw(ctx, &self.dog.pee, param)?;
                        self.dog.pee.clear();
                }
                else if self.dog.state == "walking" {
                    if self.frames % 20 < 5 {
                        self.dog.walk_1.add(p);
                        graphics::draw(ctx, &self.dog.walk_1, param)?;
                        self.dog.walk_1.clear();
                    }
                    else if self.frames % 20 < 10 {
                        self.dog.walk_2.add(p);
                        graphics::draw(ctx, &self.dog.walk_2, param)?;
                        self.dog.walk_2.clear();
                    }
                    else if self.frames % 20 < 15 {
                        self.dog.walk_3.add(p);
                        graphics::draw(ctx, &self.dog.walk_3, param)?;
                        self.dog.walk_3.clear();
                    }
                    else { 
                        self.dog.walk_4.add(p);
                        graphics::draw(ctx, &self.dog.walk_4, param)?;
                        self.dog.walk_4.clear();
                    }
                }
                else if self.dog.state == "stopped" {
                    self.dog.walk_2.add(p);
                    graphics::draw(ctx, &self.dog.walk_2, param)?;
                    self.dog.walk_2.clear(); 
                }
            }
        }

        let _dest_point = ggez::mint::Point2::from_slice(&[650.0,20.0]);
        let dest_point = DrawParam::default().dest(_dest_point);
        let s = format!("Points: {}", self.points);
        //let text = graphics::Text::new(ctx, s.as_str(), &self.font)?;
        let text = graphics::Text::new(s.as_str());
        graphics::draw(ctx, &text, dest_point)?;


        if self.state == "menu" {
                let p = graphics::DrawParam {
                    dest: ggez::mint::Point2::from_slice(&[220.0,50.0]),
                    scale: ggez::mint::Vector2::from_slice(&[6.0,6.0]),
                    rotation: 0.0,
                    ..Default::default()
                };
                self.twoo.add(p);
            let param = graphics::DrawParam {
                //dest: ggez::mint::Point2::from_slice(self.offset/2.0,0.0),
                dest: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                scale: ggez::mint::Vector2::from_slice(&[1.0,1.0]),
                rotation: 0.0,
                offset: ggez::mint::Point2::from_slice(&[0.0,0.0]),
                ..Default::default()
            };

            graphics::draw(ctx, &self.twoo, param)?;
            self.twoo.clear();
            let _dest_point = ggez::mint::Point2::from_slice(&[300.0,500.0]);
            let dest_point = DrawParam::default().dest(_dest_point);
            let s = format!("Previous Score: {}", self.points);
            let text = graphics::Text::new(s.as_str());
            graphics::draw(ctx, &text, dest_point)?;
        }


        let present = graphics::present(ctx);
        if present.is_err() {
            println!("Present is error: {:?}", present);
        }

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {}

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {}

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, _axis: Axis, _value: f32, _id: GamepadId) {
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {}

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        false
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title,
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game,
// * then just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let (mut ctx, mut events_loop) = ContextBuilder::new("thewizzerofoz", "maccam912").build().unwrap();
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so 
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        println!("{:?}", path);
        ggez::filesystem::mount(&mut ctx, &path, true);
    }


    let state = &mut MainState::new(&mut ctx).unwrap();
    if let Err(e) = event::run(&mut ctx, &mut events_loop, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}