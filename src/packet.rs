use std::io::Write;

use crate::{
    container::{InputContainer, OutputContainer},
    error::Error,
    ffi,
};

pub struct Packet(ffi::AVPacket);

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    #[inline]
    pub fn empty() -> Self {
        unsafe {
            let mut pkt: ffi::AVPacket = std::mem::zeroed();
            ffi::av_init_packet(&mut pkt);
            Packet(pkt)
        }
    }

    #[inline]
    pub fn new(size: usize) -> Self {
        unsafe {
            let mut pkt: ffi::AVPacket = std::mem::zeroed();
            ffi::av_init_packet(&mut pkt);
            // TODO: handle error
            ffi::av_new_packet(&mut pkt, size as _);
            Packet(pkt)
        }
    }

    #[inline]
    pub fn from_slice(data: &[u8]) -> Self {
        let mut pkt = Packet::new(data.len());
        pkt.data_mut().unwrap().write_all(data).unwrap();
        pkt
    }

    #[inline]
    pub fn shrink(&mut self, size: usize) {
        unsafe {
            ffi::av_shrink_packet(&mut self.0, size as _);
        }
    }

    #[inline]
    pub fn grow(&mut self, size: usize) {
        unsafe {
            ffi::av_grow_packet(&mut self.0, size as _);
        }
    }

    #[inline]
    pub(crate) unsafe fn as_ptr(&self) -> *const ffi::AVPacket {
        &self.0
    }

    #[inline]
    pub(crate) unsafe fn as_mut_ptr(&mut self) -> *mut ffi::AVPacket {
        &mut self.0
    }
}

impl Packet {
    #[inline]
    pub fn stream_index(&self) -> u32 {
        self.0.stream_index as _
    }

    #[inline]
    pub fn set_stream_index(&mut self, index: u32) {
        self.0.stream_index = index as _;
    }

    #[inline]
    pub fn pts(&self) -> Option<i64> {
        match self.0.pts {
            ffi::AV_NOPTS_VALUE => None,
            pts => Some(pts),
        }
    }

    #[inline]
    pub fn set_pts(&mut self, pts: Option<i64>) {
        self.0.pts = pts.unwrap_or(ffi::AV_NOPTS_VALUE);
    }

    #[inline]
    pub fn dts(&self) -> Option<i64> {
        match self.0.dts {
            ffi::AV_NOPTS_VALUE => None,
            dts => Some(dts),
        }
    }

    #[inline]
    pub fn set_dts(&mut self, dts: Option<i64>) {
        self.0.dts = dts.unwrap_or(ffi::AV_NOPTS_VALUE);
    }

    #[inline]
    pub fn time_base(&self) -> crate::Rational {
        self.0.time_base.into()
    }

    #[inline]
    pub fn set_time_base<R: Into<crate::Rational>>(&mut self, time_base: R) {
        self.0.time_base = time_base.into().into();
    }

    #[inline]
    pub fn duration(&self) -> i64 {
        self.0.duration
    }

    #[inline]
    pub fn set_duration(&mut self, duration: i64) {
        self.0.duration = duration;
    }

    #[inline]
    pub fn pos(&self) -> i64 {
        self.0.pos
    }

    #[inline]
    pub fn set_pos(&mut self, pos: i64) {
        self.0.pos = pos;
    }

    #[inline]
    pub fn rescale_ts<S, D>(&mut self, source: Option<S>, dest: D)
    where
        S: Into<crate::Rational>,
        D: Into<crate::Rational>,
    {
        let source = match source {
            Some(source) => source.into(),
            None => self.time_base(),
        };
        let dest = dest.into();
        unsafe {
            ffi::av_packet_rescale_ts(&mut self.0, source.into(), dest.into());
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.0.size as _
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(self.0.flags)
    }

    #[inline]
    pub fn set_flags(&mut self, flags: Flags) {
        self.0.flags = flags.bits();
    }

    #[inline]
    pub fn is_key(&self) -> bool {
        self.flags().contains(Flags::KEY)
    }

    #[inline]
    pub fn is_corrupted(&self) -> bool {
        self.flags().contains(Flags::CORRUPT)
    }

    #[inline]
    pub fn is_discard(&self) -> bool {
        self.flags().contains(Flags::DISCARD)
    }

    #[inline]
    pub fn is_trusted(&self) -> bool {
        self.flags().contains(Flags::TRUSTED)
    }

    #[inline]
    pub fn is_disposable(&self) -> bool {
        self.flags().contains(Flags::DISPOSABLE)
    }

    #[inline]
    pub fn data(&self) -> Option<&[u8]> {
        if self.0.data.is_null() {
            None
        } else {
            Some(unsafe { std::slice::from_raw_parts(self.0.data, self.0.size as _) })
        }
    }

    #[inline]
    pub fn data_mut(&mut self) -> Option<&mut [u8]> {
        if self.0.data.is_null() {
            None
        } else {
            Some(unsafe { std::slice::from_raw_parts_mut(self.0.data, self.0.size as _) })
        }
    }
}

impl Packet {
    #[inline]
    pub fn read_from(&mut self, container: &mut InputContainer) -> Result<(), Error> {
        unsafe {
            match ffi::av_read_frame(container.as_mut_ptr(), self.as_mut_ptr()) {
                0 => Ok(()),
                e => Err(Error::from_ffmpeg_error_code(e)),
            }
        }
    }

    #[inline]
    pub(crate) unsafe fn is_empty(&self) -> bool {
        self.0.size == 0
    }

    #[inline]
    pub fn write(&self, container: &mut OutputContainer) -> Result<bool, Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match ffi::av_write_frame(container.as_mut_ptr(), self.as_ptr() as _) {
                1 => Ok(true),
                0 => Ok(false),
                e => Err(Error::from_ffmpeg_error_code(e)),
            }
        }
    }

    #[inline]
    pub fn write_interleaved(&self, container: &mut OutputContainer) -> Result<(), Error> {
        unsafe {
            if self.is_empty() {
                return Err(Error::InvalidData);
            }

            match ffi::av_interleaved_write_frame(container.as_mut_ptr(), self.as_ptr() as _) {
                0 => Ok(()),
                e => Err(Error::from_ffmpeg_error_code(e)),
            }
        }
    }
}

impl Clone for Packet {
    #[inline]
    fn clone(&self) -> Self {
        let mut pkt = Packet::empty();
        pkt.clone_from(self);
        pkt
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        unsafe {
            ffi::av_packet_ref(&mut self.0, &source.0);
            ffi::av_packet_make_writable(&mut self.0);
        }
    }
}

impl Drop for Packet {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::av_packet_unref(&mut self.0);
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Flags: i32 {
        const KEY = ffi::AV_PKT_FLAG_KEY;
        const CORRUPT = ffi::AV_PKT_FLAG_CORRUPT;
        const DISCARD = ffi::AV_PKT_FLAG_DISCARD;
        const TRUSTED = ffi::AV_PKT_FLAG_TRUSTED;
        const DISPOSABLE = ffi::AV_PKT_FLAG_DISPOSABLE;
    }
}
