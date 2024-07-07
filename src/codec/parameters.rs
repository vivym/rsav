use std::rc::Rc;

use crate::{ffi, MediaType};
use super::CodecId;

pub struct CodecParameters<D> {
    ptr: *mut ffi::AVCodecParameters,
    dtor: Option<Rc<D>>,
}

unsafe impl<D> Send for CodecParameters<D> {}

impl<D> CodecParameters<D> {
    pub unsafe fn wrap(ptr: *mut ffi::AVCodecParameters, dtor: Option<Rc<D>>) -> Self {
        CodecParameters { ptr, dtor }
    }

    #[inline]
    pub unsafe fn as_ptr(&self) -> *const ffi::AVCodecParameters {
        self.ptr
    }

    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVCodecParameters {
        self.ptr
    }
}

impl<D> CodecParameters<D> {
    pub fn new() -> Self {
        unsafe {
            CodecParameters {
                ptr: ffi::avcodec_parameters_alloc(),
                dtor: None,
            }
        }
    }

    pub fn codec_type(&self) -> MediaType {
        unsafe { (*self.as_ptr()).codec_type }.into()
    }

    pub fn codec_id(&self) -> CodecId {
        unsafe { (*self.as_ptr()).codec_id }.into()
    }
}

impl<D> Default for CodecParameters<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D> Drop for CodecParameters<D> {
    fn drop(&mut self) {
        if self.dtor.is_none() {
            unsafe {
                ffi::avcodec_parameters_free(&mut self.ptr);
            }
        }
    }
}

impl<D> Clone for CodecParameters<D> {
    fn clone(&self) -> Self {
        let mut params = CodecParameters::new();
        params.clone_from(self);
        params
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe {
            ffi::avcodec_parameters_copy(self.as_mut_ptr(), source.as_ptr());
        }
    }
}
