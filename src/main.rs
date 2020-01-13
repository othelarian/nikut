use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowId;
use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, Tess, TessBuilder};
use luminance_windowing::{WindowDim, WindowOpt};
use std::collections::HashMap;

mod winger;
use winger::{CtxCurrWrapper, WinManager, WinSurface};

mod in_utils;
use in_utils::{TRIS_FULL, Semantics, TessMethod, new_nb};

const VS: &'static str = include_str!("../ressources/simple-vs.glsl");
const FS: &'static str = include_str!("../ressources/simple-fs.glsl");

struct WinData {
    redraw: bool,
    demo: TessMethod,
    tesses: Option<[Tess; 4]>,
    bgcol: [f32; 4]
}

impl WinData {
    fn new() -> WinData {
        WinData {
            redraw: false,
            demo: TessMethod::Direct,
            tesses: None,
            bgcol: [0.0, 0.0, 0.0, 1.0]
        }
    }
}

fn tesses<C: GraphicsContext>(surface: &mut C) {
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
}

fn main() {
    let el = EventLoop::new();
    let mut win_manager = WinManager::default();
    let mut win_data: HashMap<WindowId, WinData> = HashMap::default();

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
    //let surface = win_manager.get_current(win_id);
    //


    //
    let win_id = win_manager.insert_window(surface).unwrap();
    //win_data.insert(win_id, WinData::new());
    //

    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

    /*
    let direct_tris = TessBuilder::new(&mut surface)
        .add_vertices(TRIS_FULL.tri_verts.clone())
        .set_mode(Mode::Triangle)
        .build()
        .unwrap();
    */


    el.run(move |evt, _, ctrl_flow| {
        //*ctrl_flow = ControlFlow::Wait;
        match evt {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, window_id} => match event {
                WindowEvent::Resized(phys_size) => {
                    let surface = win_manager.get_current(window_id).unwrap();
                    match surface.ctx() {
                        CtxCurrWrapper::PossiblyCurrent(ctx) => ctx.resize(phys_size),
                        CtxCurrWrapper::NotCurrent(_) => panic!("Error with opengl")
                    }
                }
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => win_manager.remove_window(window_id),
                //
                //
                //
                /*
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
                    //surface.ctx().window().request_redraw();
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
                    //surface.ctx().window().request_redraw();
                }
                */
                _ => ()
            }
            Event::MainEventsCleared => {
                //
                //
                //
                //surface.ctx().window().request_redraw();
                //
                //
            }
            Event::RedrawRequested(win_id) => {
                let surface = win_manager.get_current(win_id).unwrap();
                let back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default().set_clear_color([0.0, 0.0, 0.0, 1.0]),
                    |_, _| ()
                );
                surface.swap_buffers();
                //
                //
                /*
                back_buffer = surface.back_buffer().unwrap();
                surface.pipeline_builder().pipeline(
                    &back_buffer,
                    &PipelineState::default().set_clear_color(color),
                    //|_, _| ()
                    /*
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
                    */
                );
                surface.swap_buffers();
                */
            }
            _ => ()
        }
        if win_manager.len() == 0 { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
