use bevy_ecs::entity::Entity;
use bevy_input::{
    keyboard::{KeyCode, KeyboardInput},
    pointer::{
        MouseButton, PenButton, PointerButton, PointerEvent, PointerEventType, PointerId, Tool,
    },
    ButtonState,
};
use bevy_math::{DVec2, Vec2};
use bevy_window::{CursorIcon, EnabledButtons, WindowLevel, WindowTheme};

pub fn convert_keyboard_input(
    keyboard_input: &winit::event::KeyEvent,
    window: Entity,
) -> KeyboardInput {
    KeyboardInput {
        physical_key: Some(KeyCode::A),
        // physical_key: convert_virtual_key_code(keyboard_input.physical_key),
        state: convert_element_state(keyboard_input.state),
        // key_code: convert_key_code(keyboard_input.logical_key),
        key_code: Some(KeyCode::A),
        window,
    }
}

pub fn convert_element_state(element_state: winit::event::ElementState) -> ButtonState {
    match element_state {
        winit::event::ElementState::Pressed => ButtonState::Pressed,
        winit::event::ElementState::Released => ButtonState::Released,
    }
}

pub fn convert_mouse_button(mouse_button: winit::event::MouseButton) -> MouseButton {
    match mouse_button {
        winit::event::MouseButton::Left => MouseButton::Left,
        winit::event::MouseButton::Right => MouseButton::Right,
        winit::event::MouseButton::Middle => MouseButton::Middle,
        winit::event::MouseButton::Back => MouseButton::Back,
        winit::event::MouseButton::Forward => MouseButton::Forward,
        winit::event::MouseButton::Other(val) => MouseButton::Other(val),
    }
}

pub fn convert_window_level(window_level: WindowLevel) -> winit::window::WindowLevel {
    match window_level {
        WindowLevel::AlwaysOnBottom => winit::window::WindowLevel::AlwaysOnBottom,
        WindowLevel::Normal => winit::window::WindowLevel::Normal,
        WindowLevel::AlwaysOnTop => winit::window::WindowLevel::AlwaysOnTop,
    }
}

pub fn convert_winit_theme(theme: winit::window::Theme) -> WindowTheme {
    match theme {
        winit::window::Theme::Light => WindowTheme::Light,
        winit::window::Theme::Dark => WindowTheme::Dark,
    }
}

pub fn convert_window_theme(theme: WindowTheme) -> winit::window::Theme {
    match theme {
        WindowTheme::Light => winit::window::Theme::Light,
        WindowTheme::Dark => winit::window::Theme::Dark,
    }
}

pub fn convert_enabled_buttons(enabled_buttons: EnabledButtons) -> winit::window::WindowButtons {
    let mut window_buttons = winit::window::WindowButtons::empty();
    if enabled_buttons.minimize {
        window_buttons.insert(winit::window::WindowButtons::MINIMIZE);
    }
    if enabled_buttons.maximize {
        window_buttons.insert(winit::window::WindowButtons::MAXIMIZE);
    }
    if enabled_buttons.close {
        window_buttons.insert(winit::window::WindowButtons::CLOSE);
    }
    window_buttons
}

pub fn convert_pointer_input(
    event: winit::event::PointerEvent,
    pointer_id: winit::event::PointerId,
    sf: f64,
) -> Option<PointerEvent> {
    let pointer = match pointer_id {
        winit::event::PointerId::Cursor => PointerId::Mouse,
        winit::event::PointerId::Touch { finger } => PointerId::Touch { finger },
        winit::event::PointerId::Pen { tool } => PointerId::Pen {
            tool: match tool {
                winit::event::Tool::Pen => Tool::Pen,
                winit::event::Tool::Eraser => Tool::Eraser,
            },
        },
    };
    let event = match event {
        winit::event::PointerEvent::Entered => PointerEventType::Entered,
        winit::event::PointerEvent::Left => PointerEventType::Entered,
        winit::event::PointerEvent::Moved {
            position,
            force,
            tilt,
        } => PointerEventType::Moved {
            position: (DVec2::new(position.x, position.y) / sf).as_vec2(),
            force: force.as_ref().map(winit::event::Force::normalized),
        },
        winit::event::PointerEvent::Button {
            button,
            state,
            position,
            force,
            tilt,
        } => PointerEventType::Button {
            button: match button {
                winit::event::PointerButton::Mouse(mouse) => {
                    PointerButton::Mouse(convert_mouse_button(mouse))
                }
                winit::event::PointerButton::Touch => PointerButton::Touch,
                winit::event::PointerButton::Pen(_) => PointerButton::Pen(PenButton::Pressed),
            },
            pressed: matches!(state, winit::event::ElementState::Pressed),
            position: position.map(|x| (DVec2::new(x.x, x.y) / sf).as_vec2()),
            force: force.as_ref().map(winit::event::Force::normalized),
        },
        _ => return None,
    };
    Some(PointerEvent { pointer, event })
}
