#[derive(Debug, Clone)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    matrix: Vec<Vec<f32>>,
}

#[allow(dead_code)]
impl Mat {
    pub fn new(mat: Vec<Vec<f32>>) -> Self {
        let row_len = mat[0].len();
        assert!(mat.iter().all(|r| row_len == r.len()));
        Self {
            rows: mat.len(),
            cols: row_len,
            matrix: mat,
        }
    }

    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut matrix = Self::fill(0.0, rows, cols).matrix;
        for i in 0..rows {
            for j in 0..cols {
                if i == j {
                    matrix[i][j] = 1.0;
                }
            }
        }
        Self { rows, cols, matrix }
    }

    pub fn fill(n: f32, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            matrix: vec![vec![n; cols]; rows],
        }
    }

    pub fn rotation(rows: usize, cols: usize, plane: [usize; 2], theta: f32) -> Self {
        let mut m = Self::identity(rows, cols);
        m.matrix[plane[0]][plane[0]] = theta.cos();
        m.matrix[plane[0]][plane[1]] = -theta.sin();
        m.matrix[plane[1]][plane[0]] = theta.sin();
        m.matrix[plane[1]][plane[1]] = theta.cos();
        m
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}

impl std::fmt::Display for Mat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.matrix {
            write!(f, "[ ")?;
            for col in row {
                write!(f, "{col} ")?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl std::ops::Mul for Mat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);
        let mut m: Vec<Vec<f32>> = Vec::new();
        for i in 0..self.rows {
            m.push(Vec::new());
            for j in 0..rhs.cols {
                m[i].push(
                    (0..self.cols).fold(0.0, |acc, k| acc + self.matrix[i][k] * rhs.matrix[k][j]),
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

impl std::ops::Mul<Vec<f32>> for Mat {
    type Output = Vec<f32>;
    fn mul(self, mut rhs: Vec<f32>) -> Self::Output {
        assert_eq!(rhs.len(), self.cols);
        let v = rhs.clone();
        rhs.truncate(self.rows);
        for i in 0..self.rows {
            rhs[i] = v
                .iter()
                .enumerate()
                .map(|(j, v)| v * self.matrix[i][j])
                .sum();
        }
        rhs
    }
}

impl std::ops::Mul<f32> for Mat {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.matrix[i][j] *= rhs;
            }
        }
        m
    }
}
impl std::ops::Add<f32> for Mat {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.matrix[i][j] += rhs;
            }
        }
        m
    }
}
impl std::ops::Sub<f32> for Mat {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        let mut m = self.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                m.matrix[i][j] -= rhs;
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
        self.matrix
            .iter()
            .enumerate()
            .all(|(i, r)| r.iter().enumerate().all(|(j, v)| *v == other.matrix[i][j]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mat_mul() {
        let a = Mat::new(vec![
            vec![1.0, 2.0],
            vec![-10.0, 4.0],
            vec![2.0, 30.0],
            vec![2.0, 10.0],
        ]);
        let b = Mat::new(vec![vec![2.0, 4.0, -10.0], vec![2.0, 4.0, -20.0]]);
        let c = Mat::new(vec![
            vec![6.0, 12.0, -50.0],
            vec![-12.0, -24.0, 20.0],
            vec![64.0, 128.0, -620.0],
            vec![24.0, 48.0, -220.0],
        ]);
        assert_eq!(a * b, c);
    }
    #[test]
    fn mat_identity() {
        assert_eq!(
            Mat::identity(4, 5),
            Mat::new(vec![
                vec![1.0, 0.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0, 0.0],
            ])
        );
    }
    #[test]
    fn mat_vec_mul() {
        let a = Mat::new(vec![
            vec![1.0, 2.0, 3.0, 2.0],
            vec![2.0, 1.0, 2.0, 0.0],
            vec![3.0, 0.0, 1.0, 2.0],
        ]);
        let b: Vec<f32> = Vec::from([1.0, 2.0, 3.0, 4.0]);
        let c: Vec<f32> = Vec::from([22.0, 10.0, 14.0]);
        assert_eq!(a * b, c);
    }
}
