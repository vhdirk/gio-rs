// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

#[cfg(feature = "v2_38")]
use Icon;
use MenuModel;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MenuItem(Object<ffi::GMenuItem>);

    match fn {
        get_type => || ffi::g_menu_item_get_type(),
    }
}

impl MenuItem {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(label: P, detailed_action: Q) -> MenuItem {
        let label = label.into();
        let label = label.to_glib_none();
        let detailed_action = detailed_action.into();
        let detailed_action = detailed_action.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_menu_item_new(label.0, detailed_action.0))
        }
    }

    #[cfg(feature = "v2_34")]
    pub fn new_from_model<P: IsA<MenuModel>>(model: &P, item_index: i32) -> MenuItem {
        unsafe {
            from_glib_full(ffi::g_menu_item_new_from_model(model.to_glib_none().0, item_index))
        }
    }

    pub fn new_section<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(label: P, section: &Q) -> MenuItem {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_menu_item_new_section(label.0, section.to_glib_none().0))
        }
    }

    pub fn new_submenu<'a, P: Into<Option<&'a str>>, Q: IsA<MenuModel>>(label: P, submenu: &Q) -> MenuItem {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_menu_item_new_submenu(label.0, submenu.to_glib_none().0))
        }
    }
}

pub trait MenuItemExt {
    //#[cfg(feature = "v2_34")]
    //fn get_attribute(&self, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    #[cfg(feature = "v2_34")]
    fn get_attribute_value<'a, P: Into<Option<&'a glib::VariantTy>>>(&self, attribute: &str, expected_type: P) -> Option<glib::Variant>;

    #[cfg(feature = "v2_34")]
    fn get_link(&self, link: &str) -> Option<MenuModel>;

    //fn set_action_and_target<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, action: P, format_string: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_action_and_target_value<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b glib::Variant>>>(&self, action: P, target_value: Q);

    //fn set_attribute<'a, P: Into<Option<&'a str>>>(&self, attribute: &str, format_string: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn set_attribute_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, attribute: &str, value: P);

    fn set_detailed_action(&self, detailed_action: &str);

    #[cfg(feature = "v2_38")]
    fn set_icon<P: IsA<Icon>>(&self, icon: &P);

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P);

    fn set_link<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, link: &str, model: Q);

    fn set_section<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, section: Q);

    fn set_submenu<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, submenu: Q);
}

impl<O: IsA<MenuItem>> MenuItemExt for O {
    //#[cfg(feature = "v2_34")]
    //fn get_attribute(&self, attribute: &str, format_string: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_menu_item_get_attribute() }
    //}

    #[cfg(feature = "v2_34")]
    fn get_attribute_value<'a, P: Into<Option<&'a glib::VariantTy>>>(&self, attribute: &str, expected_type: P) -> Option<glib::Variant> {
        let expected_type = expected_type.into();
        let expected_type = expected_type.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_menu_item_get_attribute_value(self.to_glib_none().0, attribute.to_glib_none().0, expected_type.0))
        }
    }

    #[cfg(feature = "v2_34")]
    fn get_link(&self, link: &str) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_item_get_link(self.to_glib_none().0, link.to_glib_none().0))
        }
    }

    //fn set_action_and_target<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(&self, action: P, format_string: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_menu_item_set_action_and_target() }
    //}

    fn set_action_and_target_value<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b glib::Variant>>>(&self, action: P, target_value: Q) {
        let action = action.into();
        let action = action.to_glib_none();
        let target_value = target_value.into();
        let target_value = target_value.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_action_and_target_value(self.to_glib_none().0, action.0, target_value.0);
        }
    }

    //fn set_attribute<'a, P: Into<Option<&'a str>>>(&self, attribute: &str, format_string: P, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_menu_item_set_attribute() }
    //}

    fn set_attribute_value<'a, P: Into<Option<&'a glib::Variant>>>(&self, attribute: &str, value: P) {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_attribute_value(self.to_glib_none().0, attribute.to_glib_none().0, value.0);
        }
    }

    fn set_detailed_action(&self, detailed_action: &str) {
        unsafe {
            ffi::g_menu_item_set_detailed_action(self.to_glib_none().0, detailed_action.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_38")]
    fn set_icon<P: IsA<Icon>>(&self, icon: &P) {
        unsafe {
            ffi::g_menu_item_set_icon(self.to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_label<'a, P: Into<Option<&'a str>>>(&self, label: P) {
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_label(self.to_glib_none().0, label.0);
        }
    }

    fn set_link<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, link: &str, model: Q) {
        let model = model.into();
        let model = model.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_link(self.to_glib_none().0, link.to_glib_none().0, model.0);
        }
    }

    fn set_section<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, section: Q) {
        let section = section.into();
        let section = section.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_section(self.to_glib_none().0, section.0);
        }
    }

    fn set_submenu<'a, P: IsA<MenuModel> + 'a, Q: Into<Option<&'a P>>>(&self, submenu: Q) {
        let submenu = submenu.into();
        let submenu = submenu.to_glib_none();
        unsafe {
            ffi::g_menu_item_set_submenu(self.to_glib_none().0, submenu.0);
        }
    }
}
