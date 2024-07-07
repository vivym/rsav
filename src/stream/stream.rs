use crate::{codec::CodecParameters, container::Container, ffi, MediaType, Rational};

pub struct Stream<'a, D> {
    container: &'a Container<D>,
    index: u32,
}

impl<'a, D> Stream<'a, D> {
    pub unsafe fn wrap(container: &'a Container<D>, index: u32) -> Self {
        Stream { container, index }
    }

    pub unsafe fn as_ptr(&self) -> *const ffi::AVStream {
        *(*self.container.as_ptr()).streams.add(self.index as usize)
    }

    pub fn parameters(&self) -> CodecParameters<D> {
        unsafe {
            let ptr = (*self.as_ptr()).codecpar;
            CodecParameters::wrap(ptr, Some(self.container.destructor()))
        }
    }

    #[inline]
    pub fn index(&self) -> u32 {
        unsafe { (*self.as_ptr()).index as _ }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).time_base) }
    }

    pub fn start_time(&self) -> i64 {
        unsafe { (*self.as_ptr()).start_time }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }

    pub fn nb_frames(&self) -> i64 {
        unsafe { (*self.as_ptr()).nb_frames }
    }

    pub fn r_frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).r_frame_rate) }
    }

    pub fn avg_frame_rate(&self) -> Rational {
        unsafe { Rational::from((*self.as_ptr()).avg_frame_rate) }
    }
}

impl<'a, D> PartialEq for Stream<'a, D>{
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ptr() == other.as_ptr() }
    }
}

impl<'a, D> Eq for Stream<'a, D> {}

pub struct StreamIter<'a, D> {
    container: &'a Container<D>,
    current: u32,
}

impl<'a, D> StreamIter<'a, D> {
    pub fn new(container: &'a Container<D>) -> Self {
        StreamIter { container, current: 0 }
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

impl<'a, D> Iterator for StreamIter<'a, D> {
    type Item = Stream<'a, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.container.nb_streams() {
            let stream = unsafe {
                Stream::wrap(self.container, self.current)
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

impl<'a, D> ExactSizeIterator for StreamIter<'a, D> {}
