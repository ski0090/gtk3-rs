// This file was generated by gir (af5277e) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Dialog;
use Widget;
use Window;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MessageDialog(Object<ffi::GtkMessageDialog>): Dialog, Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_message_dialog_get_type(),
    }
}

impl MessageDialog {
    //pub fn new<T: IsA<Window>>(parent: Option<&T>, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new() }
    //}

    //pub fn new_with_markup<T: IsA<Window>>(parent: Option<&T>, flags: DialogFlags, type_: MessageType, buttons: ButtonsType, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> MessageDialog {
    //    unsafe { TODO: call ffi::gtk_message_dialog_new_with_markup() }
    //}

    //pub fn format_secondary_markup(&self, message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_markup() }
    //}

    //pub fn format_secondary_text(&self, message_format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_message_dialog_format_secondary_text() }
    //}

    pub fn get_image(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_image(self.to_glib_none().0))
        }
    }

    pub fn get_message_area(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_message_dialog_get_message_area(self.to_glib_none().0))
        }
    }

    pub fn set_image<T: IsA<Widget>>(&self, image: &T) {
        unsafe {
            ffi::gtk_message_dialog_set_image(self.to_glib_none().0, image.to_glib_none().0);
        }
    }

    pub fn set_markup(&self, str: &str) {
        unsafe {
            ffi::gtk_message_dialog_set_markup(self.to_glib_none().0, str.to_glib_none().0);
        }
    }
}
