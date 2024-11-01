//! field module
//!
//! contains several field types

use num::Zero;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct ScalarField<T: Zero + Copy> {
    data: Vec<T>,
    size: [usize; 3],
    r_offset: usize,
    p_offset: usize,
}

impl<T: Zero + Copy> ScalarField<T> {
    pub fn new(
        x_cells: usize,
        y_cells: usize,
        z_cells: usize,
    ) -> Result<ScalarField<T>, anyhow::Error> {
        let size = [x_cells, y_cells, z_cells];
        let r_offset = x_cells;
        let p_offset = x_cells * y_cells;

        let data: Vec<T> = vec![T::zero(); size[0] * size[1] * size[2]];
        Ok(ScalarField {
            data,
            size,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Zero + Copy + Display> Index<(usize, usize, usize)> for ScalarField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
        let (i, j, k) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Zero + Copy + Display> IndexMut<(usize, usize, usize)> for ScalarField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k]
    }
}

impl<T: Zero + Copy + Display> Display for ScalarField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size[0] {
            for j in 0..self.size[1] {
                for k in 0..self.size[2] {
                    write!(
                        f,
                        "ScalarField({}, {}, {}) = {}\n",
                        i,
                        j,
                        k,
                        self[(i, j, k)]
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct VectorField<T: Zero + Copy + Display> {
    data: Vec<[T; 3]>,
    size: [usize; 3],
    r_offset: usize,
    p_offset: usize,
}

impl<T: Zero + Copy + Display> VectorField<T> {
    pub fn new(
        x_cells: usize,
        y_cells: usize,
        z_cells: usize,
    ) -> Result<VectorField<T>, anyhow::Error> {
        let size = [x_cells, y_cells, z_cells];
        let r_offset = x_cells;
        let p_offset = x_cells * y_cells;

        let data: Vec<[T; 3]> = vec![[T::zero(); 3]; size[0] * size[1] * size[2]];
        Ok(VectorField {
            data,
            size,
            r_offset,
            p_offset,
        })
    }
}

impl<T: Zero + Copy + Display> Index<(usize, usize, usize, usize)> for VectorField<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize, usize, usize)) -> &Self::Output {
        let (i, j, k, dir) = idx;
        &self.data[i + self.r_offset * j + self.p_offset * k][dir]
    }
}

impl<T: Zero + Copy + Display> IndexMut<(usize, usize, usize, usize)> for VectorField<T> {
    fn index_mut(&mut self, index: (usize, usize, usize, usize)) -> &mut Self::Output {
        let (i, j, k, dir) = index;
        &mut self.data[i + self.r_offset * j + self.p_offset * k][dir]
    }
}

impl<T: Zero + Copy + Display> Display for VectorField<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size[0] {
            for j in 0..self.size[1] {
                for k in 0..self.size[2] {
                    write!(
                        f,
                        "VectorField({}, {}, {}) = [{}, {}, {}]\n",
                        i,
                        j,
                        k,
                        self[(i, j, k, 0)],
                        self[(i, j, k, 1)],
                        self[(i, j, k, 2)]
                    )?;
                }
            }
        }
        Ok(())
    }
}
