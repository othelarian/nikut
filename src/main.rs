use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder};
use luminance_windowing::{WindowDim, WindowOpt};

mod winger;
use winger::WinSurface;
//use winger::{WinSurface, ContextTracker};

mod in_utils;
use in_utils::{TRIS_FULL, Semantics, TessMethod, new_nb};

const VS: &'static str = include_str!("../ressources/simple-vs.glsl");
const FS: &'static str = include_str!("../ressources/simple-fs.glsl");

fn main() {
    let el = EventLoop::new();
    //let mut ctx_tracker = ContextTracker::default();
    //let mut windows = std::collections::HashMap::new();

    /*
    for win_idx in 0..3 {
        let surface = WinSurface::new(
            &el,
            WindowDim::Windowed(800, 400),
            &format!("Test Lumglut multiWin #{}", win_idx+1),
            WindowOpt::default()
        ).expect("Glutin surface creation");
        windows.insert(win_idx, surface);
    }
    */
    //
    let mut surface = WinSurface::new(
        &el,
        WindowDim::Windowed(800, 400),
        "Test Lumglut hi",
        WindowOpt::default()
    ).expect("Glutin surface creation");
    //
    //


    // program & tris
    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    let direct_tris = TessBuilder::new(&mut surface)
        .add_vertices(TRIS_FULL.tri_verts.clone())
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
        let indexed_tris = TessBuilder::new(&mut surface)
        .add_vertices(TRIS_FULL.tri_verts)
        .set_indices(TRIS_FULL.tri_inds)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
        let direct_deint_tris = TessBuilder::new(&mut surface)
        .add_vertices(TRIS_FULL.tri_deint_pos_verts)
        .add_vertices(TRIS_FULL.tri_deint_col_verts)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
        let indexed_deint_tris = TessBuilder::new(&mut surface)
        .add_vertices(TRIS_FULL.tri_deint_pos_verts)
        .add_vertices(TRIS_FULL.tri_deint_col_verts)
        .set_indices(TRIS_FULL.tri_inds)
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();

    //
    //
    //
    let mut back_buffer = surface.back_buffer().unwrap();
    let mut color = [0.0, 0.5, 1.0, 1.0];
    let mut demo = TessMethod::Direct;
    println!("demo mode : {:?}", demo);
    //
    //

    el.run(move |evt, _, ctrl_flow| {
        *ctrl_flow = ControlFlow::Wait;
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(phys_size) => surface.ctx().resize(phys_size),
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => *ctrl_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                    ..
                } => {
                    demo = demo.toggle();
                    println!("demo mode : {:?}", demo);
                    surface.ctx().window().request_redraw();
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        ..
                    },
                    ..
                } => {
                    color = [new_nb(), new_nb(), new_nb(), 1.0];
                    println!("test bg color");
                    surface.ctx().window().request_redraw();
                }
                _ => ()
            },
            Event::RedrawRequested(_) => {
                back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default().set_clear_color(color),
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                let tess = match demo {
                                    TessMethod::Direct => &direct_tris,
                                    TessMethod::Indexed => &indexed_tris,
                                    TessMethod::DirectDeinter => &direct_deint_tris,
                                    TessMethod::IndexedDeinter => &indexed_deint_tris
                                };
                                tess_gate.render(tess);
                            });
                        });
                    }
                            );
                surface.swap_buffers();
            }
            _ => ()
        }
    });
}
