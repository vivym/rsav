use super::CodecContext;

pub struct Decoder(CodecContext);

impl Decoder {
    pub fn new(ctx: CodecContext) -> Self {
        Decoder(ctx)
    }
}
