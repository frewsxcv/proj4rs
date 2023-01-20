//!
//! Handle Nadgrids
//!
use crate::errors::{Error, Result};

//
// Implement a dummy grid shift operator and provider
//
use std::ops::ControlFlow;

#[derive(PartialEq, Debug)]
pub struct NadGrids {}

impl NadGrids {
    pub fn apply(&self, _inverse: bool, x: f64, y: f64, z: f64) -> Result<(f64, f64, f64)> {
        Ok((x, y, z))
    }

    pub fn new_grid_transform(griddefn: &str) -> Result<Self> {
        // Parse the grid list and return an error
        // if there is any mandatory grid or the list is not terminated by
        // '@null'
        match griddefn.split(',').try_for_each(|s| {
            let s = s.trim();
            if s == "@null" || s == "null" {
                ControlFlow::Break(true)
            } else if s.starts_with('@') {
                // Optional grid
                ControlFlow::Continue(())
            } else {
                // Mand
                ControlFlow::Break(false)
            }
        }) {
            ControlFlow::Break(true) => Ok(Self {}),
            _ => Err(Error::NoNADGridAvailable),
        }
    }
}