//! an API to improve event handling here!
//! currently WIP!~!
//!
//! pls don't use

use std::collections::HashMap;

use winit::event::ModifiersState;

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyInputState(u8);
impl KeyInputState {
    /// happens when you start the key press
    /// or, when you have the haven't yet let go of the key
    /// can be true while `is_repeated` is true
    pub fn is_pressed(&self) -> bool {
        self.0 & 1 != 0 // if the first bit is set, it's pressed
    }
    /// happens when you press the key again within the `repeated_press_interval`
    /// can be true while `is_pressed` is true
    pub fn is_repeated(&self) -> bool {
        self.0 & 2 != 0 // if the second bit is set, it's repeated
    }
    pub fn is_pressed_but_not_repeated(&self) -> bool {
        self.is_pressed() && !self.is_repeated()
    }
    /// note that this is not the same as `is_up`
    /// and `is_pressed` and `is_held` can be true together
    pub fn is_held(&self) -> bool {
        self.0 & 4 != 0 // if the third bit is set, it's held
    }
    /// when the key is free, note that this is not the same as `is_held`
    /// and `is_released` and `is_up` can be true together
    pub fn is_up(&self) -> bool {
        !self.is_pressed() && !self.is_repeated()
    }
    /// happens when you stop pressing the key
    pub fn is_released(&self) -> bool {
        self.0 & 8 != 0 // if the fourth bit is set, it's released
    }
}

/// Hold state of every `key` and
/// allow for direct access to info about it.
pub struct KeyState {
    modifiers: ModifiersState, // it's a newtype wrapper around u32
    keys: HashMap<winit::event::VirtualKeyCode, KeyInputState>,
}

/// Hold state of every `key` on mouse, `coordinate` of the mouse,
/// etc. and allow for direct access to info about it.
pub struct MouseState {}

pub struct InputQueue {}

pub struct EventHandler {
    pub key_state: KeyState,
    pub mouse_state: MouseState,
    pub input_queue: InputQueue, // stores input history to allow for checking things like mouse move delta and
}
