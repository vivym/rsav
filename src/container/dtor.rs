use crate::ffi;

pub enum Mode {
    Input,
    Output,
}

pub trait Destructor {
    const MODE: Mode;

    fn wrap(ptr: *mut ffi::AVFormatContext) -> Self;
}

pub struct InputDestructor(*mut ffi::AVFormatContext);

impl Destructor for InputDestructor {
    const MODE: Mode = Mode::Input;

    fn wrap(ptr: *mut ffi::AVFormatContext) -> Self {
        InputDestructor(ptr)
    }
}

impl Drop for InputDestructor {
    fn drop(&mut self) {
        unsafe {
            ffi::avformat_close_input(&mut self.0);
        }
    }
}

pub struct OutputDestructor(*mut ffi::AVFormatContext);

impl Destructor for OutputDestructor {
    const MODE: Mode = Mode::Output;

    fn wrap(ptr: *mut ffi::AVFormatContext) -> Self {
        OutputDestructor(ptr)
    }
}

impl Drop for OutputDestructor {
    fn drop(&mut self) {
        unsafe {
            let self_ptr = self.0;
            let pb = (*self_ptr).pb;
            if !pb.is_null() {
                ffi::avio_close(pb);
            }
            ffi::avformat_free_context(self_ptr);
        }
    }
}
