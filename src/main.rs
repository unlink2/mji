use mji::{gitmoji::GITMOJI, Error, CFG};
#[macro_use]
extern crate mji;

fn main() -> Result<(), Error> {
    let mut stdout = std::io::stdout().lock();

    if CFG.read().unwrap().list {
        mji::mjimap::list(&mut stdout, &GITMOJI);
    } else {
        if CFG.read().unwrap().inputs.is_empty() {
            mji::prompt::prompt(&mut stdout, &GITMOJI);
        }
        let inputs = CFG.read().unwrap().inputs.clone();
        let v: Vec<&str> = inputs.iter().map(|x| &**x).collect();
        // TODO better errors
        if CFG.read().unwrap().commit {
            let mut buffer = Vec::<u8>::new();
            print_error!(mji::mjimap::find_or(&mut buffer, &GITMOJI, &v));
            print_error!(mji::mjimap::commit(&buffer));
        } else {
            print_error!(mji::mjimap::find_or(&mut stdout, &GITMOJI, &v));
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {}
