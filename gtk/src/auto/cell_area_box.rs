// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::CellArea;
use crate::CellLayout;
use crate::CellRenderer;
use crate::Orientable;
use crate::Orientation;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkCellAreaBox")]
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox, ffi::GtkCellAreaBoxClass>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    #[doc(alias = "gtk_cell_area_box_new")]
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe { CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellAreaBox`] objects.
    ///
    /// This method returns an instance of [`CellAreaBoxBuilder`](crate::builders::CellAreaBoxBuilder) which can be used to create [`CellAreaBox`] objects.
    pub fn builder() -> CellAreaBoxBuilder {
        CellAreaBoxBuilder::default()
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellAreaBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct CellAreaBoxBuilder {
    spacing: Option<i32>,
    focus_cell: Option<CellRenderer>,
    orientation: Option<Orientation>,
}

impl CellAreaBoxBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CellAreaBoxBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellAreaBox`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> CellAreaBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref focus_cell) = self.focus_cell {
            properties.push(("focus-cell", focus_cell));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<CellAreaBox>(&properties)
            .expect("Failed to create an instance of CellAreaBox")
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn focus_cell(mut self, focus_cell: &impl IsA<CellRenderer>) -> Self {
        self.focus_cell = Some(focus_cell.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl CellAreaBox {
    pub const NONE: Option<&'static CellAreaBox> = None;
}

pub trait CellAreaBoxExt: 'static {
    #[doc(alias = "gtk_cell_area_box_get_spacing")]
    #[doc(alias = "get_spacing")]
    fn spacing(&self) -> i32;

    #[doc(alias = "gtk_cell_area_box_pack_end")]
    fn pack_end(&self, renderer: &impl IsA<CellRenderer>, expand: bool, align: bool, fixed: bool);

    #[doc(alias = "gtk_cell_area_box_pack_start")]
    fn pack_start(&self, renderer: &impl IsA<CellRenderer>, expand: bool, align: bool, fixed: bool);

    #[doc(alias = "gtk_cell_area_box_set_spacing")]
    fn set_spacing(&self, spacing: i32);

    #[doc(alias = "spacing")]
    fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellAreaBox>> CellAreaBoxExt for O {
    fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_cell_area_box_get_spacing(self.as_ref().to_glib_none().0) }
    }

    fn pack_end(&self, renderer: &impl IsA<CellRenderer>, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    fn pack_start(
        &self,
        renderer: &impl IsA<CellRenderer>,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(
                self.as_ref().to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P: IsA<CellAreaBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellAreaBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellAreaBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellAreaBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellAreaBox")
    }
}
