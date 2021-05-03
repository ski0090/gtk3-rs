// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Misc(Object<ffi::AtkMisc, ffi::AtkMiscClass>);

    match fn {
        type_ => || ffi::atk_misc_get_type(),
    }
}

impl Misc {
    #[doc(alias = "atk_misc_get_instance")]
    #[doc(alias = "get_instance")]
    pub fn instance() -> Option<Misc> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::atk_misc_get_instance()) }
    }
}

pub const NONE_MISC: Option<&Misc> = None;

pub trait AtkMiscExt: 'static {
    #[doc(alias = "atk_misc_threads_enter")]
    fn threads_enter(&self);

    #[doc(alias = "atk_misc_threads_leave")]
    fn threads_leave(&self);
}

impl<O: IsA<Misc>> AtkMiscExt for O {
    fn threads_enter(&self) {
        unsafe {
            ffi::atk_misc_threads_enter(self.as_ref().to_glib_none().0);
        }
    }

    fn threads_leave(&self) {
        unsafe {
            ffi::atk_misc_threads_leave(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for Misc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Misc")
    }
}
