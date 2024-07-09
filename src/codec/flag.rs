use crate::ffi;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct CodecFlags: u32 {
        const UNALIGNED       = ffi::AV_CODEC_FLAG_UNALIGNED;
        const QSCALE          = ffi::AV_CODEC_FLAG_QSCALE;
        const _4MV            = ffi::AV_CODEC_FLAG_4MV;
        const OUTPUT_CORRUPT  = ffi::AV_CODEC_FLAG_OUTPUT_CORRUPT;
        const QPEL            = ffi::AV_CODEC_FLAG_QPEL;
        const PASS1           = ffi::AV_CODEC_FLAG_PASS1;
        const PASS2           = ffi::AV_CODEC_FLAG_PASS2;
        const GRAY            = ffi::AV_CODEC_FLAG_GRAY;
        const PSNR            = ffi::AV_CODEC_FLAG_PSNR;
    }
}
