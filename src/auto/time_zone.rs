// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use TimeType;
use ffi;
use ffi as glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct TimeZone(Shared<ffi::GTimeZone>);

    match fn {
        ref => |ptr| ffi::g_time_zone_ref(ptr),
        unref => |ptr| ffi::g_time_zone_unref(ptr),
        get_type => || ffi::g_time_zone_get_type(),
    }
}

impl TimeZone {
    pub fn new<'a, P: Into<Option<&'a str>>>(identifier: P) -> TimeZone {
        let identifier = identifier.into();
        let identifier = identifier.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_time_zone_new(identifier.0))
        }
    }

    pub fn new_local() -> TimeZone {
        unsafe {
            from_glib_full(ffi::g_time_zone_new_local())
        }
    }

    pub fn new_utc() -> TimeZone {
        unsafe {
            from_glib_full(ffi::g_time_zone_new_utc())
        }
    }

    pub fn find_interval(&self, type_: TimeType, time_: i64) -> i32 {
        unsafe {
            ffi::g_time_zone_find_interval(self.to_glib_none().0, type_.to_glib(), time_)
        }
    }

    pub fn get_abbreviation(&self, interval: i32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_time_zone_get_abbreviation(self.to_glib_none().0, interval))
        }
    }

    pub fn get_offset(&self, interval: i32) -> i32 {
        unsafe {
            ffi::g_time_zone_get_offset(self.to_glib_none().0, interval)
        }
    }

    pub fn is_dst(&self, interval: i32) -> bool {
        unsafe {
            from_glib(ffi::g_time_zone_is_dst(self.to_glib_none().0, interval))
        }
    }
}
