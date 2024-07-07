use std::ops::Deref;

use crate::{codec::CodecParameters, container::Container, ffi, MediaType, Rational};
use super::Stream;

pub struct StreamMut<'a, D> {
    container: &'a mut Container<D>,
    index: u32,
}

impl<'a, D> StreamMut<'a, D> {
    pub unsafe fn wrap(container: &'a mut Container<D>, index: u32) -> Self {
        StreamMut { container, index }
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVStream {
        *(*self.container.as_mut_ptr()).streams.add(self.index as usize)
    }

    pub fn set_parameters<P: Into<CodecParameters<D>>>(&mut self, parameters: P) {
        let parameters = parameters.into();

        unsafe {
            ffi::avcodec_parameters_copy(
                (*self.as_mut_ptr()).codecpar, parameters.as_ptr()
            );
        }
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, time_base: R) {
        unsafe {
            (*self.as_mut_ptr()).time_base = time_base.into().into();
        }
    }

    pub fn set_r_frame_rate<R: Into<Rational>>(&mut self, r_frame_rate: R) {
        unsafe {
            (*self.as_mut_ptr()).r_frame_rate = r_frame_rate.into().into();
        }
    }

    pub fn set_avg_frame_rate<R: Into<Rational>>(&mut self, avg_frame_rate: R) {
        unsafe {
            (*self.as_mut_ptr()).avg_frame_rate = avg_frame_rate.into().into();
        }
    }
}

impl<'a, D> Deref for StreamMut<'a, D> {
    type Target = Stream<'a, D>;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*(self as *const Self as *const Self::Target)) }
    }
}

pub struct StreamIterMut<'a, D> {
    container: &'a mut Container<D>,
    current: u32,
}

impl<'a, D> StreamIterMut<'a, D> {
    pub fn new(container: &'a mut Container<D>) -> Self {
        StreamIterMut { container, current: 0 }
    }

    pub fn best(self, kind: MediaType) -> Option<Stream<'a, D>> {
        unsafe {
            let wanted_stream_nb = -1;
            let related_stream = -1;
            let decoder = std::ptr::null_mut();

            let index = ffi::av_find_best_stream(
                self.container.as_ptr() as _,
                kind.into(),
                wanted_stream_nb,
                related_stream,
                decoder,
                0,
            );

            if index >= 0 {
                Some(Stream::wrap(self.container, index as _))
            } else {
                None
            }
        }
    }

    pub fn video(self) -> Option<Stream<'a, D>> {
        self.best(MediaType::Video)
    }

    pub fn audio(self) -> Option<Stream<'a, D>> {
        self.best(MediaType::Audio)
    }

    pub fn subtitle(self) -> Option<Stream<'a, D>> {
        self.best(MediaType::Subtitle)
    }
}

impl<'a, D> Iterator for StreamIterMut<'a, D> {
    type Item = StreamMut<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.container.nb_streams() {
            let stream = unsafe {
                StreamMut::wrap(
                    std::mem::transmute_copy(&self.container),  // TODO: ???
                    self.current,
                )
            };
            self.current += 1;
            Some(stream)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.container.nb_streams() - self.current) as usize;
        (remaining, Some(remaining))
    }
}

impl<'a, D> ExactSizeIterator for StreamIterMut<'a, D> {}
