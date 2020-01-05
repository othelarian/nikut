use luminance::context::GraphicsContext;
use luminance::pipeline::PipelineState;
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput,
    Surface, WindowDim, WindowEvent, WindowOpt};
use rand::Rng;
use std::process::exit;

/*
fn main() {
    //
    // TODO : using thread to have multiple windows
    //
}
*/

fn main() {
    let surface = GlutinSurface::new(
        WindowDim::Windowed(960, 540),
        "Luminance first try",
        WindowOpt::default());
    match surface {
        Ok(surface) => {
            main_loop(surface)
        }
        Err(e) => {
            eprintln!("error with the surface creation: {:?}", e);
            exit(1);
        }
    }
}

fn new_nb() -> f32 {
    (rand::thread_rng().gen_range(0, 100) as f32) / 100.0
}

fn main_loop(mut surface: GlutinSurface) {
    let back_buff = surface.back_buffer().unwrap();
    let mut color = [0.0, 0.5, 1.0, 1.0];
    'app: loop {
        for evt in surface.poll_events() {
            if let Event::WindowEvent {event, ..} = evt {
                match event {
                    WindowEvent::CloseRequested => break 'app,
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {state: ElementState::Released, ..},
                        ..
                    } => { color = [new_nb(), new_nb(), new_nb(), 1.0]; }
                    _ => ()
                }
            }
        }
        surface.pipeline_builder().pipeline(
            &back_buff,
            &PipelineState::default().set_clear_color(color),
            |_, _| ()
        );
        surface.swap_buffers();
    }
}
