use luminance::{StateQueryError};

pub use glutin::{
    ContextError, CreationError, Event, PossiblyCurrent, WindowContext
};
pub use luminance_windowing::{CursorMode, Surface, WindowDim, WindowOpt};

#[derive(Debug)]
pub enum Error {
    CreationError(CreationError),
    ContextError(ContextError),
    GraphicsStateError(StateQueryError)
}

impl From<CreationError> for Error {
    fn From(e: CreationError) -> Self {
        Error::CreationError(e)
    }
}

impl From<ContextError> for Error {
    fn from(e: ContextError) -> Self {
        Error::ContextError(e)
    }
}

pub struct WinSurface {
    ctx: WindowContext<PossiblyCurrent>,
    //
}

impl Surface for WinSurface {
    type Error = Error;
    type Event = Event;

    fn new(dim: WindowDim, title: &str, win_opt: WindowOpt) -> Result<Self, Self::Error> {
        //
        //
    }

    //
    //
    //
    //

    fn poll_events<'a>(&'a mut self) -> Box<dyn Iterator<Item = Self::Event> + 'a> {
        //
        //
    }

    fn swap_buffers(&mut self) {
        self.ctx.swap_buffers().unwrap();
    }
}
