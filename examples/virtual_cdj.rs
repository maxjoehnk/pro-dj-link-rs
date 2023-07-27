use pro_dj_link::*;

pub fn main() -> ProDjLinkResult<()> {
    let mut cdj = VirtualCdj::new()?;

    loop {
        cdj.send_keep_alive()?;
        if let Some(packet) = cdj.recv()? {
            println!("{packet:?}");
        }
    }
}
