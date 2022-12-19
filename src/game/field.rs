use std::convert;
use std::ops::Deref;
use std::ops::DerefMut;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FieldError {
    #[error("Cell out of bound ({row}:{col} in {size}:{size})")]
    CellOutOfBound { size: usize, row: usize, col: usize },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field<T> {
    field: Box<[T]>,
    size: usize,
}

impl<T> Field<T> {
    pub fn from_indexes<F>(size: usize, generator: F) -> Self
    where
        F: Fn(usize, usize) -> T,
    {
        let contents: Vec<T> = (0..size)
            .into_iter()
            .flat_map(move |row| (0..size).into_iter().map(move |col| (row, col)))
            .map(move |(row, col)| generator(row, col))
            .collect();
        Self {
            field: contents.into_boxed_slice(),
            size,
        }
    }

    pub fn from_nested_slices(field: Vec<Vec<T>>) -> Self {
        let size = (*field).len();
        let contents: Vec<T> = field
            .into_iter()
            .flat_map(|row| {
                assert_eq!(row.len(), size);
                row.into_iter()
            })
            .collect();
        Self {
            field: contents.into_boxed_slice(),
            size,
        }
    }

    fn get_raw_ref<'a>(&'a self, coord: FieldCoordinate) -> Result<&'a T, FieldError> {
        let FieldCoordinate { row, col } = coord;
        if row > self.size || col > self.size {
            Err(FieldError::CellOutOfBound {
                size: self.size,
                row,
                col,
            })?;
        }
        Ok(&self.field[row * self.size + col])
    }

    fn get_raw_mut<'a>(&'a mut self, coord: FieldCoordinate) -> Result<&'a mut T, FieldError> {
        let FieldCoordinate { row, col } = coord;
        if row > self.size || col > self.size {
            Err(FieldError::CellOutOfBound {
                size: self.size,
                row,
                col,
            })?;
        }
        Ok(&mut self.field[row * self.size + col])
    }

    pub fn get<'a>(&'a self, coord: FieldCoordinate) -> Result<FieldCell<'a, T>, FieldError> {
        let FieldCoordinate { row, col } = coord;
        if row > self.size || col > self.size {
            Err(FieldError::CellOutOfBound {
                size: self.size,
                row,
                col,
            })?;
        }
        Ok(FieldCell {
            field: self,
            coord: FieldCoordinate { row, col },
        })
    }

    pub fn get_mut<'a>(
        &'a mut self,
        coord: FieldCoordinate,
    ) -> Result<FieldCellMut<'a, T>, FieldError> {
        let FieldCoordinate { row, col } = coord;
        if row > self.size || col > self.size {
            Err(FieldError::CellOutOfBound {
                size: self.size,
                row,
                col,
            })?;
        }
        Ok(FieldCellMut {
            field: self,
            coord: FieldCoordinate { row, col },
        })
    }
}

impl<T: Default> Field<T> {
    pub fn default_field(size: usize) -> Self {
        let mut vec: Vec<T> = Vec::new();
        vec.resize_with(size * size, T::default);
        Self {
            field: vec.into_boxed_slice(),
            size,
        }
    }
}

impl<T> AsRef<[T]> for Field<T> {
    fn as_ref(&self) -> &[T] {
        self.field.as_ref()
    }
}

impl<T> AsMut<[T]> for Field<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.field.as_mut()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

impl Direction {
    pub fn change_coords(&self, coord: FieldCoordinate) -> FieldCoordinate {
        let FieldCoordinate { row, col } = coord;
        match self {
            Direction::UpLeft => FieldCoordinate::new(row - 1, col - 1),
            Direction::Up => FieldCoordinate::new(row - 1, col),
            Direction::UpRight => FieldCoordinate::new(row - 1, col + 1),
            Direction::Right => FieldCoordinate::new(row, col + 1),
            Direction::DownRight => FieldCoordinate::new(row + 1, col + 1),
            Direction::Down => FieldCoordinate::new(row + 1, col),
            Direction::DownLeft => FieldCoordinate::new(row + 1, col - 1),
            Direction::Left => FieldCoordinate::new(row, col - 1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FieldCoordinate {
    pub row: usize,
    pub col: usize,
}

impl FieldCoordinate {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Into<(usize, usize)> for FieldCoordinate {
    fn into(self) -> (usize, usize) {
        (self.row, self.col)
    }
}

pub struct FieldCell<'a, T> {
    field: &'a Field<T>,
    coord: FieldCoordinate,
}

impl<'a, T> FieldCell<'a, T> {
    pub fn get_neighbour(&'a self, direction: Direction) -> Option<FieldCell<'a, T>> {
        let coord = direction.change_coords(self.coord);
        self.field.get(coord).ok()
    }

    pub fn neighbours(&'a self) -> impl Iterator<Item = FieldCell<'a, T>> {
        [
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
        ]
        .into_iter()
        .map(|dir| dir.change_coords(self.coord))
        .flat_map(|coord| self.field.get(coord))
    }
}

pub struct FieldCellMut<'a, T> {
    field: &'a mut Field<T>,
    coord: FieldCoordinate,
}

impl<'a, T> FieldCellMut<'a, T> {
    pub fn get_neighbours(&'a mut self, direction: Direction) -> Option<FieldCellMut<'a, T>> {
        let coord = direction.change_coords(self.coord);
        self.field.get_mut(coord).ok()
    }
}

impl<'a, T> Deref for FieldCell<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.field
            .get_raw_ref(self.coord)
            .expect("Cell pointed to by FieldCell is valid")
    }
}

impl<'a, T> Deref for FieldCellMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.field
            .get_raw_ref(self.coord)
            .expect("Cell pointed to by FieldCellMut is valid")
    }
}

impl<'a, T> DerefMut for FieldCellMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.field
            .get_raw_mut(self.coord)
            .expect("Cell pointed to by FieldCellMut is valid")
    }
}

impl<'a, T> convert::Into<FieldCell<'a, T>> for FieldCellMut<'a, T> {
    fn into(self) -> FieldCell<'a, T> {
        let Self { field, coord } = self;
        FieldCell { field, coord }
    }
}
