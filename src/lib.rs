pub enum EventType {
    PointerOver,
    PointerEnter,
    PointerDown,
    PointerMove,
    PointerUp,
    PointerCancel,
    PointerOut,
    PointerLeave,
}

pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

pub struct PointerEvent {
    pub event_type: EventType,
    pub screen_x: i32,
    pub screen_y: i32,
    pub client_x: i32,
    pub client_y: i32,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub alt_key: bool,
    pub meta_key: bool,
    pub button: u16,
    pub buttons: u16,
    pub pointer_id: i32,
    pub width: f32,
    pub height: f32,
    pub pressure: f32,
    pub tangential_pressure: f32,
    pub tilt_x: i32,
    pub tilt_y: i32,
    pub twist: i32,
    pub pointer_type: PointerType,
    pub is_primary: bool,
}
