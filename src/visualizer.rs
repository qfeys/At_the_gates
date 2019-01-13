use context::Context;
use core::battlefield::Battlefield;
use glutin::Event;
use scene::Scene;
use std::fs::metadata;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{process, thread, time};
use types::Time;
use GameCommand;
use GameState;
use UI::gui::Gui;
use UI::main_menu_screen::MainMenuScreen;
use UI::screen::{EventStatus, Screen, ScreenCommand, ScreenType};

fn check_assets_dir() {
    if let Err(e) = metadata("assets") {
        println!("Can`t find 'assets' dir: {}", e);
        println!("Note: see 'Assets' section of README.rst");
        process::exit(1);
    }
}

pub struct Visualizer {
    scene: Option<Scene>,
    gui: Gui,
    popups: Vec<Box<Screen>>,
    should_close: bool,
    last_time: Time,
    context: Context,
    rx: Receiver<ScreenCommand>,
}

impl Visualizer {
    pub fn new() -> Visualizer {
        check_assets_dir();
        let (tx, rx) = channel();
        let mut context = Context::new(tx);
        let last_time = context.current_time();
        Visualizer {
            scene: Option::None,
            gui: Gui::new(&mut context, &GameState::Menu),
            popups: Vec::new(),
            should_close: false,
            last_time: last_time,
            context: context,
            rx: rx,
        }
    }

    pub fn new_gui(&mut self, gamestate: &GameState) {
        self.context.clear();
        self.gui = Gui::new(&mut self.context, gamestate);
    }

    pub fn new_scene(&mut self, battlefield: &Battlefield) {
        self.scene = Option::Some(Scene::new(&mut self.context, battlefield));
    }

    pub fn tick(&mut self, gamestate: &GameState, tx: &Sender<GameCommand>) {
        let max_fps = 60;
        let max_frame_time = time::Duration::from_millis(1000 / max_fps);
        let start_frame_time = time::Instant::now();
        let dtime = self.update_time();
        self.draw(gamestate);
        self.handle_events();
        self.handle_commands(tx);
        let delta_time = start_frame_time.elapsed();
        if max_frame_time > delta_time {
            thread::sleep(max_frame_time - delta_time);
        }
    }

    fn draw(&mut self, gamestate: &GameState) {
        self.context.clear();
        match *gamestate {
            GameState::Battle(ref battlefield) => match self.scene {
                Some(ref mut s) => s.draw(&mut self.context, &battlefield),
                None => panic!("No Scene!"),
            },
            _ => {}
        }
        self.gui.draw(&mut self.context);
        for popup in &mut self.popups {
            popup.tick(&mut self.context);
        }
        self.context.flush();
    }

    fn handle_events(&mut self) {
        let events = self.context.poll_events();
        for event in &events {
            // only window events are handled. DeviceEvents are ignored, Awakended panics
            let event = match event {
                Event::WindowEvent { ref event, .. } => event,
                Event::DeviceEvent { .. } => continue,
                Event::Awakened { .. } => unimplemented!("{:?}", event),
            };
            self.context.handle_event_pre(event);
            let mut event_status = EventStatus::NotHandled;
            for i in (0..self.popups.len()).rev() {
                event_status = self.popups[i].handle_event(&mut self.context, event);
                if event_status == EventStatus::Handled {
                    break;
                }
            }
            if event_status == EventStatus::NotHandled {
                event_status = self.gui.handle_event(&mut self.context, event);
            }
            if event_status == EventStatus::NotHandled && self.scene.is_some() {
                self.scene
                    .as_mut()
                    .unwrap()
                    .handle_event(&mut self.context, event);
            }
            self.context.handle_event_post(event);
        }
    }

    fn handle_commands(&mut self, tx: &Sender<GameCommand>) {
        while let Ok(command) = self.rx.try_recv() {
            match command {
                ScreenCommand::ChangeScreen(screen) => match screen {
                    ScreenType::ShuttingDown => {
                        self.should_close = true;
                        self.popups.clear();
                    }
                    ScreenType::Menu => {
                        tx.send(GameCommand::ChangeState(GameState::Menu));
                    }
                    ScreenType::Battle => {
                        tx.send(GameCommand::ChangeState(GameState::Battle(
                            Battlefield::new(),
                        )));
                    }
                },
                ScreenCommand::PushPopup(popup) => {
                    self.popups.push(popup);
                }
                ScreenCommand::PopPopup => {
                    assert!(self.popups.len() > 0);
                    let _ = self.popups.pop();
                }
            }
        }
    }

    pub fn is_running(&self) -> bool {
        !self.should_close && !self.context.should_close()
    }

    fn update_time(&mut self) -> Time {
        let time = self.context.current_time();
        let dtime = Time {
            n: time.n - self.last_time.n,
        };
        self.last_time = time;
        dtime
    }
}
