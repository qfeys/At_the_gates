use context::Context;
use std::collections::HashMap;
use ui::button::ButtonManager;
use ui::gui::Gui;

pub fn tactical_screen(_context: &mut Context) -> Gui {
    Gui::new_from_buttons(ButtonManager::new(), HashMap::new())
}
