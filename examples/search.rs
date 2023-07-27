use pro_dj_link::*;

pub fn main() -> ProDjLinkResult<()> {
    let mut search = SearchService::new()?;

    loop {
        if let Some(package) = search.recv()? {
            println!("{package:?}")
        }
    }
}
