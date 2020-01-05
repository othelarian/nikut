// Glutin single win example

/*
use glutin::self;

type ContextId = usize;


fn main() {
    let mut el = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new().with_title("First Win");
    let win_ctxt = glutin::ContextBuilder::new()
        .build_windowed(wb, &el).unwrap();
    let win_ctxt = unsafe { win_ctxt.make_current().unwrap() };
    //
    //
    let mut running = true;
    while running {
        el.poll_events(|evt| {
            match evt {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = win_ctxt.window().get_hidpi_factor();
                        win_ctxt.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => ()
                }
                _ => ()
            }
        });
    }
}
*/