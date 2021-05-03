// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    pub struct Cancellable(Object<ffi::GCancellable, ffi::GCancellableClass>);

    match fn {
        type_ => || ffi::g_cancellable_get_type(),
    }
}

impl Cancellable {
    #[doc(alias = "g_cancellable_new")]
    pub fn new() -> Cancellable {
        unsafe { from_glib_full(ffi::g_cancellable_new()) }
    }

    #[doc(alias = "g_cancellable_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<Cancellable> {
        unsafe { from_glib_none(ffi::g_cancellable_get_current()) }
    }
}

impl Default for Cancellable {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for Cancellable {}
unsafe impl Sync for Cancellable {}

pub const NONE_CANCELLABLE: Option<&Cancellable> = None;

pub trait CancellableExt: 'static {
    #[doc(alias = "g_cancellable_cancel")]
    fn cancel(&self);

    //#[doc(alias = "g_cancellable_connect")]
    //fn connect<P: Fn() + Send + Sync + 'static>(&self, callback: P, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong;

    #[doc(alias = "g_cancellable_disconnect")]
    fn disconnect(&self, handler_id: libc::c_ulong);

    #[doc(alias = "g_cancellable_get_fd")]
    #[doc(alias = "get_fd")]
    fn fd(&self) -> i32;

    #[doc(alias = "g_cancellable_is_cancelled")]
    fn is_cancelled(&self) -> bool;

    //#[doc(alias = "g_cancellable_make_pollfd")]
    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool;

    #[doc(alias = "g_cancellable_pop_current")]
    fn pop_current(&self);

    #[doc(alias = "g_cancellable_push_current")]
    fn push_current(&self);

    #[doc(alias = "g_cancellable_release_fd")]
    fn release_fd(&self);

    #[doc(alias = "g_cancellable_set_error_if_cancelled")]
    fn set_error_if_cancelled(&self) -> Result<(), glib::Error>;

    #[doc(alias = "cancelled")]
    fn connect_cancelled<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Cancellable>> CancellableExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::g_cancellable_cancel(self.as_ref().to_glib_none().0);
        }
    }

    //fn connect<P: Fn() + Send + Sync + 'static>(&self, callback: P, data: /*Unimplemented*/Option<Fundamental: Pointer>) -> libc::c_ulong {
    //    unsafe { TODO: call ffi:g_cancellable_connect() }
    //}

    fn disconnect(&self, handler_id: libc::c_ulong) {
        unsafe {
            ffi::g_cancellable_disconnect(self.as_ref().to_glib_none().0, handler_id);
        }
    }

    fn fd(&self) -> i32 {
        unsafe { ffi::g_cancellable_get_fd(self.as_ref().to_glib_none().0) }
    }

    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_cancellable_is_cancelled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool {
    //    unsafe { TODO: call ffi:g_cancellable_make_pollfd() }
    //}

    fn pop_current(&self) {
        unsafe {
            ffi::g_cancellable_pop_current(self.as_ref().to_glib_none().0);
        }
    }

    fn push_current(&self) {
        unsafe {
            ffi::g_cancellable_push_current(self.as_ref().to_glib_none().0);
        }
    }

    fn release_fd(&self) {
        unsafe {
            ffi::g_cancellable_release_fd(self.as_ref().to_glib_none().0);
        }
    }

    fn set_error_if_cancelled(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_cancellable_set_error_if_cancelled(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "cancelled")]
    fn connect_cancelled<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancelled_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GCancellable,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Cancellable>,
        {
            let f: &F = &*(f as *const F);
            f(&Cancellable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Cancellable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Cancellable")
    }
}
