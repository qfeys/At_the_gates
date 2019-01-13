use cgmath::{ortho, Matrix4};
use context::Context;
use glutin::{self, MouseButton, VirtualKeyCode, WindowEvent};
use std::collections::HashMap;
use types::Size2;
use GameState;
use UI::button::{Button, ButtonId, ButtonManager};
use UI::main_menu_screen;
use UI::screen::{EventStatus, Screen, ScreenCommand, ScreenType};
use UI::tactical_screen;

//#[derive(Clone, Debug)]
pub struct Gui {
    button_manager: ButtonManager,
    callbacks: HashMap<ButtonId, Box<dyn Fn(&mut Context) -> ()>>,
}

impl Gui {
    pub fn new(context: &mut Context, gamestate: &GameState) -> Gui {
        match *gamestate {
            GameState::Menu => main_menu_screen::main_menu(context),
            GameState::Battle(_) => tactical_screen::TacticalScreen(context),
        }
    }

    pub fn new_from_buttons(
        button_manager: ButtonManager,
        callbacks: HashMap<ButtonId, Box<dyn Fn(&mut Context) -> ()>>,
    ) -> Gui {
        Gui {
            button_manager,
            callbacks,
        }
    }
    pub fn draw(&self, context: &mut Context) {
        context.set_basic_color([0.0, 0.0, 0.0, 1.0]);
        self.button_manager.draw(context);
    }

    pub fn handle_event(&mut self, context: &mut Context, event: &WindowEvent) -> EventStatus {
        match *event {
            WindowEvent::MouseInput {
                state: glutin::ElementState::Released,
                button: MouseButton::Left,
                ..
            } => self.handle_event_lmb_release(context),
            WindowEvent::KeyboardInput {
                input:
                    glutin::KeyboardInput {
                        state: glutin::ElementState::Released,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => self.handle_event_key_press(context, key),
            _ => EventStatus::NotHandled,
        }
    }

    fn handle_event_lmb_release(&mut self, context: &mut Context) -> EventStatus {
        if let Some(button_id) = self.button_manager.get_clicked_button_id(context) {
            self.handle_event_button_press(context, button_id);
            EventStatus::Handled
        } else {
            EventStatus::NotHandled
        }
    }

    /// Pressing a UI button
    fn handle_event_button_press(&mut self, context: &mut Context, button_id: ButtonId) {
        match self.callbacks.get(&button_id) {
            Some(call) => call(context),
            None => panic!("Not implemented button with id: {}", button_id.id),
        }
    }

    fn handle_event_key_press(
        &mut self,
        context: &mut Context,
        key: VirtualKeyCode,
    ) -> EventStatus {
        match key {
            glutin::VirtualKeyCode::Q | glutin::VirtualKeyCode::Escape => {
                context.add_command(ScreenCommand::ChangeScreen(ScreenType::ShuttingDown));
                EventStatus::Handled
            }
            _ => EventStatus::NotHandled,
        }
    }
}

/// Check if this was a tap or swipe
pub fn is_tap(context: &Context) -> bool {
    let mouse = context.mouse();
    let diff = mouse.pos.v - mouse.last_press_pos.v;
    let tolerance = 20; // TODO: read from config file
    diff.x.abs() < tolerance && diff.y.abs() < tolerance
}

pub fn basic_text_size(context: &Context) -> f32 {
    // TODO: use different value for android
    let lines_per_screen_h = 14.0;
    (context.win_size().h as f32) / lines_per_screen_h
}

pub fn small_text_size(context: &Context) -> f32 {
    basic_text_size(context) / 2.0
}

pub fn get_2d_screen_matrix(win_size: Size2) -> Matrix4<f32> {
    let left = 0.0;
    let right = win_size.w as f32;
    let bottom = 0.0;
    let top = win_size.h as f32;
    let near = -1.0;
    let far = 1.0;
    ortho(left, right, bottom, top, near, far)
}
