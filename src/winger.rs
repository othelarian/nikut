use gl;
use glutin::{
    Api, ContextBuilder, GlProfile, GlRequest,
    NotCurrent, PossiblyCurrent, WindowedContext
};
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
use takeable_option::Takeable;

pub use glutin::{ContextError, CreationError};
pub use luminance_windowing::{CursorMode, Surface, WindowDim, WindowOpt};

#[derive(Debug)]
pub enum WinError {
    CreationError(CreationError),
    ContextError(ContextError),
    GraphicsStateError(StateQueryError),
    WinInternError(&'static str)
}

impl fmt::Display for WinError {
    fn fmt(&self,f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            WinError::CreationError(ref e) =>
                write!(f, "Win surface creation error: {}", e),
            WinError::ContextError(ref e) =>
                write!(f, "Win OGL context creation error: {}", e),
            WinError::GraphicsStateError(ref e) =>
                write!(f, "OGL graphics state init error: {}", e),
            WinError::WinInternError(e) =>
                write!(f, "Win Intern error: {}", e)
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
    //win_ctx: WindowedContext<PossiblyCurrent>,
    win_ctx: CtxCurrWrapper,
    gfx_state: Rc<RefCell<GraphicsState>>
}

unsafe impl GraphicsContext for WinSurface {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}

impl WinSurface {
    pub fn new<T>(
        el: &EventLoop<T>,
        dim: WindowDim,
        title: &str,
        win_opt: WindowOpt,
        //gfx_state: Rc<RefCell<GraphicsState>>
        //
        manager: Option<&mut WinManager>
        //
    ) -> Result<Self, WinError> {
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
        gl::load_with(|s| win_ctx.get_proc_address(s) as *const c_void);
        win_ctx.window().set_visible(true);
        //
        //
        //let gfx_state = GraphicsState::new().map_err(WinError::GraphicsStateError)?;
        //
        let gfx_state = match manager {
            None => Rc::new(RefCell::new(GraphicsState::new().unwrap())),//.map_err(WinError::GraphicsStateError)?,
            Some(manager) => manager.gfx()
        };
        //
        //
        Ok(WinSurface {
            win_ctx: CtxCurrWrapper::PossiblyCurrent(win_ctx),
            //gfx_state: Rc::new(RefCell::new(gfx_state))
            gfx_state: gfx_state
        })
    }

    pub fn ctx(&mut self) -> &mut CtxCurrWrapper { &mut self.win_ctx }

    pub fn back_buffer(&mut self) -> Result<Framebuffer<Flat, Dim2, (), ()>, WinError> {
        match &self.win_ctx {
            CtxCurrWrapper::PossiblyCurrent(ctx) => {
                let (w, h) = ctx.window().inner_size().into();
                Ok(Framebuffer::back_buffer(self, [w, h]))
            }
            CtxCurrWrapper::NotCurrent(_) =>
                Err(WinError::WinInternError("using back buffer of not current ctx"))
        }
    }

    pub fn swap_buffers(&mut self) {
        if let CtxCurrWrapper::PossiblyCurrent(ctx) = &self.win_ctx {
            ctx.swap_buffers().unwrap();
        }
    }
}

pub enum CtxCurrWrapper {
    PossiblyCurrent(WindowedContext<PossiblyCurrent>),
    NotCurrent(WindowedContext<NotCurrent>)
}

pub struct WinManager {
    current: Option<WindowId>,
    gfx_state: Option<Rc<RefCell<GraphicsState>>>,
    others: HashMap<WindowId, Takeable<WinSurface>>
}

/*
unsafe impl GraphicsContext for WinManager {
    fn state(&self) -> &Rc<RefCell<GraphicsState>> { &self.gfx_state }
}
*/

impl WinManager {
    pub fn new() -> Result<Self, WinError> {
        //let gfx_state = GraphicsState::new().map_err(WinError::GraphicsStateError)?;
        Ok(WinManager {
            current: None,
            gfx_state: None,
            others: HashMap::default()
        })
    }

    pub fn gfx(&mut self) -> Rc<RefCell<GraphicsState>> {
        match &self.gfx_state {
            None => {
                //
                //let gfx = GraphicsState::new().unwrap();
                //GraphicsState::get_from_context()
                //
                let gfx = Rc::new(RefCell::new(GraphicsState::new().unwrap()));
                //
                let ret = Rc::clone(&gfx);
                self.gfx_state = Some(gfx);
                //
                ret
            }
            Some(gfx_state) => Rc::clone(&gfx_state)
        }
    }

    pub fn insert_window(&mut self, surface: WinSurface) -> Result<WindowId, WinError> {
        match &surface.win_ctx {
            CtxCurrWrapper::PossiblyCurrent(ctx) => {
                let id = ctx.window().id();
                if let Some(old_curr) = self.current {
                    if let Some(old_curr_surf) = self.others.get_mut(&old_curr) {
                        let mut old_win = Takeable::take(old_curr_surf);
                        if let CtxCurrWrapper::PossiblyCurrent(ctx) = old_win.win_ctx {
                            let nctx = unsafe { ctx.treat_as_not_current() };
                            old_win.win_ctx = CtxCurrWrapper::NotCurrent(nctx);
                        }
                        *old_curr_surf = Takeable::new(old_win);
                    }
                    self.current = Some(id);
                }
                self.others.insert(id, Takeable::new(surface));
                Ok(id)
            }
            CtxCurrWrapper::NotCurrent(_) =>
                Err(WinError::WinInternError("This window current ctx is not current!"))
        }
    }

    pub fn remove_window(&mut self, id: WindowId) {
        if Some(id) == self.current { self.current.take(); }
        self.others.remove(&id);
    }

    pub fn len(&mut self) -> usize { self.others.len() }

    pub fn get_current(&mut self, id: WindowId) -> Result<&mut WinSurface, ContextError> {
        let res = if Some(id) != self.current {
            let ncurr_ref = self.others.get_mut(&id).unwrap();
            let mut ncurr_surface = Takeable::take(ncurr_ref);
            match ncurr_surface.win_ctx {
                CtxCurrWrapper::PossiblyCurrent(_) => {
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(())
                }
                CtxCurrWrapper::NotCurrent(nctx) => unsafe {
                    match nctx.make_current() {
                        Err((rctx, err)) => {
                            match rctx.make_not_current() {
                                Ok(rctx) => {
                                    ncurr_surface.win_ctx = CtxCurrWrapper::NotCurrent(rctx);
                                    *ncurr_ref = Takeable::new(ncurr_surface);
                                    Err(err)
                                }
                                Err((_, err2)) =>
                                    panic!("Couldn't make and not make current: {:?}, {:?}", err, err2)
                            }
                        }
                        Ok(rctx) => {
                            //
                            //ncurr_surface.
                            /*if let Some(gfx) = &self.gfx_state {
                                //
                                //
                                let mut state = gfx.borrow_mut();
                                //
                                state = GraphicsState::get_from_context().unwrap();
                                //
                            }*/
                            //
                            ncurr_surface.win_ctx = CtxCurrWrapper::PossiblyCurrent(rctx);
                            *ncurr_ref = Takeable::new(ncurr_surface);
                            Ok(())
                        }
                    }
                }
            }
        }
        else {
            let ncurr_ref = self.others.get_mut(&id).unwrap();
            let ncurr_surface = Takeable::take(ncurr_ref);
            match &ncurr_surface.win_ctx {
                CtxCurrWrapper::PossiblyCurrent(_) => {
                    //
                    //
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(())
                }
                CtxCurrWrapper::NotCurrent(_) => panic!()
            }
        };
        match res {
            Ok(()) => {
                if Some(id) != self.current {
                    if let Some(oid) = self.current.take() {
                        let old_ref = self.others.get_mut(&oid).unwrap();
                        let mut old_surface = Takeable::take(old_ref);
                        if let CtxCurrWrapper::PossiblyCurrent(octx) = old_surface.win_ctx {
                            unsafe { old_surface.win_ctx = CtxCurrWrapper::NotCurrent(octx.treat_as_not_current()); }
                        }
                        *old_ref = Takeable::new(old_surface);
                    }
                    self.current = Some(id);
                }
                Ok(self.others.get_mut(&id).unwrap())
            }
            Err(err) => {
                if let Some(oid) = self.current.take() {
                    let old_ref = self.others.get_mut(&oid).unwrap();
                    let mut old_surface = Takeable::take(old_ref);
                    if let CtxCurrWrapper::PossiblyCurrent(octx) = old_surface.win_ctx {
                        unsafe {
                            match octx.make_not_current() {
                                Err((_, err2)) =>
                                    panic!("Couldn't make and not make current: {:?}, {:?}", err, err2),
                                Ok(octx) =>
                                    old_surface.win_ctx = CtxCurrWrapper::NotCurrent(octx)
                            }
                        }
                    }
                    *old_ref = Takeable::new(old_surface);
                }
                Err(err)
            }
        }
    }
}
