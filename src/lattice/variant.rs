use std::{fmt::Debug, str::FromStr};

use super::{def::BLattice, tetra::Tetragonal};

#[derive(Debug)]
pub enum Lat2 {
    Tetra2(Tetragonal<2>),
}

impl Default for Lat2 {
    fn default() -> Self {
        Lat2::Tetra2(Tetragonal::default())
    }
}

impl Lat2 {
    pub fn lat2(self) -> impl BLattice<2> {
        match self {
            Lat2::Tetra2(lattice) => lattice,
        }
    }
}

#[derive(Debug)]
pub enum Lat3 {
    Tetra3(Tetragonal<3>),
}

impl Default for Lat3 {
    fn default() -> Self {
        Lat3::Tetra3(Tetragonal::default())
    }
}

impl Lat3 {
    pub fn lat3(self) -> impl BLattice<3> {
        match self {
            Lat3::Tetra3(lattice) => lattice,
        }
    }
}

impl FromStr for LatVar {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tetra2" => Ok(LatVar::Lat2(Lat2::Tetra2(Tetragonal::default()))),
            "Tetra3" => Ok(LatVar::Lat3(Lat3::Tetra3(Tetragonal::default()))),
            _ => Err("Lattice Variant Not Found!"),
        }
    }
}

#[derive(Debug)]
pub enum LatVar {
    Lat2(Lat2),
    Lat3(Lat3),
}

impl Default for LatVar {
    fn default() -> Self {
        LatVar::Lat2(Lat2::default())
    }
}

impl LatVar {
    pub fn lat2(self) -> Option<impl BLattice<2>> {
        if let LatVar::Lat2(variant) = self {
            return Some(variant.lat2());
        }
        None
    }

    pub fn lat3(self) -> Option<impl BLattice<3>> {
        if let LatVar::Lat3(variant) = self {
            return Some(variant.lat3());
        }
        None
    }
}
