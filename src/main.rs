#[macro_use]
extern crate gfx;

extern crate cgmath;
extern crate collision;
extern crate gfx_device_gl as gfx_gl;
extern crate gfx_window_glutin as gfx_glutin;
extern crate glutin;
extern crate image;
extern crate rand;
extern crate rusttype;

mod UI;
mod camera;
mod context;
mod core;
mod fs;
mod gen;
mod geom;
mod mesh;
mod mesh_manager;
mod obj;
mod pipeline;
mod scene;
mod texture;
mod types;
mod visualizer;

use scene::Scene;
use std::sync::mpsc::{channel, Receiver};
use visualizer::Visualizer;

pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut visualizer = Visualizer::new();
    let mut game_state = GameState::Menu;
    let (tx, rx) = channel();
    while visualizer.is_running() {
        visualizer.tick(&game_state, &tx);

        /* match game_state {
            GameState::Battle(battlefield) => battlefield.tick(),
            _ => (),
        }*/

        process_commands(&mut game_state, &rx, &mut visualizer);
    }
}

#[derive(Debug)]
pub enum GameState {
    Menu,
    Battle(core::battlefield::Battlefield),
}

pub enum GameCommand {
    ChangeState(GameState),
}

fn process_commands(
    game_state: &mut GameState,
    rx: &Receiver<GameCommand>,
    visualizer: &mut Visualizer,
) {
    while let Ok(command) = rx.try_recv() {
        match command {
            GameCommand::ChangeState(state) => match state {
                GameState::Menu => {
                    if let GameState::Menu = game_state {
                    } else {
                        *game_state = state;
                        visualizer.new_gui(game_state);
                    }
                }
                GameState::Battle(battlefield) => {
                    if let GameState::Battle(_) = game_state {
                    } else {
                        visualizer.new_scene(&battlefield);
                        *game_state = GameState::Battle(battlefield);
                        visualizer.new_gui(game_state);
                    }
                }
            },
        }
    }
}
