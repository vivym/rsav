use std::ops::{Deref, DerefMut};

use super::{Codec, CodecParameters, Context};
use crate::{error::Error, ffi, frame::Frame, packet::Packet};

pub struct Decoder(pub(crate) Context);

impl Decoder {
    pub fn new() -> Self {
        Self(Context::new())
    }

    pub fn from_codec(codec: Codec) -> Self {
        Self(Context::from_codec(codec))
    }

    pub fn from_parameters<D, P: Into<CodecParameters<D>>>(parameters: P) -> Result<Self, Error> {
        Context::from_parameters(parameters).map(Self)
    }
}

impl Decoder {
    pub fn open(mut self) -> Result<OpenedDecoder, Error> {
        unsafe {
            let codec = std::ptr::null();
            let options = std::ptr::null_mut();
            match ffi::avcodec_open2(self.0.as_mut_ptr(), codec, options) {
                0 => Ok(OpenedDecoder(self.0)),
                e => Err(Error::from_ffmpeg_error_code(e)),
            }
        }
    }
}

impl Deref for Decoder {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct OpenedDecoder(pub(crate) Context);

impl OpenedDecoder {
    pub fn send_packet(&mut self, packet: Packet) -> Result<(), Error> {
        unsafe {
            match ffi::avcodec_send_packet(self.0.as_mut_ptr(), packet.as_ptr()) {
                e if e < 0 => Err(Error::from_ffmpeg_error_code(e)),
                _ => Ok(()),
            }
        }
    }

    /// Sends a NULL packet to the decoder to signal the end of the stream and enter draining mode.
    pub fn send_eof(&mut self) -> Result<(), Error> {
        unsafe {
            match ffi::avcodec_send_packet(self.0.as_mut_ptr(), std::ptr::null()) {
                e if e < 0 => Err(Error::from_ffmpeg_error_code(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            match ffi::avcodec_receive_frame(self.0.as_mut_ptr(), frame.as_mut_ptr()) {
                e if e < 0 => Err(Error::from_ffmpeg_error_code(e)),
                _ => Ok(()),
            }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            ffi::avcodec_flush_buffers(self.0.as_mut_ptr());
        }
    }
}

impl Drop for OpenedDecoder {
    fn drop(&mut self) {
        unsafe {
            ffi::avcodec_close(self.0.as_mut_ptr());
        }
    }
}
