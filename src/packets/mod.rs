pub use cdj_status::*;
pub use keep_alive::*;
pub use mixer_status::*;
pub use timing::*;

mod cdj_status;
pub(crate) mod keep_alive;
mod mixer_status;
mod status;
mod timing;

#[derive(Debug, Clone)]
pub enum StatusPacket<'a> {
    CdjStatus(CdjStatus<'a>),
    MixerStatus(MixerStatus<'a>),
}

impl<'a> StatusPacket<'a> {
    pub fn parse(bytes: &'a [u8]) -> Option<Self> {
        CdjStatus::try_parse(bytes)
            .map(StatusPacket::CdjStatus)
            .or_else(|| MixerStatus::try_parse(bytes).map(StatusPacket::MixerStatus))
    }
}
