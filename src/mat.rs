#[macro_export]
macro_rules! emat {
    ($m:ident [ $i:expr ] [ $j:expr ]) => {
        $m.matrix[($i * $m.cols) + $j]
    };
}

#[derive(Debug, Clone)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    matrix: Vec<f64>,
}

#[allow(dead_code)]
impl Mat {
    pub fn new(mat: &[&[f64]]) -> Self {
        let row_len = mat[0].len();
        assert!(mat.iter().all(|r| row_len == r.len()));
        Self {
            rows: mat.len(),
            cols: row_len,
            matrix: mat.concat().to_vec(),
        }
    }

    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut m = Self::fill(0.0, rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                if i == j {
                    emat!(m[i][j]) = 1.0;
                }
            }
        }
        Self {
            rows,
            cols,
            matrix: m.matrix,
        }
    }

    pub fn fill(n: f64, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![n; rows * cols],
        }
    }

    pub fn rotation(
        rows: usize,
        cols: usize,
        planes: &Vec<(usize, usize)>,
        thetas: &Vec<f64>,
    ) -> Self {
        let mut m = Self::identity(rows, cols);
        let assign_element = |element: &mut f64, v: f64| {
            *element = if *element == 0.0 { v } else { *element * v };
        };
        for i in 0..planes.len() {
            assign_element(&mut emat!(m[planes[i].0][planes[i].0]), thetas[i].cos());
            assign_element(&mut emat!(m[planes[i].0][planes[i].1]), -thetas[i].sin());
            assign_element(&mut emat!(m[planes[i].1][planes[i].0]), thetas[i].sin());
            assign_element(&mut emat!(m[planes[i].1][planes[i].1]), thetas[i].cos());
        }
        m
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

impl std::fmt::Display for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.matrix.len() {
            if i % self.cols == 0 {
                write!(f, "[ ")?;
            }
            write!(f, "{} ", self.matrix[i])?;
            if i % self.cols == self.cols - 1 {
                writeln!(f, "]")?;
            }
        }
        Ok(())
    }
}

impl std::ops::Mul for Mat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut m: Vec<f64> = Vec::with_capacity(self.rows * rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                m.push(
                    (0..self.cols).fold(0.0, |acc, k| acc + emat!(self[i][k]) * emat!(rhs[k][j])),
                );
            }
        }
        Self::Output {
            rows: self.rows,
            cols: rhs.cols,
            matrix: m,
        }
    }
}

impl std::ops::Mul<Vec<f64>> for Mat {
    type Output = Vec<f64>;
    fn mul(self, mut rhs: Vec<f64>) -> Self::Output {
        assert_eq!(rhs.len(), self.cols);
        let v = rhs.clone();
        rhs.truncate(self.rows);
        for i in 0..self.rows {
            rhs[i] = v
                .iter()
                .enumerate()
                .map(|(j, v)| v * emat!(self[i][j]))
                .sum();
        }
        rhs
    }
}

impl std::ops::Mul<f64> for Mat {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                emat!(m[i][j]) *= rhs;
            }
        }
        m
    }
}
impl std::ops::Add<f64> for Mat {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                emat!(m[i][j]) += rhs;
            }
        }
        m
    }
}
impl std::ops::Sub<f64> for Mat {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                emat!(m[i][j]) -= rhs;
            }
        }
        m
    }
}

impl std::cmp::PartialEq for Mat {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        self.matrix == other.matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mat_mul() {
        let a = Mat::new(&[&[1.0, 2.0], &[-10.0, 4.0], &[2.0, 30.0], &[2.0, 10.0]]);
        let b = Mat::new(&[&[2.0, 4.0, -10.0], &[2.0, 4.0, -20.0]]);
        let c = Mat::new(&[
            &[6.0, 12.0, -50.0],
            &[-12.0, -24.0, 20.0],
            &[64.0, 128.0, -620.0],
            &[24.0, 48.0, -220.0],
        ]);
        assert_eq!(a * b, c);
    }
    #[test]
    fn mat_identity() {
        assert_eq!(
            Mat::identity(4, 5),
            Mat::new(&[
                &[1.0, 0.0, 0.0, 0.0, 0.0],
                &[0.0, 1.0, 0.0, 0.0, 0.0],
                &[0.0, 0.0, 1.0, 0.0, 0.0],
                &[0.0, 0.0, 0.0, 1.0, 0.0],
            ])
        );
    }
    #[test]
    fn mat_vec_mul() {
        let a = Mat::new(&[
            &[1.0, 2.0, 3.0, 2.0],
            &[2.0, 1.0, 2.0, 0.0],
            &[3.0, 0.0, 1.0, 2.0],
        ]);
        let b: Vec<f64> = Vec::from([1.0, 2.0, 3.0, 4.0]);
        let c: Vec<f64> = Vec::from([22.0, 10.0, 14.0]);
        assert_eq!(a * b, c);
    }
}
