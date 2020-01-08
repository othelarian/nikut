use luminance_derive::{Semantics, Vertex};
use rand::Rng;

pub fn new_nb() -> f32 {
    (rand::thread_rng().gen_range(0, 100) as f32) / 100.0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Positions { pos: VertexPosition }

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct Colors {
    #[vertex(normalized = "true")]
    color: VertexColor
}

pub struct TriFull<'a> {
    tri_verts: [Vertex; 6],
    tri_inds: [u32; 6],
    tri_deint_pos_verts: &'a[Positions],
    tri_deint_col_verts: &'a[Colors]
}

pub const TRIS_FULL: TriFull = TriFull {
    tri_verts: [
        Vertex {pos: VertexPosition::new([0.5, -0.5]), rgb: VertexColor::new([0, 255, 0])},
        Vertex {pos: VertexPosition::new([0.0, 0.5]), rgb: VertexColor::new([0, 0, 255])},
        Vertex {pos: VertexPosition::new([-0.5, -0.5]), rgb: VertexColor::new([255, 0, 0])},
        Vertex {pos: VertexPosition::new([-0.5, 0.5]), rgb: VertexColor::new([255, 51, 255])},
        Vertex {pos: VertexPosition::new([0.0, -0.5]), rgb: VertexColor::new([51, 255, 255])},
        Vertex {pos: VertexPosition::new([0.5, 0.5]), rgb: VertexColor::new([51, 51, 255])}
    ],
    tri_inds: [
        0, 1, 2, // First triangle.
        3, 4, 5, // Second triangle.
    ],
    tri_deint_pos_verts: &[
        Positions {pos: VertexPosition::new([0.5, -0.5])},
        Positions {pos: VertexPosition::new([0.0, 0.5])},
        Positions {pos: VertexPosition::new([-0.5, -0.5])},
        Positions {pos: VertexPosition::new([-0.5, 0.5])},
        Positions {pos: VertexPosition::new([0.0, -0.5])},
        Positions {pos: VertexPosition::new([0.5, 0.5])}
    ],
    tri_deint_col_verts: &[
        Colors {color: VertexColor::new([0, 255, 0])},
        Colors {color: VertexColor::new([0, 0, 255])},
        Colors {color: VertexColor::new([255, 0, 0])},
        Colors {color: VertexColor::new([255, 51, 255])},
        Colors {color: VertexColor::new([51, 255, 255])},
        Colors {color: VertexColor::new([51, 51, 255])}
    ]
};

#[derive(Copy, Clone, Debug)]
pub enum TessMethod {
    Direct,
    Indexed,
    DirectDeinter,
    IndexedDeinter
}

impl TessMethod {
    pub fn toggle(self) -> Self {
        match self {
            TessMethod::Direct => TessMethod::Indexed,
            TessMethod::Indexed => TessMethod::DirectDeinter,
            TessMethod::DirectDeinter => TessMethod::IndexedDeinter,
            TessMethod::IndexedDeinter => TessMethod::Direct
        }
    }
}
