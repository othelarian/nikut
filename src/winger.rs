use gl;
use glutin::dpi::PhysicalSize;
use glutin::{
    Api, ContextBuilder, EventsLoop, GlProfile, GlRequest, PossiblyCurrent,
    WindowBuilder, WindowedContext
};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::state::{GraphicsState, StateQueryError};
use luminance::texture::{Dim2, Flat};
use std::cell::RefCell;
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;

pub use glutin::{ContextError, CreationError};
pub use luminance_windowing::{CursorMode, Surface, WindowDim, WindowOpt};

#[derive(Debug)]
pub enum WinError {
    CreationError(CreationError),
    ContextError(ContextError),
    GraphicsStateError(StateQueryError)
}

impl fmt::Display for WinError {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            WinError::CreationError(ref e) =>
                write!(f, "Win surface creation error: {}", e),
            WinError::ContextError(ref e) =>
                write!(f, "Win OGL context creation error: {}", e),
            WinError::GraphicsStateError(ref e) =>
                write!(f, "OGL graphics state init error: {}", e)
        }
    }
}

impl From<CreationError> for WinError {
    fn from(e: CreationError) -> Self {
        WinError::CreationError(e)
    }
}

impl From<ContextError> for WinError {
    fn from(e: ContextError) -> Self {
        WinError::ContextError(e)
    }
}

pub struct WinSurface {
    ctx: WindowedContext<PossiblyCurrent>,
    gfx_state: Rc<RefCell<GraphicsState>>
}

unsafe impl GraphicsContext for WinSurface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl WinSurface {
    pub fn new(el: &EventsLoop, dim: WindowDim, title: &str, win_opt: WindowOpt)
    -> Result<Self, WinError> {
        let win_builder = WindowBuilder::new().with_title(title);
        let win_builder = match dim {
            WindowDim::Windowed(w, h) => win_builder.with_dimensions((w, h).into()),
            WindowDim::Fullscreen => win_builder.with_fullscreen(Some(el.get_primary_monitor())),
            WindowDim::FullscreenRestricted(w, h) => {
                win_builder.with_dimensions((w, h).into())
                .with_fullscreen(Some(el.get_primary_monitor()))
            }
        };
        let win_ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_multisampling(win_opt.num_samples().unwrap_or(0) as u16)
            .with_double_buffer(Some(true))
            .build_windowed(win_builder, &el)?;
        let ctx = unsafe { win_ctx.make_current().map_err(|(_, e)| e)? };
        gl::load_with(|s| ctx.get_proc_address(s) as *const c_void);
        match win_opt.cursor_mode() {
            CursorMode::Visible => ctx.window().hide_cursor(false),
            CursorMode::Invisible | CursorMode::Disabled => ctx.window().hide_cursor(true)
        }
        ctx.window().show();
        let gfx_state = GraphicsState::new().map_err(WinError::GraphicsStateError)?;
        Ok(WinSurface {ctx, gfx_state: Rc::new(RefCell::new(gfx_state))})
    }

    pub fn size(&self) -> [u32; 2] {
        let logical = self.ctx.window().get_inner_size().unwrap();
        let (w, h) = PhysicalSize::from_logical(logical, self.ctx.window().get_hidpi_factor()).into();
        [w, h]
    }

    pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, WinError> {
        Ok(Framebuffer::back_buffer(self, self.size()))
    }

    pub fn swap_buffers(&mut self) {
        self.ctx.swap_buffers().unwrap();
    }
}
