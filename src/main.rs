use mji::{gitmoji::GITMOJI, CFG};

fn main() {
    if CFG.list {
        mji::mjimap::list(&GITMOJI);
    } else {
        let v: Vec<&str> = CFG.inputs.iter().map(|x| &**x).collect();
        // TODO better errors
        mji::mjimap::find_or(&GITMOJI, &v).unwrap();
    }
}

#[cfg(test)]
mod test {}
