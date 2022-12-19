use std::ops::Deref;
use super::field::{Field, FieldCoordinate, FieldError};
use super::setup_state::ShipState::{self, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VisibilityState {
    Unknown,
    AutoShown,
    MissShown,
    HitShown,
}

impl Default for VisibilityState {
    fn default() -> Self {
        VisibilityState::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayingMessage {
    WaterHit,
    ShipHit,
    ShipSunk,
    IneffectiveHit,
}

impl PlayingMessage {
    pub fn turn_ends(&self) -> bool {
        match self {
            Self::IneffectiveHit => false,
            _ => true,
        }
    }
}

pub type PlayingField = Field<VisibilityState>;

pub fn check_hit(visibility_field: &mut PlayingField, opponent_field: &Field<ShipState>, attempt: FieldCoordinate) -> Result<PlayingMessage, FieldError> {
    match visibility_field.get(attempt)? {
        Unknown => (),
        _ => return Ok(PlayingMessage::IneffectiveHit),
    };

    match opponent_field.get(attempt)?.deref() {
        NoShip => Ok(PlayingMessage::WaterHit),
        Ship(id) => Ok(PlayingMessage::ShipHit),
    }
}

