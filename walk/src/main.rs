use jwalk::WalkDir;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let output = Mutex::new(Vec::with_capacity(args.len()));

    args.par_iter().for_each(|arg| {
        let dir_tree = WalkDir::new(arg)
            .skip_hidden(false)
            .into_iter()
            .map(|entry| entry.unwrap().path());

        dir_tree.for_each(|entry| {
            let mut output = output.lock();
            output.push(entry);
        });
    });

    let output = output.lock();

    output
        .iter()
        .for_each(|entry| println!("{}", entry.display()));
}
