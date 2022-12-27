use mji::{gitmoji::GITMOJI, CFG};

fn main() {
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
            mji::mjimap::find_or(&mut buffer, &GITMOJI, &v).unwrap();
            mji::mjimap::commit(&buffer).unwrap();
        } else {
            mji::mjimap::find_or(&mut stdout, &GITMOJI, &v).unwrap();
        }
    }
}

#[cfg(test)]
mod test {}
