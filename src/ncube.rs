use crate::vec::MathOps;

pub trait ExtendedMathOps {
    fn factorial(&self) -> Self;
    fn chooses(&self, k: Self) -> Self;
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
    fn chooses(&self, k: Self) -> Self {
        self.factorial() / (k.factorial() * (self - k).factorial())
    }
}

#[derive(Debug)]
pub struct NCube {
    pub dimensions: usize,
    pub size: f32,
    /// Cartesian coordinates of the vertices of the hypercube.
    pub vertices: NVertices,
    /// Vertex indices of the edges of the hypercube.
    pub edges: NEdges,
    /// Vertex indices of the 2D faces of the hypercube.
    pub faces: NFaces,
}

#[derive(Debug)]
pub struct NVertices(pub Vec<Vec<f32>>);

#[derive(Debug)]
/// Each edge is composed of 2 vertices (index)
pub struct NEdges(pub Vec<(usize, usize)>);

#[derive(Debug)]
/// Each face is composed of 3 vertices (index)
pub struct NFaces(pub Vec<(usize, usize, usize)>);

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

impl NCube {
    /// Creates an `n` dimensional hypercube of size `s`.
    pub fn new(n: usize, s: f32) -> Self {
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
        2_usize.pow((n - m).try_into().unwrap()) * n.chooses(m)
    }

    fn _vertices(n: usize, s: f32) -> NVertices {
        let s = s / 2.0;
        let v_count = Self::_face_count(n, 0);
        let vertices = (0..v_count)
            .map(|i| {
                (0..n)
                    .map(|j| {
                        let direction =
                            -1 + 2 * ((i / 2_usize.pow(j.try_into().unwrap())) % 2 == 0) as i8;
                        s * direction as f32
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();
        NVertices(vertices)
    }

    fn _edges(vertices: &NVertices, n: usize) -> NEdges {
        let e_count = Self::_face_count(n, 1);
        let mut edges = Vec::new();
        for i in 0..vertices.0.len() {
            let vertex_a = &vertices.0[i];
            vertices
                .0
                .iter()
                .enumerate()
                .skip(i)
                .for_each(|(j, vertex_b)| {
                    if Vec::shared_dimensions(&[vertex_a, vertex_b]) == 2 {
                        edges.push((i, j));
                    }
                });
            if edges.len() == e_count {
                break;
            }
        }
        NEdges(edges)
    }

    fn _faces(vertices: &NVertices, n: usize) -> NFaces {
        // NOTE: This was not trivial
        let mut faces: Vec<(usize, usize, usize)> = Vec::new();
        let extract_faces = |vertices: Vec<(usize, &Vec<f32>)>| {
            vertices
                .windows(4)
                .filter(|w| Vec::shared_dimensions(&w.iter().map(|i| i.1).collect::<Vec<_>>()) == 1)
                .flat_map(|w| [(w[0].0, w[1].0, w[2].0), (w[3].0, w[2].0, w[1].0)])
                .collect::<Vec<_>>()
        };
        (0..n).for_each(|d| {
            let (positive_vertices, negative_vertices): (Vec<_>, Vec<_>) = vertices
                .0
                .iter()
                .enumerate()
                .partition(|(_, vertex)| vertex[d] > 0.0);
            faces.append(&mut extract_faces(positive_vertices));
            faces.append(&mut extract_faces(negative_vertices));
        });
        assert_eq!(faces.len(), Self::_face_count(n, 2) * 2);
        NFaces(faces)
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
