use std::{
    io,
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    use std::{
        env,
        process::{self, Command},
    };

    let mut pwd = env::current_dir()?;

    loop {
        let prompt = gen_prompt(&pwd);

        let input = prompt_for_input(&prompt)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let input_split = input.split_whitespace();

        // We know at this point that input is not empty so we can unwrap.
        let cmd_name = input_split.clone().take(1).next().unwrap();

        let args: Vec<_> = input_split.skip(1).collect();

        match cmd_name {
            "cd" => {
                let args = args.join(" ");
                cd(&args, &mut pwd);
            }
            "exit" => process::exit(0),
            _ => {
                if let Err(e) = Command::new(cmd_name).args(args).current_dir(&pwd).status() {
                    eprintln!("sh: {}", e);
                }
            }
        }
    }
}

fn prompt_for_input(p: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", p);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}

fn gen_prompt(pwd: &Path) -> String {
    format!("{}> ", pwd.display())
}

fn prefix_path(prefix: PathBuf, base: &Path) -> PathBuf {
    let mut path = prefix;
    path.push(base);
    path
}

fn cd(target: &str, pwd: &mut PathBuf) {
    use path_clean::PathClean;

    let path = if target.is_empty() {
        match dirs::home_dir() {
            Some(home_dir) => home_dir,
            None => {
                eprintln!("cd: could not determine home directory.");
                return;
            }
        }
    } else {
        let mut path = Path::new(&target).to_path_buf();

        // Make path absolute if it is relative by prefixing it with the current
        // directory.
        if path.is_relative() {
            path = prefix_path(pwd.clone(), &path);
        }

        path
    }
    .clean();

    match path {
        path if !path.exists() => {
            eprintln!("cd: file or directory ‘{}’ does not exist.", path.display())
        }
        path if !path.is_dir() => eprintln!("cd: path ‘{}’ is not a directory.", path.display()),
        _ => *pwd = path,
    }
}
