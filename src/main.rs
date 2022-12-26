use mji::{gitmoji::GITMOJI, CFG};

fn main() {
    let mut stdout = std::io::stdout().lock();

    if CFG.list {
        mji::mjimap::list(&mut stdout, &GITMOJI);
    } else {
        let v: Vec<&str> = CFG.inputs.iter().map(|x| &**x).collect();
        // TODO better errors
        if CFG.commit {
            let mut buffer = Vec::<u8>::new();
            mji::mjimap::find_or(&mut buffer, &GITMOJI, &v).unwrap();
            mji::mjimap::commit(&buffer).unwrap();
        } else {
            mji::mjimap::find_or(&mut stdout, &GITMOJI, &v).unwrap();
        }
    }
}

#[cfg(test)]
mod test {}
