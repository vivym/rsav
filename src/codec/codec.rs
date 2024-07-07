use crate::{ffi, MediaType};

use super::CodecId;

pub struct Codec(*const ffi::AVCodec);

unsafe impl Send for Codec {}
unsafe impl Sync for Codec {}

impl Codec {
    pub unsafe fn wrap(ptr: *const ffi::AVCodec) -> Self {
        Codec(ptr)
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const ffi::AVCodec {
        self.0
    }
}

impl Codec {
    pub fn find_decoder_by_id(id: CodecId) -> Option<Codec> {
        unsafe {
            let ptr = ffi::avcodec_find_decoder(id.into());
            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    pub fn find_decoder_by_name(name: &str) -> Option<Codec> {
        unsafe {
            let name = std::ffi::CString::new(name);
            if name.is_err() {
                return None;
            }
            let name = name.unwrap();
            let ptr = ffi::avcodec_find_decoder_by_name(name.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    pub fn find_encoder_by_id(id: CodecId) -> Option<Codec> {
        unsafe {
            let ptr = ffi::avcodec_find_encoder(id.into());
            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    pub fn find_encoder_by_name(name: &str) -> Option<Codec> {
        unsafe {
            let name = std::ffi::CString::new(name);
            if name.is_err() {
                return None;
            }
            let name = name.unwrap();
            let ptr = ffi::avcodec_find_encoder_by_name(name.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }
}

impl Codec {
    pub fn is_encoder(&self) -> bool {
        unsafe { ffi::av_codec_is_encoder(self.as_ptr()) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { ffi::av_codec_is_decoder(self.as_ptr()) != 0 }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let ptr = (*self.as_ptr()).name;
            let cstr = std::ffi::CStr::from_ptr(ptr);
            std::str::from_utf8_unchecked(cstr.to_bytes())
        }
    }

    pub fn long_name(&self) -> &str {
        unsafe {
            let ptr = (*self.as_ptr()).long_name;
            if ptr.is_null() {
                ""
            } else {
                let cstr = std::ffi::CStr::from_ptr(ptr);
                std::str::from_utf8_unchecked(cstr.to_bytes())
            }
        }
    }

    pub fn medium(&self) -> MediaType {
        unsafe { (*self.as_ptr()).type_ }.into()
    }

    pub fn id(&self) -> CodecId {
        unsafe { (*self.as_ptr()).id }.into()
    }

    pub fn is_video(&self) -> bool {
        self.medium() == MediaType::Video
    }

    pub fn is_audio(&self) -> bool {
        self.medium() == MediaType::Audio
    }
}
