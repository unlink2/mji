use mji::CFG;
#[macro_use]
extern crate mji;

fn main() -> Result<(), anyhow::Error> {
    let mut stdout = std::io::stdout().lock();
    let mjimap = CFG.read().unwrap().mji_map();
    if mjimap.is_err() {
        print_error_and_exit!(mjimap);
    }
    let mjimap = mjimap.unwrap();

    if CFG.read().unwrap().list {
        mji::mjimap::list(&mut stdout, &mjimap);
    } else {
        if CFG.read().unwrap().inputs.is_empty() {
            mji::prompt::prompt(&mut stdout, &mjimap);
        }
        let inputs = CFG.read().unwrap().inputs.clone();
        let v: Vec<&str> = inputs.iter().map(|x| &**x).collect();
        // TODO better errors
        if CFG.read().unwrap().commit {
            let mut buffer = Vec::<u8>::new();
            print_error_and_exit!(mji::mjimap::find_or(&mut buffer, &mjimap, &v));
            print_error_and_exit!(mji::mjimap::commit(&buffer));
        } else {
            print_error_and_exit!(mji::mjimap::find_or(&mut stdout, &mjimap, &v));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {}
