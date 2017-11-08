// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use AppInfoCreateFlags;
use AppLaunchContext;
use Error;
use File;
use Icon;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct AppInfo(Object<ffi::GAppInfo, ffi::GAppInfoIface>);

    match fn {
        get_type => || ffi::g_app_info_get_type(),
    }
}

impl AppInfo {
    pub fn create_from_commandline<'a, P: Into<Option<&'a str>>>(commandline: &str, application_name: P, flags: AppInfoCreateFlags) -> Result<AppInfo, Error> {
        let application_name = application_name.into();
        let application_name = application_name.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_app_info_create_from_commandline(commandline.to_glib_none().0, application_name.0, flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_all() -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_all())
        }
    }

    pub fn get_all_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_all_for_type(content_type.to_glib_none().0))
        }
    }

    pub fn get_default_for_type(content_type: &str, must_support_uris: bool) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::g_app_info_get_default_for_type(content_type.to_glib_none().0, must_support_uris.to_glib()))
        }
    }

    pub fn get_default_for_uri_scheme(uri_scheme: &str) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::g_app_info_get_default_for_uri_scheme(uri_scheme.to_glib_none().0))
        }
    }

    pub fn get_fallback_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_fallback_for_type(content_type.to_glib_none().0))
        }
    }

    pub fn get_recommended_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_recommended_for_type(content_type.to_glib_none().0))
        }
    }

    pub fn launch_default_for_uri<'a, P: Into<Option<&'a AppLaunchContext>>>(uri: &str, launch_context: P) -> Result<(), Error> {
        let launch_context = launch_context.into();
        let launch_context = launch_context.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_launch_default_for_uri(uri.to_glib_none().0, launch_context.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(any(feature = "v2_50", feature = "dox"))]
    //pub fn launch_default_for_uri_async<'a, 'b, P: Into<Option<&'a Cancellable>>, Q: Into<Option<&'b /*Unimplemented*/AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(uri: &str, launch_context: &AppLaunchContext, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::g_app_info_launch_default_for_uri_async() }
    //}

    //#[cfg(any(feature = "v2_50", feature = "dox"))]
    //pub fn launch_default_for_uri_finish<P: IsA</*Ignored*/AsyncResult>>(result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_app_info_launch_default_for_uri_finish() }
    //}

    pub fn reset_type_associations(content_type: &str) {
        unsafe {
            ffi::g_app_info_reset_type_associations(content_type.to_glib_none().0);
        }
    }
}

pub trait AppInfoExt {
    fn add_supports_type(&self, content_type: &str) -> Result<(), Error>;

    fn can_delete(&self) -> bool;

    fn can_remove_supports_type(&self) -> bool;

    fn delete(&self) -> bool;

    fn dup(&self) -> Option<AppInfo>;

    fn equal<P: IsA<AppInfo>>(&self, appinfo2: &P) -> bool;

    fn get_commandline(&self) -> Option<std::path::PathBuf>;

    fn get_description(&self) -> Option<String>;

    fn get_display_name(&self) -> Option<String>;

    fn get_executable(&self) -> Option<std::path::PathBuf>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_id(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_supported_types(&self) -> Vec<String>;

    fn launch<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, files: &[File], launch_context: P) -> Result<(), Error>;

    fn launch_uris<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, uris: &[&str], launch_context: P) -> Result<(), Error>;

    fn remove_supports_type(&self, content_type: &str) -> Result<(), Error>;

    fn set_as_default_for_extension<P: AsRef<std::path::Path>>(&self, extension: P) -> Result<(), Error>;

    fn set_as_default_for_type(&self, content_type: &str) -> Result<(), Error>;

    fn set_as_last_used_for_type(&self, content_type: &str) -> Result<(), Error>;

    fn should_show(&self) -> bool;

    fn supports_files(&self) -> bool;

    fn supports_uris(&self) -> bool;
}

impl<O: IsA<AppInfo>> AppInfoExt for O {
    fn add_supports_type(&self, content_type: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_add_supports_type(self.to_glib_none().0, content_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn can_delete(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_can_delete(self.to_glib_none().0))
        }
    }

    fn can_remove_supports_type(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_can_remove_supports_type(self.to_glib_none().0))
        }
    }

    fn delete(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_delete(self.to_glib_none().0))
        }
    }

    fn dup(&self) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::g_app_info_dup(self.to_glib_none().0))
        }
    }

    fn equal<P: IsA<AppInfo>>(&self, appinfo2: &P) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_equal(self.to_glib_none().0, appinfo2.to_glib_none().0))
        }
    }

    fn get_commandline(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_commandline(self.to_glib_none().0))
        }
    }

    fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_description(self.to_glib_none().0))
        }
    }

    fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_display_name(self.to_glib_none().0))
        }
    }

    fn get_executable(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_executable(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_icon(self.to_glib_none().0))
        }
    }

    fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_id(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_name(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_supported_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_app_info_get_supported_types(self.to_glib_none().0))
        }
    }

    fn launch<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, files: &[File], launch_context: P) -> Result<(), Error> {
        let launch_context = launch_context.into();
        let launch_context = launch_context.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_launch(self.to_glib_none().0, files.to_glib_none().0, launch_context.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn launch_uris<'a, P: Into<Option<&'a AppLaunchContext>>>(&self, uris: &[&str], launch_context: P) -> Result<(), Error> {
        let launch_context = launch_context.into();
        let launch_context = launch_context.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_launch_uris(self.to_glib_none().0, uris.to_glib_none().0, launch_context.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_supports_type(&self, content_type: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_remove_supports_type(self.to_glib_none().0, content_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_as_default_for_extension<P: AsRef<std::path::Path>>(&self, extension: P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_set_as_default_for_extension(self.to_glib_none().0, extension.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_as_default_for_type(&self, content_type: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_set_as_default_for_type(self.to_glib_none().0, content_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_as_last_used_for_type(&self, content_type: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_set_as_last_used_for_type(self.to_glib_none().0, content_type.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn should_show(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_should_show(self.to_glib_none().0))
        }
    }

    fn supports_files(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_supports_files(self.to_glib_none().0))
        }
    }

    fn supports_uris(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_supports_uris(self.to_glib_none().0))
        }
    }
}
