use crate::ffi;

pub enum Mode {
    Input,
    Output,
}

pub trait Owner {
    const MODE: Mode;

    fn wrap(ptr: *mut ffi::AVFormatContext) -> Self;
}

pub trait OwnerInput: Owner {
    const MODE: Mode = Mode::Input;
}

pub trait OwnerOutput: Owner {
    const MODE: Mode = Mode::Output;
}

pub struct InputOwner {
    ptr: *mut ffi::AVFormatContext,
}