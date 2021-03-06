use context::Context;
use glutin::WindowEvent;

#[allow(dead_code)]
pub enum ScreenCommand {
    ChangeScreen(ScreenType),
    PopPopup,
    PushPopup(Box<Screen>),
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ScreenType {
    Menu,
    Battle,
    ShuttingDown,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EventStatus {
    Handled,
    NotHandled,
}

pub trait Screen {
    fn tick(&mut self, context: &mut Context);
    fn handle_event(&mut self, context: &mut Context, event: &WindowEvent) -> EventStatus;
}
