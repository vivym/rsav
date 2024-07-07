use thiserror::Error;

use crate::ffi;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid path, cannot be converted to C string in UTF-8 encoding")]
    InvalidPath,
    #[error("Write after close")]
    WriteAfterClose,
    // Error codes from ffmpeg
    #[error("Bitstream filter not found")]
    BsfNotFound,
    #[error("Internal bug, should not have happened")]
    Bug,
    #[error("Internal bug, should not have happened (2)")]
    Bug2,
    #[error("Buffer too small")]
    BufferTooSmall,
    #[error("Decoder not found")]
    DecoderNotFound,
    #[error("Demuxer not found")]
    DemuxerNotFound,
    #[error("Encoder not found")]
    EncoderNotFound,
    #[error("End of file")]
    Eof,
    #[error("Immediate exit requested")]
    Exit,
    #[error("Generic error in an external library")]
    External,
    #[error("Filter not found")]
    FilterNotFound,
    #[error("Input changed")]
    InputChanged,
    #[error("Invalid data found when processing input")]
    InvalidData,
    #[error("Muxer not found")]
    MuxerNotFound,
    #[error("Option not found")]
    OptionNotFound,
    #[error("Output changed")]
    OutputChanged,
    #[error("Not yet implemented in FFmpeg, patches welcome")]
    PatchWelcome,
    #[error("Protocol not found")]
    ProtocolNotFound,
    #[error("Stream not found")]
    StreamNotFound,
    #[error("Unknown error occurred")]
    Unknown,
    #[error("Requested feature is flagged experimental")]
    Experimental,
    #[error("Input and output changed")]
    InputOutputChanged,
    #[error("Server returned 400 Bad Request")]
    HttpBadRequest,
    #[error("Server returned 401 Unauthorized (authorization failed)")]
    HttpUnauthorized,
    #[error("Server returned 403 Forbidden (access denied)")]
    HttpForbidden,
    #[error("Server returned 404 Not Found")]
    HttpNotFound,
    #[error("Server returned 4XX Client Error, but not one of 40{{0,1,3,4}}")]
    HttpOther4xx,
    #[error("Server returned 5XX Server Error reply")]
    HttpServerError,
}

impl Error {
    pub fn from_ffmpeg_error_code(code: libc::c_int) -> Self {
        match code {
            ffi::AVERROR_BSF_NOT_FOUND => Error::BsfNotFound,
            ffi::AVERROR_BUG => Error::Bug,
            ffi::AVERROR_BUG2 => Error::Bug2,
            ffi::AVERROR_BUFFER_TOO_SMALL => Error::BufferTooSmall,
            ffi::AVERROR_DECODER_NOT_FOUND => Error::DecoderNotFound,
            ffi::AVERROR_DEMUXER_NOT_FOUND => Error::DemuxerNotFound,
            ffi::AVERROR_ENCODER_NOT_FOUND => Error::EncoderNotFound,
            ffi::AVERROR_EOF => Error::Eof,
            ffi::AVERROR_EXIT => Error::Exit,
            ffi::AVERROR_EXTERNAL => Error::External,
            ffi::AVERROR_FILTER_NOT_FOUND => Error::FilterNotFound,
            ffi::AVERROR_INPUT_CHANGED => Error::InputChanged,
            ffi::AVERROR_INVALIDDATA => Error::InvalidData,
            ffi::AVERROR_MUXER_NOT_FOUND => Error::MuxerNotFound,
            ffi::AVERROR_OPTION_NOT_FOUND => Error::OptionNotFound,
            ffi::AVERROR_OUTPUT_CHANGED => Error::OutputChanged,
            ffi::AVERROR_PATCHWELCOME => Error::PatchWelcome,
            ffi::AVERROR_PROTOCOL_NOT_FOUND => Error::ProtocolNotFound,
            ffi::AVERROR_STREAM_NOT_FOUND => Error::StreamNotFound,
            ffi::AVERROR_UNKNOWN => Error::Unknown,
            ffi::AVERROR_EXPERIMENTAL => Error::Experimental,
            ffi::AVERROR_HTTP_BAD_REQUEST => Error::HttpBadRequest,
            ffi::AVERROR_HTTP_UNAUTHORIZED => Error::HttpUnauthorized,
            ffi::AVERROR_HTTP_FORBIDDEN => Error::HttpForbidden,
            ffi::AVERROR_HTTP_NOT_FOUND => Error::HttpNotFound,
            ffi::AVERROR_HTTP_OTHER_4XX => Error::HttpOther4xx,
            ffi::AVERROR_HTTP_SERVER_ERROR => Error::HttpServerError,
            _ => unreachable!("Unknown error code: {}", code),
        }
    }
}
