use glutin::{Event, EventsLoop, WindowEvent};
use luminance::shader::program::Program;
use luminance_derive::{Semantics, Vertex};
use luminance_windowing::{WindowDim, WindowOpt};

mod winger;
use winger::{WinSurface};

const VS: &'static str = include_str!("../ressources/simple-vs.glsl");
const FS: &'static str = include_str!("../ressources/simple-fs.glsl");

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
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    rgb: VertexColor
}




fn main() {
    let mut el = EventsLoop::new();
    let mut surface = WinSurface::new(
        &el,
        WindowDim::Windowed(800, 400),
        "test lumglut",
        WindowOpt::default()
    ).expect("Glutin surface creation");
    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();
    //
    //
    //
    //
    let mut back_buffer = surface.back_buffer().unwrap();
    //
    let mut color = [0.0, 0.5, 1.0, 1.0];
    //
    //let mut demo = TessMethod::Direct;
    //
    let mut resized = false;
    let mut quit_app = false;
    'app: loop {
        el.poll_events(|evt| { if let Event::WindowEvent { event, ..} = evt { match event {
            WindowEvent::CloseRequested
            | WindowEvent::Destroyed => quit_app = true,
            //
            //
            WindowEvent::Resized(_) | WindowEvent::HiDpiFactorChanged(_) => resized = true,
            _ => ()
        }}});
        //
        //
        if quit_app { break 'app; }
        if resized {
            back_buffer = surface.back_buffer().unwrap();
            resized = false;
        }
        //
        surface.pipeline_builder();
        //
        //
        surface.swap_buffers();
    }
}
