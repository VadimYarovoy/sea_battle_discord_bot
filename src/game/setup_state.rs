use std::{num::NonZeroUsize, ops::Deref};

use super::field::{Field, FieldCell};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShipId(NonZeroUsize);

#[derive(Debug, PartialEq, Eq)]
pub enum ShipState {
    NoShip,
    Ship(ShipId),
}

impl Default for ShipState {
    fn default() -> Self {
        ShipState::NoShip
    }
}

pub type SetupField = Field<ShipState>;

impl SetupField {
    // pub fn ship_cells<'a>(&'a self, id: ShipId) -> impl Iterator<Item = FieldCell<'a, ShipState>> {
    // }
}

impl<'a> FieldCell<'a, ShipState> {
    pub fn has_ship(&'a self) -> Option<ShipId> {
        match self.deref() {
            ShipState::NoShip => None,
            ShipState::Ship(id) => Some(id.clone()),
        }
    }

    pub fn neighbouring_ships(&self) -> Vec<ShipId> {
        self.neighbours()
            .into_iter()
            .flat_map(|cell| cell.has_ship())
            .collect()
    }
}
