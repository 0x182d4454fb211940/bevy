use bevy_ecs::prelude::Event;
use bevy_math::Vec2;
use bevy_reflect::Reflect;

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub struct PointerEvent {
    pub pointer: PointerId,
    pub event: PointerEventType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum PointerId {
    Mouse,
    Touch { finger: u64 },
    Pen { tool: Tool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum Tool {
    Pen,
    Eraser,
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum PointerEventType {
    Entered,
    Left,
    Moved {
        position: Vec2,
        force: Option<f64>,
    },
    Button {
        button: PointerButton,
        pressed: bool,
        position: Option<Vec2>,
        force: Option<f64>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum PointerButton {
    Mouse(MouseButton),
    // Touch (probably) doesn't have any variations, different fingers
    // are different pointers.
    Touch,
    Pen(PenButton),
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Debug, PartialEq)]
#[cfg_attr(
    feature = "serialize",
    derive(serde::Serialize, serde::Deserialize),
    reflect(Serialize, Deserialize)
)]
pub enum PenButton {
    Touch,
    Side,
}
