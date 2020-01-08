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

pub use self::context_tracker::ContextTracker;

mod context_tracker {
    use glutin::{
        Context, ContextCurrentState, ContextError, NotCurrent,
        PossiblyCurrent, WindowedContext
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

        fn map<T2: ContextCurrentState, FH, FW>(self, fh: FH, fw: FW)
        -> Result<ContextWrapper<T2>, (Self, ContextError)>
        where
            FH: FnOnce(Context<T>) -> Result<Context<T2>, (Context<T>, ContextError)>,
            FW: FnOnce(WindowedContext<T>) -> Result<WindowedContext<T2>, (WindowedContext<T>, ContextError)>
        {
            match self {
                ContextWrapper::Headless(ctx) => match fh(ctx) {
                    Ok(ctx) => Ok(ContextWrapper::Headless(ctx)),
                    Err((ctx, err)) => Err((ContextWrapper::Headless(ctx), err))
                }
                ContextWrapper::Windowed(ctx) => match fw(ctx) {
                    Ok(ctx) => Ok(ContextWrapper::Windowed(ctx)),
                    Err((ctx, err)) => Err((ContextWrapper::Windowed(ctx), err))
                }
            }
        }
    }

    pub enum ContextCurrentWrapper {
        PossiblyCurrent(ContextWrapper<PossiblyCurrent>),
        NotCurrent(ContextWrapper<NotCurrent>)
    }

    impl ContextCurrentWrapper {
        fn map_possibly<F>(self, f: F) -> Result<Self, (Self, ContextError)>
        where
            F: FnOnce(ContextWrapper<PossiblyCurrent>) -> Result<
                ContextWrapper<NotCurrent>,
                (ContextWrapper<PossiblyCurrent>, ContextError)
            >
        {
            match self {
                ret @ ContextCurrentWrapper::NotCurrent(_) => Ok(ret),
                ContextCurrentWrapper::PossiblyCurrent(ctx) => match f(ctx) {
                    Ok(ctx) => Ok(ContextCurrentWrapper::NotCurrent(ctx)),
                    Err((ctx, err)) => Err((ContextCurrentWrapper::PossiblyCurrent(ctx), err))
                }
            }
        }

        fn map_not<F>(self, f: F) -> Result<Self, (Self, ContextError)>
        where
            F: FnOnce(ContextWrapper<NotCurrent>) -> Result<
                ContextWrapper<PossiblyCurrent>,
                (ContextWrapper<NotCurrent>, ContextError)
            >
        {
            match self {
                ret @ ContextCurrentWrapper::PossiblyCurrent(_) => Ok(ret),
                ContextCurrentWrapper::NotCurrent(ctx) => match f(ctx) {
                    Ok(ctx) => Ok(ContextCurrentWrapper::PossiblyCurrent(ctx)),
                    Err((ctx, err)) => Err((ContextCurrentWrapper::NotCurrent(ctx), err))
                }
            }
        }
    }

    pub type ContextId = usize;

    #[derive(Default)]
    pub struct ContextTracker {
        current: Option<ContextId>,
        others: Vec<(ContextId, Takeable<ContextCurrentWrapper>)>,
        next_id: ContextId
    }

    impl ContextTracker {
        pub fn insert(&mut self, ctx: ContextCurrentWrapper) -> ContextId {
            let id = self.next_id;
            self.next_id += 1;
            if let ContextCurrentWrapper::PossiblyCurrent(_) = ctx {
                if let Some(old_curr) = self.current {
                    unsafe {
                        self.modify(old_curr, |ctx| {
                            ctx.map_possibly(|ctx| {ctx.map(
                                |ctx| Ok(ctx.treat_as_not_current()),
                                |ctx| Ok(ctx.treat_as_not_current())
                            )})
                        })
                    }.unwrap()
                }
                self.current = Some(id);
            }
            self.others.push((id, Takeable::new(ctx)));
            id
        }

        pub fn remove(&mut self, id: ContextId) -> ContextCurrentWrapper {
            if Some(id) == self.current { self.current.take(); }
            let this_idx = self
                .others
                .binary_search_by(|(sid, _)| sid.cmp(&id))
                .unwrap();
            Takeable::take(&mut self.others.remove(this_idx).1)
        }

        fn modify<F>(&mut self, id: ContextId, f: F) -> Result<(), ContextError>
        where
            F: FnOnce(ContextCurrentWrapper) -> Result<
                ContextCurrentWrapper,
                (ContextCurrentWrapper, ContextError)
            >
        {
            let this_idx = self
                .others
                .binary_search_by(|(sid, _)| sid.cmp(&id))
                .unwrap();
            let this_ctx = Takeable::take(&mut self.others[this_idx].1);
            match f(this_ctx) {
                Err((ctx, err)) => {
                    self.others[this_idx].1 = Takeable::new(ctx);
                    Err(err)
                }
                Ok(ctx) => {
                    self.others[this_idx].1 = Takeable::new(ctx);
                    Ok(())
                }
            }
        }

        pub fn get_current(&mut self, id: ContextId)
        -> Result<&mut ContextWrapper<PossiblyCurrent>, ContextError> {
            let this_idx = self
                .others
                .binary_search_by(|(sid, _)| sid.cmp(&id))
                .unwrap();
            unsafe {
                if Some(id) != self.current {
                    let old_curr = self.current.take();
                    if let Err(err) = self.modify(id, |ctx| {
                        ctx.map_not(|ctx| {ctx.map(
                            |ctx| ctx.make_current(),
                            |ctx| ctx.make_current()
                        )})
                    }) {
                        if let Some(old_curr) = old_curr {
                            if let Err(err2) = self.modify(old_curr, |ctx| {
                                ctx.map_possibly(|ctx| {ctx.map(
                                    |ctx| ctx.make_not_current(),
                                    |ctx| ctx.make_not_current()
                                )})
                            }) {
                                panic!("Couldn't make or not make current: {:?}, {:?}", err, err2);
                            }
                        }
                        if let Err(err2) = self.modify(id, |ctx| {
                            ctx.map_possibly(|ctx| {ctx.map(
                                |ctx| ctx.make_not_current(),
                                |ctx| ctx.make_not_current()
                            )})
                        }) {
                            panic!("Couldn't male or not make current: {:?}, {:?}", err, err2);
                        }
                        return Err(err);
                    }
                    self.current = Some(id);
                    if let Some(old_curr) = old_curr {
                        self.modify(old_curr, |ctx| {ctx.map_possibly(|ctx| {ctx.map(
                            |ctx| Ok(ctx.treat_as_not_current()),
                            |ctx| Ok(ctx.treat_as_not_current())
                        )})}).unwrap();
                    }
                }
                match *self.others[this_idx].1 {
                    ContextCurrentWrapper::PossiblyCurrent(ref mut ctx) => Ok(ctx),
                    ContextCurrentWrapper::NotCurrent(_) => panic!()
                }
            }
        }
    }
}
