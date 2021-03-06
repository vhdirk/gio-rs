// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::translate::*;
use glib::GString;
use std;
use std::mem;
use std::ptr;
use Error;
use InputStream;
use ResourceLookupFlags;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Resource(Shared<gio_sys::GResource>);

    match fn {
        ref => |ptr| gio_sys::g_resource_ref(ptr),
        unref => |ptr| gio_sys::g_resource_unref(ptr),
        get_type => || gio_sys::g_resource_get_type(),
    }
}

impl Resource {
    pub fn enumerate_children(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<Vec<GString>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_resource_enumerate_children(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn get_info(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<(usize, u32), Error> {
        unsafe {
            let mut size = mem::MaybeUninit::uninit();
            let mut flags = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_resource_get_info(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.to_glib(),
                size.as_mut_ptr(),
                flags.as_mut_ptr(),
                &mut error,
            );
            let size = size.assume_init();
            let flags = flags.assume_init();
            if error.is_null() {
                Ok((size, flags))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn lookup_data(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<glib::Bytes, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_resource_lookup_data(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn open_stream(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<InputStream, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_resource_open_stream(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(filename: P) -> Result<Resource, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_resource_load(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
