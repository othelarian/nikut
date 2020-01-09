use gl;
use glutin::{Api, ContextBuilder, GlProfile, GlRequest, PossiblyCurrent, WindowedContext};
use glutin::dpi::LogicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::{Fullscreen, WindowBuilder, WindowId};
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::state::{GraphicsState, StateQueryError};
use luminance::texture::{Dim2, Flat};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_void;
use std::rc::Rc;

#[path = "in_utils.rs"]
mod in_utils;
use in_utils::TessMethod;

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
    win_ctx: WindowedContext<PossiblyCurrent>,
    gfx_state: Rc<RefCell<GraphicsState>>,
    demo: TessMethod,
    bgcolor: [f32; 4]
}

unsafe impl GraphicsContext for WinSurface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl WinSurface {
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
            demo: TessMethod::Direct,
            bgcolor: [0.0, 0.0, 0.0, 1.0]
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

#[derive(Default)]
pub struct WinManager {
    current: Option<WindowId>,
    others: HashMap<WindowId, WinSurface>
}

impl WinManager {
    pub fn insert_window(&mut self, surface: WinSurface) {
        if surface.ctx().is_current() {
            if let Some(old_curr) = self.current { unsafe {
                if let Some(old_curr_surface) = self.others.get_mut(&old_curr) {
                    old_curr_surface.ctx().treat_as_not_current();
                }
            }}
            self.current = Some(surface.ctx().window().id());
        }
        self.others.insert(surface.ctx().window().id(), surface);
    }

    pub fn remove_window(&mut self, id: WindowId) {
        if Some(id) == self.current { self.current.take(); }
        self.others.remove(&id);
    }

    pub fn get_current(&mut self, id: WindowId) -> Result<&mut WinSurface, ContextError> {
        let surface = self.others.get_mut(&id).unwrap();
        if Some(id) != self.current {
            let old_curr = self.current.take();
            unsafe {
                if let Err((_, err)) = surface.ctx().make_current() {
                    if let Some(old_curr) = old_curr {
                        let old_win = self.others.get_mut(&old_curr).unwrap();
                        if let Err((_, err2)) = old_win.ctx().make_not_current() {
                            panic!("Couldn't make or not make current: {:?}, {:?}", err, err2);
                        }
                    }
                    if let Err((_, err2)) = surface.ctx().make_not_current() {
                        panic!("Couldn't make or not make current: {:?}, {:?}", err, err2);
                    }
                    Err(err)
                }
                else {
                    self.current = Some(id);
                    if let Some(old_curr) = old_curr {
                        self.others.get_mut(&old_curr)
                            .unwrap()
                            .ctx()
                            .treat_as_not_current();
                    }
                    Ok(surface)
                }
            }
        }
        else {
            if surface.ctx().is_current() { Ok(surface) }
            else { panic!() }
        }
    }
}
