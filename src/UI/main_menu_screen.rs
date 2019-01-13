use cgmath::Vector2;
use context::Context;
use glutin::ElementState::Released;
use glutin::{self, MouseButton, VirtualKeyCode, WindowEvent};
use std::collections::HashMap;
use types::ScreenPos;
use UI::button::{Button, ButtonId, ButtonManager};
use UI::gui::{is_tap, Gui};
use UI::screen::{EventStatus, Screen, ScreenCommand, ScreenType};

#[derive(Clone, Debug)]
pub struct MainMenuScreen {
    button_start_hotseat_id: ButtonId,
    button_start_vs_ai_id: ButtonId,
    button_map_id: ButtonId,
    button_manager: ButtonManager,
    selected_map_index: usize,
}

pub fn main_menu(context: &mut Context) -> Gui {
    let mut button_manager = ButtonManager::new();
    let mut callbacks: HashMap<ButtonId, Box<dyn Fn(&mut Context) -> ()>> = HashMap::new();
    // TODO: Use relative coords in ScreenPos - x: [0.0, 1.0], y: [0.0, 1.0]
    // TODO: Add analog of Qt::Alignment
    let mut button_pos = ScreenPos {
        v: Vector2 { x: 10, y: 10 },
    };
    let button_start_hotseat_id =
        button_manager.add_button(Button::new(context, "[start test]", button_pos));
    let call: Box<dyn Fn(&mut Context) -> ()> = Box::new(start_battle);
    callbacks.insert(button_start_hotseat_id, call);
    // TODO: Add something like QLayout
    let vstep = button_manager.buttons()[&button_start_hotseat_id].size().h;
    let vstep = (vstep as f32 * 1.5) as i32;
    button_pos.v.y += vstep;
    let button_start_vs_ai_id =
        button_manager.add_button(Button::new(context, "[start nothing]", button_pos));
    Gui::new_from_buttons(button_manager, callbacks)
}

fn start_battle(context: &mut Context) {
    context.add_command(ScreenCommand::ChangeScreen(ScreenType::Battle));
}

impl MainMenuScreen {
    fn handle_event_lmb_release(&mut self, context: &mut Context) {
        if !is_tap(context) {
            return;
        }
        if let Some(button_id) = self.button_manager.get_clicked_button_id(context) {
            self.handle_event_button_press(context, button_id);
        }
    }

    #[deprecated]
    fn handle_event_button_press(&mut self, context: &mut Context, button_id: ButtonId) {
        /*         let map_name = self.map_names[self.selected_map_index].to_string();
        let mut core_options = Options {
            game_type: GameType::Hotseat,
            map_name: map_name,
            players_count: 2,
        }; 
        if button_id == self.button_start_hotseat_id {
            let tactical_screen = Box::new(TacticalScreen::new(context, &core_options));
            context.add_command(ScreenCommand::PushScreen(tactical_screen));
        } else if button_id == self.button_start_vs_ai_id {
            core_options.game_type = GameType::SingleVsAi;
            let tactical_screen = Box::new(TacticalScreen::new(context, &core_options));
            context.add_command(ScreenCommand::PushScreen(tactical_screen));
        } else if button_id == self.button_map_id {
            self.selected_map_index += 1;
            if self.selected_map_index == self.map_names.len() {
                self.selected_map_index = 0;
            }
            let text = &format!("[map: {}]", self.map_names[self.selected_map_index]);
            let pos = self.button_manager.buttons()[&self.button_map_id].pos();
            let button_map = Button::new(context, text, pos);
            self.button_manager.remove_button(self.button_map_id);
            self.button_map_id = self.button_manager.add_button(button_map);
        } else {
            panic!("Bad button id: {}", button_id.id);
        }*/
    }

    fn handle_event_key_press(&mut self, context: &mut Context, key: VirtualKeyCode) {
        match key {
            glutin::VirtualKeyCode::Q | glutin::VirtualKeyCode::Escape => {
                context.add_command(ScreenCommand::ChangeScreen(ScreenType::ShuttingDown));
            }
            _ => {}
        }
    }
}

impl Screen for MainMenuScreen {
    fn tick(&mut self, context: &mut Context) {
        context.clear();
        context.set_basic_color([0.0, 0.0, 0.0, 1.0]);
        self.button_manager.draw(context);
    }

    fn handle_event(&mut self, context: &mut Context, event: &WindowEvent) -> EventStatus {
        match *event {
            WindowEvent::MouseInput {
                state: Released,
                button: MouseButton::Left,
                ..
            } => {
                self.handle_event_lmb_release(context);
            }
            WindowEvent::Touch(glutin::Touch { phase, .. }) => {
                if phase == glutin::TouchPhase::Ended {
                    self.handle_event_lmb_release(context);
                }
            }
            WindowEvent::KeyboardInput {
                input:
                    glutin::KeyboardInput {
                        state: Released,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => {
                self.handle_event_key_press(context, key);
            }
            _ => {}
        }
        EventStatus::Handled
    }
}
