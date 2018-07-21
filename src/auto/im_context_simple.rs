// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use IMContext;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IMContextSimple(Object<ffi::GtkIMContextSimple, ffi::GtkIMContextSimpleClass>): IMContext;

    match fn {
        get_type => || ffi::gtk_im_context_simple_get_type(),
    }
}

impl IMContextSimple {
    pub fn new() -> IMContextSimple {
        assert_initialized_main_thread!();
        unsafe {
            IMContext::from_glib_full(ffi::gtk_im_context_simple_new()).downcast_unchecked()
        }
    }
}

impl Default for IMContextSimple {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IMContextSimpleExt {
    fn add_compose_file(&self, compose_file: &str);

    fn add_table(&self, data: &[u16], n_seqs: i32);
}

impl<O: IsA<IMContextSimple>> IMContextSimpleExt for O {
    fn add_compose_file(&self, compose_file: &str) {
        unsafe {
            ffi::gtk_im_context_simple_add_compose_file(self.to_glib_none().0, compose_file.to_glib_none().0);
        }
    }

    fn add_table(&self, data: &[u16], n_seqs: i32) {
        let max_seq_len = data.len() as i32;
        unsafe {
            ffi::gtk_im_context_simple_add_table(self.to_glib_none().0, data.to_glib_none().0, max_seq_len, n_seqs);
        }
    }
}
