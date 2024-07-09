use crate::{ffi, pixel::PixelFormat};

#[derive(PartialEq, Eq)]
pub struct Frame(*mut ffi::AVFrame);

unsafe impl Send for Frame {}
unsafe impl Sync for Frame {}

impl Frame {
    #[inline(always)]
    pub(crate) unsafe fn as_ptr(&self) -> *const ffi::AVFrame {
        self.0
    }

    #[inline(always)]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVFrame {
        self.0
    }
}

impl Frame {
    #[inline(always)]
    pub fn empty() -> Self {
        let ptr = unsafe { ffi::av_frame_alloc() };
        Self(ptr)
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        unsafe { (*self.as_ptr()).data[0].is_null() }
    }

    #[inline(always)]
    pub fn is_corrupt(&self) -> bool {
        self.flags().contains(Flags::CORRUPT)
    }

    #[inline(always)]
    pub fn is_key(&self) -> bool {
        unsafe { (*self.as_ptr()).key_frame == 1 }
    }

    #[inline(always)]
    pub fn pts(&self) -> Option<i64> {
        match unsafe { (*self.as_ptr()).pts } {
            ffi::AV_NOPTS_VALUE => None,
            pts => Some(pts),
        }
    }

    #[inline(always)]
    pub fn set_pts(&mut self, pts: Option<i64>) {
        unsafe {
            (*self.as_mut_ptr()).pts = pts.unwrap_or(ffi::AV_NOPTS_VALUE);
        }
    }

    #[inline(always)]
    pub fn timestamp(&self) -> Option<i64> {
        match unsafe { (*self.as_ptr()).best_effort_timestamp } {
            ffi::AV_NOPTS_VALUE => None,
            t => Some(t),
        }
    }

    #[inline(always)]
    pub fn quality(&self) -> usize {
        unsafe { (*self.as_ptr()).quality as _ }
    }

    #[inline(always)]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(unsafe { (*self.as_ptr()).flags })
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            ffi::av_frame_free(&mut self.0);
        }
    }
}

#[derive(PartialEq, Eq)]
pub struct VideoFrame(Frame);

impl VideoFrame {
    #[inline(always)]
    pub fn empty() -> Self {
        Self(Frame::empty())
    }

    pub unsafe fn new(pix_fmt: PixelFormat, width: u32, height: u32) {
        let mut frame = Frame::empty();
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Flags: i32 {
        const CORRUPT = ffi::AV_FRAME_FLAG_CORRUPT;
    }
}
