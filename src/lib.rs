use std::ops::{Add, Div, Mul, Sub};

#[macro_use]
extern crate bitflags;

pub use ffmpeg_sys_next as ffi;

pub mod codec;
pub mod container;
pub mod error;
pub mod frame;
pub mod frame2;
pub mod packet;
pub mod pixel;
pub mod stream;

mod io;
pub use io::{create, open};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MediaType {
    Video,
    Audio,
    Data,
    Subtitle,
    Attachment,
    Nb,
    Unknown,
}

impl From<ffi::AVMediaType> for MediaType {
    #[inline(always)]
    fn from(value: ffi::AVMediaType) -> Self {
        match value {
            ffi::AVMediaType::AVMEDIA_TYPE_VIDEO => MediaType::Video,
            ffi::AVMediaType::AVMEDIA_TYPE_AUDIO => MediaType::Audio,
            ffi::AVMediaType::AVMEDIA_TYPE_DATA => MediaType::Data,
            ffi::AVMediaType::AVMEDIA_TYPE_SUBTITLE => MediaType::Subtitle,
            ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT => MediaType::Attachment,
            ffi::AVMediaType::AVMEDIA_TYPE_NB => MediaType::Nb,
            ffi::AVMediaType::AVMEDIA_TYPE_UNKNOWN => MediaType::Unknown,
        }
    }
}

impl From<MediaType> for ffi::AVMediaType {
    #[inline(always)]
    fn from(value: MediaType) -> Self {
        match value {
            MediaType::Video => ffi::AVMediaType::AVMEDIA_TYPE_VIDEO,
            MediaType::Audio => ffi::AVMediaType::AVMEDIA_TYPE_AUDIO,
            MediaType::Data => ffi::AVMediaType::AVMEDIA_TYPE_DATA,
            MediaType::Subtitle => ffi::AVMediaType::AVMEDIA_TYPE_SUBTITLE,
            MediaType::Attachment => ffi::AVMediaType::AVMEDIA_TYPE_ATTACHMENT,
            MediaType::Nb => ffi::AVMediaType::AVMEDIA_TYPE_NB,
            MediaType::Unknown => ffi::AVMediaType::AVMEDIA_TYPE_UNKNOWN,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rational(i32, i32);

impl Rational {
    #[inline(always)]
    pub fn new(num: i32, den: i32) -> Self {
        Rational(num, den)
    }

    #[inline(always)]
    pub fn num(&self) -> i32 {
        self.0
    }

    #[inline(always)]
    pub fn den(&self) -> i32 {
        self.1
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn reduce(&self) -> Rational {
        match self.reduce_with_limit(i32::MAX) {
            Ok(r) => r,
            Err(r) => r,
        }
    }

    #[inline]
    pub fn reduce_with_limit(&self, max: i32) -> Result<Rational, Rational> {
        let num = self.0;
        let den = self.1;
        let mut dst_num = 0;
        let mut dst_den = 0;

        unsafe {
            let exact = ffi::av_reduce(&mut dst_num, &mut dst_den, num as _, den as _, max as _);

            if exact == 1 {
                Ok(Rational(dst_num, dst_den))
            } else {
                Err(Rational(dst_num, dst_den))
            }
        }
    }
}

impl From<ffi::AVRational> for Rational {
    #[inline(always)]
    fn from(value: ffi::AVRational) -> Self {
        Rational(value.num, value.den)
    }
}

impl From<Rational> for ffi::AVRational {
    #[inline(always)]
    fn from(value: Rational) -> Self {
        ffi::AVRational {
            num: value.0,
            den: value.1,
        }
    }
}

impl From<f64> for Rational {
    #[inline(always)]
    fn from(value: f64) -> Self {
        unsafe { Rational::from(ffi::av_d2q(value, libc::c_int::MAX)) }
    }
}

impl From<Rational> for f64 {
    #[inline(always)]
    fn from(value: Rational) -> Self {
        unsafe { ffi::av_q2d(value.into()) }
    }
}

impl From<Rational> for u32 {
    #[inline(always)]
    fn from(value: Rational) -> Self {
        unsafe { ffi::av_q2intfloat(value.into()) }
    }
}

impl From<(i32, i32)> for Rational {
    #[inline(always)]
    fn from(value: (i32, i32)) -> Self {
        Rational(value.0, value.1)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == 0 && other.0 == 0 {
            true
        } else {
            let a = self.reduce();
            let b = other.reduce();

            a.0 == b.0 && a.1 == b.1
        }
    }
}

impl Eq for Rational {}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            match ffi::av_cmp_q((*self).into(), (*other).into()) {
                0 => Some(std::cmp::Ordering::Equal),
                1 => Some(std::cmp::Ordering::Greater),
                -1 => Some(std::cmp::Ordering::Less),
                _ => None,
            }
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        unsafe { Rational::from(ffi::av_add_q(self.into(), other.into())) }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        unsafe { Rational::from(ffi::av_sub_q(self.into(), other.into())) }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        unsafe { Rational::from(ffi::av_mul_q(self.into(), other.into())) }
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        unsafe { Rational::from(ffi::av_div_q(self.into(), other.into())) }
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl std::fmt::Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rational({}/{})", self.0, self.1)
    }
}

#[inline]
pub fn nearer(q: Rational, q1: Rational, q2: Rational) -> std::cmp::Ordering {
    unsafe {
        match ffi::av_nearer_q(q.into(), q1.into(), q2.into()) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            -1 => std::cmp::Ordering::Less,
            _ => unreachable!("av_nearer_q returned an invalid value"),
        }
    }
}
