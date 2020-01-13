if Some(id) != self.current {
    let old_curr = self.current.take();
    match ncurr_surface.win_ctx {
        CtxCurrWrapper::PossiblyCurrent(_) => {
            self.current = Some(id);
            //self.others[&id] = Takeable::new(ncurr_surface);
            *ncurr_ref = Takeable::new(ncurr_surface);
            Ok(self.others.get_mut(&id).unwrap())
        }
        CtxCurrWrapper::NotCurrent(nctx) => unsafe {
            match nctx.make_current() {
                Err((rctx, err)) => {
                    if let Some(old_curr) = old_curr {
                        let old_ref = self.others.get_mut(&old_curr).unwrap();
                        let old_surface = Takeable::take(old_ref);
                        if let CtxCurrWrapper::PossiblyCurrent(octx) = old_surface.win_ctx {
                            match octx.make_not_current() {
                                Err((_, err2)) =>
                                    panic!("Couldn't make or not make current: {:?}, {:?}", err, err2),
                                Ok(octx) =>
                                    old_surface.win_ctx = CtxCurrWrapper::NotCurrent(octx)
                            }
                        }
                        //self.others[&old_curr] = Takeable::new(old_surface);
                        *old_ref = Takeable::new(old_surface);
                    }
                    if let Err((_, err2)) = nctx.make_not_current() {
                        panic!("Couldn't make or not make current: {:?}, {:?}", err, err2);
                    }
                    ncurr_surface.win_ctx = CtxCurrWrapper::NotCurrent(rctx);
                    //self.others[&id] = Takeable::new(ncurr_surface);
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Err(err)
                }
                Ok(rctx) => {
                    self.current = Some(id);
                    if let Some(old_curr) = old_curr {
                        let old_ref = self.others.get_mut(&old_curr).unwrap();
                        let old_surface = Takeable::take(old_ref);
                        if let CtxCurrWrapper::PossiblyCurrent(octx) = old_surface.win_ctx {
                            old_surface.win_ctx = CtxCurrWrapper::NotCurrent(octx.treat_as_not_current());
                        }
                        //self.others[&old_curr] = Takeable::new(old_surface);
                        *old_ref = Takeable::new(old_surface);
                    }
                    ncurr_surface.win_ctx = CtxCurrWrapper::PossiblyCurrent(rctx);
                    //self.others[&id] = Takeable::new(ncurr_surface);
                    *ncurr_ref = Takeable::new(ncurr_surface);
                    Ok(self.others.get_mut(&id).unwrap())
                }
            }
        }
    }
}
else {
    match &ncurr_surface.win_ctx {
        CtxCurrWrapper::PossiblyCurrent(_) => {
            //self.others[&id] = Takeable::new(ncurr_surface);
            *ncurr_ref = Takeable::new(ncurr_surface);
            Ok(self.others.get_mut(&id).unwrap())
        }
        CtxCurrWrapper::NotCurrent(_) => panic!()
    }
}
