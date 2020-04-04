pub struct EventPointerMotion {
    pub vertical: i32,
    pub horizontal: i32,
}

pub struct EventPointerButton {
    pub code: u32,
    pub pressed: bool,
}

pub struct EventPointerScroll {
    pub vertical: i32,
    pub horizontal: i32,
}

pub struct EventKey {
    pub code: u32,
    pub pressed: bool,
}

pub struct EventResize {
    pub height: i32,
    pub width: i32,
}

pub enum Event {
    PointerMotion(EventPointerMotion),
    PointerButton(EventPointerButton),
    PointerScroll(EventPointerScroll),
    Key(EventKey),
    Resize(EventResize),
    Focus,
    Unfocus,
    Maximize,
    Minimize,
    Restore,
    Close,
}
