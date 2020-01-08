use gl;
use glutin::{Api, ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};
use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Fullscreen, WindowBuilder};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::state::{GraphicsState, StateQueryError};
use luminance::texture::{Dim2, Flat};
use std::cell::RefCell;
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;

#[path = "in_utils.rs"]
mod in_utils;
use in_utils::{
    TRIS_FULL, TriFull
    //
};

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

pub struct WinSurface<'a> {
    win_ctx: WindowedContext<PossiblyCurrent>,
    gfx_state: Rc<RefCell<GraphicsState>>,
    tris: TriFull<'a>,
    //
    //
}

unsafe impl GraphicsContext for WinSurface<'_> {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl WinSurface<'_> {
    pub fn new<T>(el: &EventLoop<T>, dim: WindowDim, title: &str, win_opt: WindowOpt)
    -> Result<Self, WinError> {
        let win_builder = WindowBuilder::new().with_title(title);
        let win_builder = match dim {
            WindowDim::Windowed(w, h) =>
                win_builder.with_inner_size(LogicalSize::new(w, h)),
            WindowDim::Fullscreen =>
                win_builder.with_fullscreen(
                    Some(Fullscreen::Exclusive(el.primary_monitor().video_modes().next().unwrap()))
                ),
            WindowDim::FullscreenRestricted(w, h) => {
                win_builder.with_inner_size(LogicalSize::new(w, h))
                    .with_fullscreen(
                        Some(Fullscreen::Exclusive(
                            el.primary_monitor().video_modes().next().unwrap())
                        )
                    )
            }
        };
        let win_ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_multisampling(win_opt.num_samples().unwrap_or(0) as u16)
            .with_double_buffer(Some(true))
            .build_windowed(win_builder, &el)
            .unwrap();
        let win_ctx = unsafe { win_ctx.make_current().map_err(|(_, e)| e)? };
        match win_opt.cursor_mode() {
            CursorMode::Visible => win_ctx.window().set_cursor_visible(true),
            CursorMode::Invisible | CursorMode::Disabled => win_ctx.window().set_cursor_visible(false)
        }
        //
        gl::load_with(|s| win_ctx.get_proc_address(s) as *const c_void);
        win_ctx.window().set_visible(true);
        let gfx_state = GraphicsState::new().map_err(WinError::GraphicsStateError)?;
        Ok(WinSurface {
            win_ctx,
            gfx_state: Rc::new(RefCell::new(gfx_state)),
            tris: TRIS_FULL,
            //
            //
        })
    }

    pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, WinError> {
        let (w, h) = self.win_ctx.window().inner_size().into();
        Ok(Framebuffer::back_buffer(self, [w, h]))
    }

    pub fn ctx(&mut self) -> &WindowedContext<PossiblyCurrent> { &self.win_ctx }

    pub fn swap_buffers(&mut self) {
        self.win_ctx.swap_buffers().unwrap();
    }
}

pub use self::context_tracker::ContextTracker;

mod context_tracker {
    use glutin::{
        Context, ContextCurrentState, NotCurrent, PossiblyCurrent,
        WindowedContext
    };
    use takeable_option::Takeable;

    pub enum ContextWrapper<T: ContextCurrentState> {
        Headless(Context<T>),
        Windowed(WindowedContext<T>)
    }

    impl<T: ContextCurrentState> ContextWrapper<T> {
        pub fn headless(&mut self) -> &mut Context<T> {
            match self {
                ContextWrapper::Headless(ref mut ctx) => ctx,
                _ => panic!()
            }
        }

        pub fn windowed(&mut self) -> &mut WindowedContext<T> {
            match self {
                ContextWrapper::Windowed(ref mut ctx) => ctx,
                _ => panic!()
            }
        }
    }

    pub enum ContextCurrentWrapper {
        PossiblyCurrent(ContextWrapper<PossiblyCurrent>),
        NotCurrent(ContextWrapper<NotCurrent>)
    }

    pub type ContextId = usize;

    #[derive(Default)]
    pub struct ContextTracker {
        current: Option<ContextId>,
        others: Vec<(ContextId, Takeable<ContextCurrentWrapper>)>,
        next_id: ContextId
    }
}
