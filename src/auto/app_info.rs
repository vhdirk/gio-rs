// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use AppLaunchContext;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct AppInfo(Object<ffi::GAppInfo>);

    match fn {
        get_type => || ffi::g_app_info_get_type(),
    }
}

pub trait AppInfoExt {
    fn add_supports_type(&self, content_type: &str) -> Result<(), Error>;

    fn can_delete(&self) -> bool;

    fn can_remove_supports_type(&self) -> bool;

    fn delete(&self) -> bool;

    fn dup(&self) -> Option<AppInfo>;

    fn equal<T: IsA<AppInfo>>(&self, appinfo2: &T) -> bool;

    fn get_commandline(&self) -> Option<String>;

    fn get_description(&self) -> Option<String>;

    fn get_display_name(&self) -> Option<String>;

    fn get_executable(&self) -> Option<String>;

    //fn get_icon(&self) -> /*Ignored*/Option<Icon>;

    fn get_id(&self) -> Option<String>;

    fn get_name(&self) -> Option<String>;

    #[cfg(feature = "v2_34")]
    fn get_supported_types(&self) -> Vec<String>;

    //fn launch(&self, files: /*Ignored*/&[File], launch_context: Option<&AppLaunchContext>) -> Result<(), Error>;

    fn launch_uris(&self, uris: &[&str], launch_context: Option<&AppLaunchContext>) -> Result<(), Error>;

    fn remove_supports_type(&self, content_type: &str) -> Result<(), Error>;

    fn set_as_default_for_extension(&self, extension: &str) -> Result<(), Error>;

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

    fn equal<T: IsA<AppInfo>>(&self, appinfo2: &T) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_equal(self.to_glib_none().0, appinfo2.to_glib_none().0))
        }
    }

    fn get_commandline(&self) -> Option<String> {
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

    fn get_executable(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_executable(self.to_glib_none().0))
        }
    }

    //fn get_icon(&self) -> /*Ignored*/Option<Icon> {
    //    unsafe { TODO: call ffi::g_app_info_get_icon() }
    //}

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

    #[cfg(feature = "v2_34")]
    fn get_supported_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_app_info_get_supported_types(self.to_glib_none().0))
        }
    }

    //fn launch(&self, files: /*Ignored*/&[File], launch_context: Option<&AppLaunchContext>) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::g_app_info_launch() }
    //}

    fn launch_uris(&self, uris: &[&str], launch_context: Option<&AppLaunchContext>) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_launch_uris(self.to_glib_none().0, uris.to_glib_none().0, launch_context.to_glib_none().0, &mut error);
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

    fn set_as_default_for_extension(&self, extension: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_set_as_default_for_extension(self.to_glib_none().0, extension.to_glib_none().0, &mut error);
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