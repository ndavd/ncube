use crate::mat::Mat;
use crate::vec::MathOps;
use bevy::prelude::Vec3;

pub trait ExtendedMathOps
where
    Self: Sized,
{
    fn factorial(&self) -> Self;
    fn permute(&self, k: Self) -> Self;
    fn pair_permutations(from: Self, to: Self) -> Vec<(Self, Self)>;
}
impl ExtendedMathOps for usize {
    /// Unoptimized basic factorial implementation
    fn factorial(&self) -> usize {
        let mut f: usize = 1;
        for i in 1..=*self {
            f *= i;
        }
        f
    }
    fn permute(&self, k: Self) -> Self {
        self.factorial() / (k.factorial() * (self - k).factorial())
    }
    /// Generates a vector with the permutations of 2 integers within a range
    fn pair_permutations(from: Self, to: Self) -> Vec<(Self, Self)> {
        (from..=to)
            .flat_map(|i| ((i + 1)..=to).map(|j| (i, j)).collect::<Vec<_>>())
            .collect()
    }
}

pub trait PermuteFour<T> {
    fn permute_four(&self) -> Vec<[T; 4]>;
}

impl<T> PermuteFour<T> for Vec<T>
where
    T: Copy,
{
    fn permute_four(&self) -> Vec<[T; 4]> {
        let to = self.len() - 1;
        (0..=to)
            .flat_map(|i| {
                ((i + 1)..=to).flat_map(move |j| {
                    ((j + 1)..=to).flat_map(move |k| {
                        ((k + 1)..=to).map(move |l| [self[i], self[j], self[k], self[l]])
                    })
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct NCube {
    pub dimensions: usize,
    pub size: f64,
    /// Cartesian coordinates of the vertices of the hypercube.
    pub vertices: NVertices,
    /// Vertex indices of the edges of the hypercube.
    pub edges: NEdges,
    /// Vertex indices of the 2D faces of the hypercube.
    pub faces: NFaces,
}

#[derive(Debug, Clone)]
pub struct NVertices(pub Vec<Vec<f64>>);

#[derive(Debug, Clone)]
/// Each edge is composed of 2 vertices (index)
pub struct NEdges(pub Vec<(usize, usize)>);

#[derive(Debug, Clone)]
/// Each face is composed of 3 vertices (index)
pub struct NFaces(pub Vec<(usize, usize, usize)>);

#[derive(Debug, Clone)]
pub struct NCorrection {
    /// plane.0, plane.1, angle, angular velocity
    pub slowest_rotation: (usize, usize, f64, f64),
    pub rotations: std::collections::HashMap<(usize, usize), (f64, f64)>,
    pub vertices_3d: Vec<Vec3>,
    pub vertices: NVertices,
    pub current_angle: f64,
}

impl std::fmt::Display for NVertices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for edge in &self.0 {
            write!(f, "  [ ")?;
            for i in edge {
                write!(
                    f,
                    "{} ",
                    if *i > 0.0 {
                        format!("+{i}")
                    } else if *i == 0.0 {
                        format!(" {i}")
                    } else {
                        format!("{i}")
                    }
                )?;
            }
            writeln!(f, "],")?;
        }
        writeln!(f, "]")
    }
}

#[allow(dead_code)]
impl NCube {
    /// Creates an `n` dimensional hypercube of size `s`.
    pub fn new(n: usize, s: f64) -> Self {
        let vertices = Self::_vertices(n, s);
        Self {
            dimensions: n,
            size: s,
            faces: Self::_faces(&vertices, n),
            edges: Self::_edges(&vertices, n),
            vertices,
        }
    }

    /// Computes how many m dimensional faces the hypercube has
    pub fn face_count(&self, m: usize) -> usize {
        Self::_face_count(self.dimensions, m)
    }

    fn _face_count(n: usize, m: usize) -> usize {
        2_usize.pow((n - m).try_into().unwrap()) * n.permute(m)
    }

    fn _vertices(n: usize, s: f64) -> NVertices {
        let s = s / 2.0;
        let v_count = Self::_face_count(n, 0);
        let vertices = (0..v_count)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let direction =
                            -1 + 2 * ((i / 2_usize.pow(j.try_into().unwrap())) % 2 == 0) as i8;
                        s * direction as f64
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        NVertices(vertices)
    }

    fn _edges(vertices: &NVertices, n: usize) -> NEdges {
        let e_count = Self::_face_count(n, 1);
        let mut edges = Vec::with_capacity(e_count);
        for i in 0..vertices.0.len() {
            let vertex_a = &vertices.0[i];
            vertices
                .0
                .iter()
                .enumerate()
                .skip(i)
                .for_each(|(j, vertex_b)| {
                    if Vec::shared_dimensions(&[vertex_a, vertex_b]).len() == n - 1 {
                        edges.push((i, j));
                    }
                });
            if edges.len() == e_count {
                break;
            }
        }
        NEdges(edges)
    }

    // NOTE: This was not trivial
    fn _faces(vertices: &NVertices, n: usize) -> NFaces {
        let extract_faces = |vertices: Vec<(usize, &Vec<f64>)>| {
            vertices
                .permute_four()
                .iter()
                .filter(|w| {
                    Vec::shared_dimensions(&w.iter().map(|i| i.1).collect::<Vec<_>>()).len()
                        == n - 2
                })
                .flat_map(|w| [(w[0].0, w[1].0, w[2].0), (w[3].0, w[2].0, w[1].0)])
                .collect::<Vec<_>>()
        };
        let iter = vertices.0.iter().enumerate();
        let faces: Vec<(usize, usize, usize)> = if n == 3 {
            (0..n)
                .flat_map(|d| {
                    let (pos, neg): (Vec<_>, Vec<_>) =
                        iter.clone().partition(|(_, vertex)| vertex[d] > 0.0);
                    [extract_faces(pos), extract_faces(neg)].concat()
                })
                .collect()
        } else {
            usize::pair_permutations(0, n - 1)
                .iter()
                .flat_map(|perm| {
                    let (pos_pos, pos_neg): (Vec<_>, Vec<_>) = iter
                        .clone()
                        .filter(|(_, v)| v[perm.0] > 0.0)
                        .partition(|(_, v)| v[perm.1] > 0.0);
                    let (neg_pos, neg_neg): (Vec<_>, Vec<_>) = iter
                        .clone()
                        .filter(|(_, v)| v[perm.0] < 0.0)
                        .partition(|(_, v)| v[perm.1] > 0.0);
                    [
                        extract_faces(pos_pos),
                        extract_faces(pos_neg),
                        extract_faces(neg_pos),
                        extract_faces(neg_neg),
                    ]
                    .concat()
                })
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect()
        };
        assert_eq!(faces.len(), Self::_face_count(n, 2) * 2);
        NFaces(faces)
    }

    pub fn rotate(&mut self, planes: &Vec<(usize, usize)>, theta_rads: &Vec<f64>) -> &mut Self {
        for vertex in &mut self.vertices.0 {
            *vertex = Mat::rotation(self.dimensions, self.dimensions, planes, theta_rads)
                * vertex.clone();
        }
        self
    }

    pub fn perspective_project_vertices(&self) -> Vec<Vec3> {
        let projection_count = self.dimensions - 3;
        let proj_m = |from_d: usize, to_d: usize, q: f64| {
            let f = self.size / (self.size * 1.5 - q);
            Mat::identity(to_d, from_d) * f
        };
        let mut v = self.vertices.0.clone();
        for i in 0..projection_count {
            let curr_d = self.dimensions - i;
            let target_d = curr_d - 1;
            for v_index in 0..self.vertices.0.len() {
                let m = proj_m(curr_d, target_d, v[v_index][curr_d - 1]);
                v[v_index] = m * v[v_index].clone();
            }
        }
        v.iter()
            .map(|x| Vec3::new(x[0] as f32, x[1] as f32, x[2] as f32))
            .collect()
    }
}

impl NCorrection {
    pub fn increment_angle(&mut self, dt: f64) {
        self.current_angle += self.slowest_rotation.3 * dt;
    }
    pub fn reset_angle(&mut self) {
        self.current_angle = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_vertices() {
        let target_vertices = vec![
            vec![1.0, 1.0, 1.0, 1.0],
            vec![-1.0, 1.0, 1.0, 1.0],
            vec![1.0, -1.0, 1.0, 1.0],
            vec![-1.0, -1.0, 1.0, 1.0],
            vec![1.0, 1.0, -1.0, 1.0],
            vec![-1.0, 1.0, -1.0, 1.0],
            vec![1.0, -1.0, -1.0, 1.0],
            vec![-1.0, -1.0, -1.0, 1.0],
            vec![1.0, 1.0, 1.0, -1.0],
            vec![-1.0, 1.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0, -1.0],
            vec![-1.0, -1.0, 1.0, -1.0],
            vec![1.0, 1.0, -1.0, -1.0],
            vec![-1.0, 1.0, -1.0, -1.0],
            vec![1.0, -1.0, -1.0, -1.0],
            vec![-1.0, -1.0, -1.0, -1.0],
        ];
        let tesseract_vertices = NCube::new(4, 2.0).vertices;
        println!("Tesseract vertices: {tesseract_vertices}");
        assert_eq!(tesseract_vertices.0, target_vertices);
    }
    #[test]
    fn get_face_count() {
        let target_face_count = vec![16, 32, 24, 8, 1];
        let tesseract_face_count = (0..=4)
            .map(|m| NCube::new(4, 1.0).face_count(m))
            .collect::<Vec<_>>();
        println!("Tesseract face count: {tesseract_face_count:?}");
        assert_eq!(target_face_count, tesseract_face_count);
    }
}
