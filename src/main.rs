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
use std::rc::Rc;

mod winger;
use winger::{CtxCurrWrapper, WinManager, WinSurface};

mod in_utils;
use in_utils::{TRIS_FULL, Semantics, TessMethod, new_nb};

const VS: &'static str = include_str!("../ressources/simple-vs.glsl");
const FS: &'static str = include_str!("../ressources/simple-fs.glsl");

struct WinData {
    redraw: bool,
    demo: TessMethod,
    pub tesses: [Tess; 4],
    pub bgcol: [f32; 4]
}

impl WinData {
    fn new(surface: &mut WinSurface) -> WinData {
        let direct_tris = TessBuilder::new(surface)
            .add_vertices(TRIS_FULL.tri_verts.clone())
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();
        let indexed_tris = TessBuilder::new(surface)
            .add_vertices(TRIS_FULL.tri_verts)
            .set_indices(TRIS_FULL.tri_inds)
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();
        let direct_deint_tris = TessBuilder::new(surface)
            .add_vertices(TRIS_FULL.tri_deint_pos_verts)
            .add_vertices(TRIS_FULL.tri_deint_col_verts)
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();
        let indexed_deint_tris = TessBuilder::new(surface)
            .add_vertices(TRIS_FULL.tri_deint_pos_verts)
            .add_vertices(TRIS_FULL.tri_deint_col_verts)
            .set_indices(TRIS_FULL.tri_inds)
            .set_mode(Mode::Triangle)
            .build()
            .unwrap();
        WinData {
            redraw: false,
            demo: TessMethod::Direct,
            tesses: [direct_tris, indexed_tris, direct_deint_tris, indexed_deint_tris],
            bgcol: [0.0, 0.0, 0.0, 1.0]
        }
    }

    pub fn toggle(&mut self) { self.redraw = true; }

    pub fn need_redraw(&self) -> bool { self.redraw }

    pub fn redrawed(&mut self) { self.redraw = false; }

    pub fn get_mode(&self) -> TessMethod { self.demo }

    pub fn next_mode(&mut self) {
        self.demo = match self.demo {
            TessMethod::Direct => TessMethod::Indexed,
            TessMethod::Indexed => TessMethod::DirectDeinter,
            TessMethod::DirectDeinter => TessMethod::IndexedDeinter,
            TessMethod::IndexedDeinter => TessMethod::Direct
        }
    }
}

fn main() {
    let el = EventLoop::new();
    let mut win_manager = WinManager::new().unwrap();
    let mut win_datas: HashMap<WindowId, WinData> = HashMap::default();

    //*
    for win_idx in 0..1 {
        let surface = WinSurface::new(
            &el,
            WindowDim::Windowed(800, 400),
            &format!("Test Lumglut multiWin #{}", win_idx+1),
            WindowOpt::default(),
            Some(&mut win_manager)
            //Rc::clone(win_manager.state())
        ).expect(&format!("Glutin surface creation {}", win_idx));
        //windows.insert(win_idx, surface);
        //
        let win_id = win_manager.insert_window(surface).unwrap();
        win_datas.insert(win_id, WinData::new(win_manager.get_current(win_id).unwrap()));
        //
    }
    //*/
    //
    /*
    let surface = WinSurface::new(
        &el,
        WindowDim::Windowed(800, 400),
        "Test Lumglut hi",
        WindowOpt::default()
    ).expect("Glutin surface creation");
    let win_id = win_manager.insert_window(surface).unwrap();
    win_datas.insert(win_id, WinData::new(win_manager.get_current(win_id).unwrap()));
    */
    //

    let program = Program::<Semantics, (), ()>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

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
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Released,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                    ..
                } => {
                    let win_data = win_datas.get_mut(&window_id).unwrap();
                    win_data.next_mode();
                    win_data.toggle();
                    println!("Switch mode for win {:?}", &window_id);
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {state: ElementState::Released, ..},
                    ..
                } => {
                    let win_data = win_datas.get_mut(&window_id).unwrap();
                    win_data.bgcol = [new_nb(), new_nb(), new_nb(), 1.0];
                    win_data.toggle();
                }
                _ => ()
            }
            Event::MainEventsCleared => {
                for (win_id, win_data) in &mut win_datas {
                    if win_data.need_redraw() {
                        match win_manager.get_current(win_id.clone()).unwrap().ctx() {
                            CtxCurrWrapper::PossiblyCurrent(ctx) => ctx.window().request_redraw(),
                            CtxCurrWrapper::NotCurrent(_) => panic!()
                        }
                        win_data.redrawed();
                    }
                }
            }
            Event::RedrawRequested(win_id) => {
                let surface = win_manager.get_current(win_id.clone()).unwrap();
                let back_buffer = surface.back_buffer().unwrap();
                let win_data = win_datas.get(&win_id).unwrap();
                //
                surface.pipeline_builder().pipeline(
                //win_manager.pipeline_builder().pipeline(
                //
                    &back_buffer,
                    &PipelineState::default().set_clear_color(win_data.bgcol),
                    |_, mut shd_gate| {
                        shd_gate.shade(&program, |_, mut rdr_gate| {
                            //*
                            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                                let tess = match win_data.get_mode() {
                                    TessMethod::Direct => &win_data.tesses[0],
                                    TessMethod::Indexed => &win_data.tesses[1],
                                    TessMethod::DirectDeinter => &win_data.tesses[2],
                                    TessMethod::IndexedDeinter => &win_data.tesses[3]
                                };
                                tess_gate.render(tess);
                            });
                            //*/
                        });
                    }
                );
                surface.swap_buffers();
            }
            _ => ()
        }
        if win_manager.len() == 0 { *ctrl_flow = ControlFlow::Exit }
        else { *ctrl_flow = ControlFlow::Wait }
    });
}
