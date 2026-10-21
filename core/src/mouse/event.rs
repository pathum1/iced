use crate::Point;

use super::Button;

/// A mouse event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// The mouse cursor entered the window.
    CursorEntered,

    /// The mouse cursor left the window.
    CursorLeft,

    /// The mouse cursor was moved
    CursorMoved {
        /// The new position of the mouse cursor
        position: Point,
    },

    /// A mouse button was pressed.
    ButtonPressed(Button),

    /// A mouse button was released.
    ButtonReleased(Button),

    /// The mouse wheel was scrolled.
    WheelScrolled {
        /// The scroll movement.
        delta: ScrollDelta,
    },

    /// A touchpad pinch (magnification) gesture was performed.
    ///
    /// Supported on macOS. The underlying windowing layer reports these, but
    /// they were previously discarded before reaching the application.
    PinchGesture {
        /// The change in magnification since the last event.
        ///
        /// Positive values zoom in, negative values zoom out. A full pinch
        /// typically accumulates to around 1.0.
        delta: f32,

        /// The phase of the gesture.
        phase: GesturePhase,
    },

    /// A touchpad rotation gesture was performed.
    RotationGesture {
        /// The change in rotation since the last event, in degrees.
        delta: f32,

        /// The phase of the gesture.
        phase: GesturePhase,
    },

    /// A touchpad "smart magnify" gesture was performed.
    ///
    /// On macOS this is a two-finger double tap, conventionally used to
    /// zoom a region to fit.
    SmartMagnify,
}

/// The phase of a continuous touchpad gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GesturePhase {
    /// The gesture has begun.
    Started,

    /// The gesture is in progress.
    Moved,

    /// The gesture has finished.
    Ended,

    /// The gesture was cancelled by the system.
    Cancelled,
}

/// A scroll movement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDelta {
    /// A line-based scroll movement
    Lines {
        /// The number of horizontal lines scrolled
        x: f32,

        /// The number of vertical lines scrolled
        y: f32,
    },
    /// A pixel-based scroll movement
    Pixels {
        /// The number of horizontal pixels scrolled
        x: f32,
        /// The number of vertical pixels scrolled
        y: f32,
    },
}
