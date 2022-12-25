use mji::{gitmoji::GITMOJI, CFG};

fn main() {
    if CFG.list {
        mji::mjimap::list(&GITMOJI);
    } else {
        mji::mjimap::find_or(&GITMOJI, &CFG.inputs);
    }
}

#[cfg(test)]
mod test {}
