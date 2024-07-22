use crate::{ffi, pixel::PixelFormat};

pub struct Frame<S: FrameState> {
    ptr: *mut ffi::AVFrame,
    state: S,
}

impl Frame<Empty> {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::av_frame_alloc() };
        Self {
            ptr,
            state: unsafe { Empty::wrap(ptr) },
        }
    }

    pub fn alloc_video(self, format: PixelFormat, width: i32, height: i32) -> Frame<Owned<Video>> {
        Frame {
            ptr: self.ptr,
            state: unsafe {
                let mut state: Owned<Video> = Owned::wrap(self.state);
                state.alloc(format, width, height);
                state
            },
        }
    }
}

/*
av_frame_alloc -> Empty                             drop: av_frame_free
Empty -> av_frame_get_buffer -> Owned               Owned(Empty)
Owned -> av_frame_ref -> Borrowed                   Borrowed('a &Owned)
Borrowed -> av_frame_make_writable -> Owned
*/

pub trait FrameState {}

pub struct Empty(*mut ffi::AVFrame);

impl FrameState for Empty {}

impl Empty {
    unsafe fn wrap(ptr: *mut ffi::AVFrame) -> Self {
        Self(ptr)
    }
}

impl Drop for Empty {
    fn drop(&mut self) {
        unsafe { ffi::av_frame_free(&mut self.0) }
    }
}

pub trait MediaType {}

pub struct Video;

impl MediaType for Video {}

pub struct Audio;

impl MediaType for Audio {}

pub struct Owned<M: MediaType> {
    ptr: Empty,
    _marker: std::marker::PhantomData<M>,
}

impl<M: MediaType> FrameState for Owned<M> {}

impl<M: MediaType> Owned<M> {
    unsafe fn wrap(ptr: Empty) -> Self {
        Self {
            ptr,
            _marker: std::marker::PhantomData,
        }
    }
}

impl Owned<Video> {
    unsafe fn alloc(&mut self, format: PixelFormat, width: i32, height: i32) {
        self.set_format(format);
        self.set_width(width);
        self.set_height(height);

        // TODO: error handling
        ffi::av_frame_get_buffer(self.ptr.0, 0);
    }

    #[inline(always)]
    fn set_format(&mut self, format: PixelFormat) {
        unsafe {
            (*self.ptr.0).format = std::mem::transmute::<ffi::AVPixelFormat, i32>(format.into());
        }
    }

    #[inline(always)]
    fn set_width(&mut self, width: i32) {
        unsafe {
            (*self.ptr.0).width = width;
        }
    }

    #[inline(always)]
    fn set_height(&mut self, height: i32) {
        unsafe {
            (*self.ptr.0).height = height;
        }
    }
}
