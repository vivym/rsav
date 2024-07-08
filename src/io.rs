use std::{ffi::CString, path::Path};

use crate::{
    container::{InputContainer, OutputContainer},
    error::Error,
    ffi,
};

pub fn open<P: AsRef<Path> + ?Sized>(path: &P) -> Result<InputContainer, Error> {
    unsafe {
        let mut ps = std::ptr::null_mut();
        let path = path_to_cstr(path).map_err(|_| Error::InvalidPath)?;
        let fmt = std::ptr::null_mut();
        let options = std::ptr::null_mut();

        match ffi::avformat_open_input(&mut ps, path.as_ptr(), fmt, options) {
            0 => match ffi::avformat_find_stream_info(ps, std::ptr::null_mut()) {
                r if r >= 0 => Ok(InputContainer::wrap(ps)),
                e => {
                    ffi::avformat_close_input(&mut ps);
                    Err(Error::from_ffmpeg_error_code(e))
                }
            },
            e => Err(Error::from_ffmpeg_error_code(e)),
        }
    }
}

pub fn create<P: AsRef<Path> + ?Sized>(path: &P) -> Result<OutputContainer, Error> {
    unsafe {
        let mut ps = std::ptr::null_mut();
        let path = path_to_cstr(path).map_err(|_| Error::InvalidPath)?;
        let oformat = std::ptr::null_mut();
        let format_name = std::ptr::null();

        match ffi::avformat_alloc_output_context2(&mut ps, oformat, format_name, path.as_ptr()) {
            0 => match ffi::avio_open(&mut (*ps).pb, path.as_ptr(), ffi::AVIO_FLAG_WRITE) {
                0 => Ok(OutputContainer::wrap(ps)),
                e => Err(Error::from_ffmpeg_error_code(e)),
            },
            e => Err(Error::from_ffmpeg_error_code(e)),
        }
    }
}

fn path_to_cstr<P: AsRef<Path> + ?Sized>(path: &P) -> Result<CString, ()> {
    let path = path.as_ref().to_str().ok_or(())?;
    CString::new(path).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open() {
        let path = "data/sample.mov";
        let container = open(path).unwrap();
        assert_eq!(container.nb_streams(), 2);
    }
}
