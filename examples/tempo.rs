use pro_dj_link::*;

pub fn main() -> ProDjLinkResult<()> {
    let mut service = TrackBpmService::new()?;

    loop {
        if let Some(package) = service.recv()? {
            println!("{package:?}")
        }
    }
}
