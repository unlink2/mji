use crate::mjimap::MjiMap;
use crate::CFG;
use rustyline::error::ReadlineError;
use rustyline::hint::{Hint, Hinter};
use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};
use std::collections::HashSet;
use std::io::Write;

#[derive(Completer, Helper, Validator, Highlighter)]
struct MjiHinter {
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl CommandHint {
    fn new(text: &str, complete_up_to: &str) -> CommandHint {
        assert!(text.starts_with(complete_up_to));
        CommandHint {
            display: text.into(),
            complete_up_to: complete_up_to.len(),
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hinter for MjiHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<CommandHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints.iter().find_map(|hint| {
            if hint.display.starts_with(line) {
                Some(hint.suffix(pos))
            } else {
                None
            }
        })
    }
}

fn mji_hints(map: &MjiMap) -> HashSet<CommandHint> {
    let mut set = HashSet::new();
    map.iter().for_each(|v| {
        set.insert(CommandHint::new(&v.1.name, &v.1.name));
    });
    set
}

pub fn prompt(_f: &mut dyn Write, map: &MjiMap) {
    println!("Starting commit. Terminate input with CTRL-D.");

    let h = MjiHinter {
        hints: mji_hints(map),
    };
    let mut rl = Editor::<MjiHinter>::new().unwrap();
    rl.set_helper(Some(h));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let mut list = line
                    .split_whitespace()
                    .map(|x| x.trim().to_owned())
                    .collect::<Vec<String>>();

                if list.is_empty() || crate::mjimap::find(map, &list[0]).is_err() {
                    println!("The first word in each commit line must be a valid mji list entry!");
                } else {
                    CFG.write().unwrap().inputs.append(&mut list);
                    CFG.write().unwrap().inputs.push("-".into());
                }
            }
            Err(ReadlineError::Interrupted) => {
                std::process::exit(0);
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
}
