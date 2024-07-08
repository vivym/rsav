use std::rc::Rc;

use ffmpeg_sys_next::{av_dump_format, avformat_write_header};

use super::dtor::{Destructor, InputDestructor, Mode, OutputDestructor};
use crate::{
    error::Error,
    ffi,
    packet::Packet,
    stream::{Stream, StreamIter, StreamIterMut, StreamMut},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Opened,
    HEADERWritten,
    Closed,
}

pub struct Container<D> {
    ptr: *mut ffi::AVFormatContext,
    dtor: Rc<D>,
    state: State,
}

pub type InputContainer = Container<InputDestructor>;

pub type OutputContainer = Container<OutputDestructor>;

impl<D> Container<D> {
    pub(crate) unsafe fn destructor(&self) -> Rc<D> {
        Rc::clone(&self.dtor)
    }

    #[inline]
    pub(crate) unsafe fn as_ptr(&self) -> *const ffi::AVFormatContext {
        self.ptr
    }

    #[inline]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFormatContext {
        self.ptr
    }

    #[inline]
    pub fn nb_streams(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_streams }
    }

    pub fn stream(&self, index: u32) -> Option<Stream<D>> {
        if index < self.nb_streams() {
            Some(unsafe { Stream::wrap(self, index) })
        } else {
            None
        }
    }

    pub fn streams(&self) -> StreamIter<D> {
        StreamIter::new(self)
    }

    pub fn stream_mut(&mut self, index: u32) -> Option<StreamMut<D>> {
        if index < self.nb_streams() {
            Some(unsafe { StreamMut::wrap(self, index) })
        } else {
            None
        }
    }

    pub fn streams_mut(&mut self) -> StreamIterMut<D> {
        StreamIterMut::new(self)
    }

    #[inline]
    pub fn nb_chapters(&self) -> u32 {
        unsafe { (*self.as_ptr()).nb_chapters }
    }

    pub fn url_cstr(&self) -> Option<&std::ffi::CStr> {
        unsafe {
            if (*self.as_ptr()).url.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr((*self.as_ptr()).url))
            }
        }
    }

    pub fn url_cstring(&self) -> Option<std::ffi::CString> {
        self.url_cstr().map(|cstr| cstr.to_owned())
    }

    pub fn url(&self) -> Option<&str> {
        self.url_cstr().and_then(|u| u.to_str().ok())
    }

    #[inline]
    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    #[inline]
    pub fn bit_rate(&self) -> i64 {
        unsafe { (*self.as_ptr()).bit_rate }
    }
}

impl<D: Destructor> Container<D> {
    const MODE: Mode = D::MODE;

    pub(crate) unsafe fn wrap(ptr: *mut ffi::AVFormatContext) -> Self {
        Container {
            ptr,
            dtor: Rc::new(D::wrap(ptr)),
            state: State::Opened,
        }
    }

    pub fn dump(&mut self, index: i32) {
        let is_output = match Self::MODE {
            Mode::Input => 0,
            Mode::Output => 1,
        };
        let url = self
            .url_cstring()
            .unwrap_or_else(|| std::ffi::CString::new("").unwrap());
        unsafe {
            av_dump_format(self.as_mut_ptr(), index, url.as_ptr(), is_output);
        }
    }
}

impl InputContainer {
    pub fn demux(&mut self) -> PacketIter {
        PacketIter::new(self)
    }
}

pub struct PacketIter<'a>(&'a mut InputContainer);

impl<'a> PacketIter<'a> {
    pub fn new(container: &'a mut InputContainer) -> Self {
        PacketIter(container)
    }
}

impl<'a> Iterator for PacketIter<'a> {
    type Item = (Stream<'a, InputDestructor>, Packet);

    fn next(&mut self) -> Option<Self::Item> {
        let mut packet = Packet::empty();

        loop {
            match packet.read_from(self.0) {
                Ok(..) => {
                    let stream = unsafe {
                        Stream::wrap(std::mem::transmute_copy(&self.0), packet.stream_index())
                    };
                    packet.set_time_base(stream.time_base());
                    return Some((stream, packet));
                }
                Err(Error::Eof) => return None,
                Err(e) => {
                    // TODO: handle error
                    eprintln!("Error reading packet: {}", e);
                }
            }
        }
    }
}

impl OutputContainer {
    pub fn add_stream_like<D>(&mut self, src: &Stream<D>) -> StreamMut<OutputDestructor> {
        unsafe {
            let codec = std::ptr::null();
            let ptr = ffi::avformat_new_stream(self.as_mut_ptr(), codec);

            if ptr.is_null() {
                // TODO: handle error
                panic!("Failed to add stream");
            }

            ffi::avcodec_parameters_copy((*ptr).codecpar, (*src.as_ptr()).codecpar);

            let index = self.nb_streams() - 1;
            StreamMut::wrap(self, index)
        }
    }

    pub fn mux(&mut self, mut packet: Packet) -> Result<(), Error> {
        // let mut packet = packet;

        if packet.time_base().is_none() {
            // TODO: handle error
            panic!("Packet time base is None")
        }

        match self.state {
            State::Opened => self.write_header()?,
            State::Closed => return Err(Error::WriteAfterClose),
            _ => {}
        }

        // TODO: handle error
        let ost = self.stream(packet.stream_index()).unwrap();
        let src_time_base = packet.time_base();
        let dst_time_base = ost.time_base();
        if src_time_base != dst_time_base {
            packet.rescale_ts(Some(src_time_base), dst_time_base);
        }

        packet.write_interleaved(self)
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        if self.state == State::Opened {
            let options = std::ptr::null_mut();
            unsafe {
                match avformat_write_header(self.as_mut_ptr(), options) {
                    0 => {
                        println!("Header written");
                        self.state = State::HEADERWritten;
                        Ok(())
                    }
                    e => Err(Error::from_ffmpeg_error_code(e)),
                }
            }
        } else {
            Ok(())
        }
    }

    pub fn write_trailer(&mut self) -> Result<(), Error> {
        match self.state {
            State::Opened => self.write_header()?,
            State::Closed => return Ok(()),
            _ => {}
        }

        unsafe {
            match ffi::av_write_trailer(self.as_mut_ptr()) {
                0 => {
                    println!("Trailer written");
                    self.state = State::Closed;
                    Ok(())
                }
                e => Err(Error::from_ffmpeg_error_code(e)),
            }
        }
    }
}

impl<D> Drop for Container<D> {
    fn drop(&mut self) {
        println!("Dropping container with state: {:?}", self.state);
        if self.state == State::HEADERWritten {
            println!("Writing trailer...");
            unsafe {
                ffi::av_write_trailer(self.as_mut_ptr());
            }
        }
    }
}
