use super::{Codec, CodecId, CodecParameters, Decoder, CodecFlags};
use crate::{error::Error, ffi, MediaType, Rational};

pub struct Context(*mut ffi::AVCodecContext);

unsafe impl Send for Context {}

impl Context {
    #[inline]
    pub(crate) unsafe fn wrap(ptr: *mut ffi::AVCodecContext) -> Self {
        Self(ptr)
    }

    #[inline]
    pub(crate) unsafe fn as_ptr(&self) -> *const ffi::AVCodecContext {
        self.0
    }

    #[inline]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVCodecContext {
        self.0
    }
}

impl Context {
    pub fn new() -> Self {
        let ptr = unsafe {
            ffi::avcodec_alloc_context3(std::ptr::null())
        };
        Self(ptr)
    }

    pub fn from_codec(codec: Codec) -> Self {
        let ptr = unsafe {
            ffi::avcodec_alloc_context3(codec.as_ptr())
        };
        Self(ptr)
    }

    pub fn from_parameters<D, P: Into<CodecParameters<D>>>(parameters: P) -> Result<Self, Error> {
        let parameters = parameters.into();
        let mut ctx = Self::new();
        unsafe {
            match ffi::avcodec_parameters_to_context(ctx.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from_ffmpeg_error_code(e)),
                _ => Ok(ctx),
            }
        }
    }

    pub fn as_decoder(self) -> Decoder {
        Decoder(self)
    }

    pub fn codec(&self) -> Option<Codec> {
        unsafe {
            let ptr = (*self.as_ptr()).codec;
            if ptr.is_null() {
                None
            } else {
                Some(Codec::wrap(ptr))
            }
        }
    }

    pub fn medium(&self) -> MediaType {
        MediaType::from(unsafe { (*self.as_ptr()).codec_type })
    }

    pub fn flags(&self) -> CodecFlags {
        CodecFlags::from_bits_truncate(unsafe { (*self.as_ptr()).flags as _ })
    }

    pub fn set_flags(&mut self, flags: CodecFlags) {
        unsafe {
            (*self.as_mut_ptr()).flags = flags.bits() as _;
        }
    }

    pub fn codec_id(&self) -> CodecId {
        CodecId::from(unsafe { (*self.as_ptr()).codec_id })
    }

    pub fn set_parameters<D, P: Into<CodecParameters<D>>>(&mut self, parameters: P) -> Result<(), Error> {
        let parameters = parameters.into();

        unsafe {
            match ffi::avcodec_parameters_to_context(self.as_mut_ptr(), parameters.as_ptr()) {
                e if e < 0 => Err(Error::from_ffmpeg_error_code(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { (*self.as_ptr()).time_base.into() }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, time_base: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into().into();
        }
    }

    pub fn frame_rate(&self) -> Rational {
        unsafe { (*self.as_ptr()).framerate.into() }
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, frame_rate: Option<R>) {
        unsafe {
            if let Some(r) = frame_rate {
                (*self.as_mut_ptr()).framerate = r.into().into();
            } else {
                (*self.as_mut_ptr()).framerate.num = 0;
                (*self.as_mut_ptr()).framerate.den = 1;
            }
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::avcodec_free_context(&mut self.0);
        }
    }
}
