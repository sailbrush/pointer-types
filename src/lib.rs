pub use keyboard_types;

use keyboard_types::Modifiers;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum PointerButton {
    None,

    /// The primary pointer button, usually the left mouse button.
    Primary,

    /// The secondary pointer bytton, usually the right mouse button.
    Secondary,

    /// The auxilary pointer button, usually the wheel or middle mouse button.
    Auxilary,

    /// The fourth button, usually the back button
    X1,

    /// The fifth button, usually the forward button
    X2,
}

impl PointerButton {
    /// Returns `true` if this is `PointerButton::Primary`.
    #[inline]
    pub fn is_primary(self) -> bool {
        self == PointerButton::Primary
    }

    /// Returns `true` if this is `PointerButton::Secondary`.
    #[inline]
    pub fn is_secondary(self) -> bool {
        self == PointerButton::Secondary
    }

    /// Returns `true` if this is `PointerButton::Auxilary`.
    #[inline]
    pub fn is_auxilary(self) -> bool {
        self == PointerButton::Auxilary
    }

    /// Returns `true` if this is `PointerButton::X1`.
    #[inline]
    pub fn is_x1(self) -> bool {
        self == PointerButton::X1
    }

    /// Returns `true` if this is `PointerButton::X2`.
    #[inline]
    pub fn is_x2(self) -> bool {
        self == PointerButton::X2
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct PointerButtons(u8);

impl PointerButtons {
    /// Create a new empty set.
    #[inline]
    pub fn new() -> PointerButtons {
        PointerButtons(0)
    }

    /// Add the `button` to the set.
    #[inline]
    pub fn insert(&mut self, button: PointerButton) {
        self.0 |= 1.min(button as u8) << button as u8;
    }

    /// Remove the `button` from the set.
    #[inline]
    pub fn remove(&mut self, button: PointerButton) {
        self.0 &= !(1.min(button as u8) << button as u8);
    }

    /// Builder-style method for adding the `button` to the set.
    #[inline]
    pub fn with(mut self, button: PointerButton) -> PointerButtons {
        self.0 |= 1.min(button as u8) << button as u8;
        self
    }

    /// Builder-style method for removing the `button` from the set.
    #[inline]
    pub fn without(mut self, button: PointerButton) -> PointerButtons {
        self.0 &= !(1.min(button as u8) << button as u8);
        self
    }

    /// Returns `true` if the `button` is in the set.
    #[inline]
    pub fn contains(self, button: PointerButton) -> bool {
        (self.0 & (1.min(button as u8) << button as u8)) != 0
    }

    /// Returns `true` if the set is empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if all the `buttons` are in the set.
    #[inline]
    pub fn is_superset(self, buttons: PointerButtons) -> bool {
        self.0 & buttons.0 == buttons.0
    }

    /// Returns `true` if `PointerButton::Primary` is in the set.
    #[inline]
    pub fn has_primary(self) -> bool {
        self.contains(PointerButton::Primary)
    }

    /// Returns `true` if `PointerButton::Secondary` is in the set.
    #[inline]
    pub fn has_secondary(self) -> bool {
        self.contains(PointerButton::Secondary)
    }

    /// Returns `true` if `PointerButton::Auxilary` is in the set.
    #[inline]
    pub fn has_auxilary(self) -> bool {
        self.contains(PointerButton::Auxilary)
    }

    /// Returns `true` if `PointerButton::X1` is in the set.
    #[inline]
    pub fn has_x1(self) -> bool {
        self.contains(PointerButton::X1)
    }

    /// Returns `true` if `PointerButton::X2` is in the set.
    #[inline]
    pub fn has_x2(self) -> bool {
        self.contains(PointerButton::X2)
    }

    /// Adds all the `buttons` to the set.
    #[inline]
    pub fn extend(&mut self, buttons: PointerButtons) {
        self.0 |= buttons.0;
    }

    /// Returns a union of the values in `self` and `other`.
    #[inline]
    pub fn union(mut self, other: PointerButtons) -> PointerButtons {
        self.0 |= other.0;
        self
    }

    /// Clear the set.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl std::fmt::Debug for PointerButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PointerButtons({:05b})", self.0 >> 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

#[derive(Debug, Clone, Copy)]
pub struct RawPointerEvent {
    /// The horizontal coordinate of the pointer event in the window.
    pub window_pos_x: f32,

    /// The vertical coordinate of the pointer event in the window.
    pub window_pos_y: f32,

    /// The horizontal scroll amount.
    pub wheel_x: f32,

    /// The vertical scroll amount.
    pub wheel_y: f32,

    /// The button responsible for a pointer event.
    /// This will always be `None` for a pointer_move event.
    pub button: PointerButton,

    /// Pointer buttons being held down during a move or after a click event.
    /// It will contain the button that triggered a pointer_down event,
    /// and it will not contain the button that triggered a pointer_up event.
    pub buttons: PointerButtons,

    /// Keyboard modifier keys pressed at the time of the event.
    pub mods: Modifiers,

    /// The number of clicks associated with this event.
    /// This will always be 0 for pointer_up and pointer_move events.
    pub count: u8,

    /// This is set to `true` if the pointer event caused the window to gain focus.
    pub focus: bool,

    /// The width, in CSS pixels, of the contact geometry of a `Touch` pointer.
    pub width: u16,

    /// The height, in CSS pixels, of the contact geometry of a `Touch` pointer.
    pub height: u16,

    /// The normalized pressure of the pointer input in the range 0 to 1, where 0 and 1 represent
    /// the minimum and maximum pressure the hardware is capable of detecting, respectively.
    pub pressure: f32,

    /// The normalized tangential pressure of the pointer input
    /// in the range -1 to 1, where 0 is the neutral position of the control.
    pub tangential_pressure: f32,

    /// The tilt of the pen in the X axis, from -1 to 1.
    pub tilt_x: f32,

    /// The tilt of the pen in the Y axis, from -1 to 1.
    pub tilt_y: f32,

    /// The clockwise rotation of the pointer (e.g. pen stylus) around
    /// its major axis in degrees, with a value in the range 0 to 359.
    pub twist: f32,

    /// Indicates the device type that caused the event.
    pub pointer_type: PointerType,
}
