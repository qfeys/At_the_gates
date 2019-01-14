use cgmath::Vector2;
use context::Context;
use std::collections::HashMap;
use types::ScreenPos;
use ui::button::{Button, ButtonId, ButtonManager};
use ui::gui::Gui;
use ui::screen::{ScreenCommand, ScreenType};

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
    let _button_start_vs_ai_id =
        button_manager.add_button(Button::new(context, "[start nothing]", button_pos));
    Gui::new_from_buttons(button_manager, callbacks)
}

fn start_battle(context: &mut Context) {
    context.add_command(ScreenCommand::ChangeScreen(ScreenType::Battle));
}
