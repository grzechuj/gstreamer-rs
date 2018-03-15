// This file was generated by gir (https://github.com/gtk-rs/gir @ d1e0127)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use SDPResult;
use ffi;
use glib::translate::*;
use glib_ffi;
use gst;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SDPMedia(Boxed<ffi::GstSDPMedia>);

    match fn {
        copy => |ptr| ffi::gst_sdp_media_copy(mut_override(ptr)),
        free => |ptr| ffi::gst_sdp_media_free(ptr),
    }
}

impl SDPMedia {
    pub fn add_attribute<'a, P: Into<Option<&'a str>>>(&mut self, key: &str, value: P) -> SDPResult {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            from_glib(ffi::gst_sdp_media_add_attribute(self.to_glib_none_mut().0, key.to_glib_none().0, value.0))
        }
    }

    pub fn add_bandwidth(&mut self, bwtype: &str, bandwidth: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_add_bandwidth(self.to_glib_none_mut().0, bwtype.to_glib_none().0, bandwidth))
        }
    }

    pub fn add_connection(&mut self, nettype: &str, addrtype: &str, address: &str, ttl: u32, addr_number: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_add_connection(self.to_glib_none_mut().0, nettype.to_glib_none().0, addrtype.to_glib_none().0, address.to_glib_none().0, ttl, addr_number))
        }
    }

    pub fn add_format(&mut self, format: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_add_format(self.to_glib_none_mut().0, format.to_glib_none().0))
        }
    }

    pub fn as_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_sdp_media_as_text(self.to_glib_none().0))
        }
    }

    pub fn attributes_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_attributes_len(self.to_glib_none().0)
        }
    }

    pub fn attributes_to_caps(&self, caps: &gst::Caps) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_attributes_to_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    pub fn bandwidths_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_bandwidths_len(self.to_glib_none().0)
        }
    }

    pub fn connections_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_connections_len(self.to_glib_none().0)
        }
    }

    pub fn formats_len(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_formats_len(self.to_glib_none().0)
        }
    }

    //pub fn get_attribute(&self, idx: u32) -> /*Ignored*/Option<SDPAttribute> {
    //    unsafe { TODO: call ffi::gst_sdp_media_get_attribute() }
    //}

    pub fn get_attribute_val(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_attribute_val(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    pub fn get_attribute_val_n(&self, key: &str, nth: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_attribute_val_n(self.to_glib_none().0, key.to_glib_none().0, nth))
        }
    }

    //pub fn get_bandwidth(&self, idx: u32) -> /*Ignored*/Option<SDPBandwidth> {
    //    unsafe { TODO: call ffi::gst_sdp_media_get_bandwidth() }
    //}

    pub fn get_caps_from_media(&self, pt: i32) -> Option<gst::Caps> {
        unsafe {
            from_glib_full(ffi::gst_sdp_media_get_caps_from_media(self.to_glib_none().0, pt))
        }
    }

    //pub fn get_connection(&self, idx: u32) -> /*Ignored*/Option<SDPConnection> {
    //    unsafe { TODO: call ffi::gst_sdp_media_get_connection() }
    //}

    pub fn get_format(&self, idx: u32) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_format(self.to_glib_none().0, idx))
        }
    }

    pub fn get_information(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_information(self.to_glib_none().0))
        }
    }

    //pub fn get_key(&self) -> /*Ignored*/Option<SDPKey> {
    //    unsafe { TODO: call ffi::gst_sdp_media_get_key() }
    //}

    pub fn get_media(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_media(self.to_glib_none().0))
        }
    }

    pub fn get_num_ports(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_get_num_ports(self.to_glib_none().0)
        }
    }

    pub fn get_port(&self) -> u32 {
        unsafe {
            ffi::gst_sdp_media_get_port(self.to_glib_none().0)
        }
    }

    pub fn get_proto(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gst_sdp_media_get_proto(self.to_glib_none().0))
        }
    }

    pub fn init(&mut self) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_init(self.to_glib_none_mut().0))
        }
    }

    //pub fn insert_attribute(&mut self, idx: i32, attr: /*Ignored*/&mut SDPAttribute) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_insert_attribute() }
    //}

    //pub fn insert_bandwidth(&mut self, idx: i32, bw: /*Ignored*/&mut SDPBandwidth) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_insert_bandwidth() }
    //}

    //pub fn insert_connection(&mut self, idx: i32, conn: /*Ignored*/&mut SDPConnection) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_insert_connection() }
    //}

    pub fn insert_format(&mut self, idx: i32, format: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_insert_format(self.to_glib_none_mut().0, idx, format.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v1_8_1", feature = "dox"))]
    //pub fn parse_keymgmt(&self, mikey: /*Ignored*/MIKEYMessage) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_parse_keymgmt() }
    //}

    pub fn remove_attribute(&mut self, idx: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_remove_attribute(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn remove_bandwidth(&mut self, idx: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_remove_bandwidth(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn remove_connection(&mut self, idx: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_remove_connection(self.to_glib_none_mut().0, idx))
        }
    }

    pub fn remove_format(&mut self, idx: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_remove_format(self.to_glib_none_mut().0, idx))
        }
    }

    //pub fn replace_attribute(&mut self, idx: u32, attr: /*Ignored*/&mut SDPAttribute) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_replace_attribute() }
    //}

    //pub fn replace_bandwidth(&mut self, idx: u32, bw: /*Ignored*/&mut SDPBandwidth) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_replace_bandwidth() }
    //}

    //pub fn replace_connection(&mut self, idx: u32, conn: /*Ignored*/&mut SDPConnection) -> SDPResult {
    //    unsafe { TODO: call ffi::gst_sdp_media_replace_connection() }
    //}

    pub fn replace_format(&mut self, idx: u32, format: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_replace_format(self.to_glib_none_mut().0, idx, format.to_glib_none().0))
        }
    }

    pub fn set_information(&mut self, information: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_set_information(self.to_glib_none_mut().0, information.to_glib_none().0))
        }
    }

    pub fn set_key(&mut self, type_: &str, data: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_set_key(self.to_glib_none_mut().0, type_.to_glib_none().0, data.to_glib_none().0))
        }
    }

    pub fn set_media(&mut self, med: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_set_media(self.to_glib_none_mut().0, med.to_glib_none().0))
        }
    }

    pub fn set_port_info(&mut self, port: u32, num_ports: u32) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_set_port_info(self.to_glib_none_mut().0, port, num_ports))
        }
    }

    pub fn set_proto(&mut self, proto: &str) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_set_proto(self.to_glib_none_mut().0, proto.to_glib_none().0))
        }
    }

    pub fn uninit(&mut self) -> SDPResult {
        unsafe {
            from_glib(ffi::gst_sdp_media_uninit(self.to_glib_none_mut().0))
        }
    }

    pub fn new() -> (SDPResult, SDPMedia) {
        assert_initialized_main_thread!();
        unsafe {
            let mut media = ptr::null_mut();
            let ret = from_glib(ffi::gst_sdp_media_new(&mut media));
            (ret, from_glib_full(media))
        }
    }

    pub fn set_media_from_caps(caps: &gst::Caps, media: &mut SDPMedia) -> SDPResult {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_sdp_media_set_media_from_caps(caps.to_glib_none().0, media.to_glib_none_mut().0))
        }
    }
}

impl Default for SDPMedia {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for SDPMedia {}