use std::{
    collections::HashMap,
    ffi::OsString,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::Stdio,
};

type EnvVars = HashMap<String, OsString>;
type Executables = HashMap<String, PathBuf>;

fn populate_env(
    env_vars: &mut EnvVars,
    file_path: &Path,
    args: impl Iterator<Item = String>,
    envs: impl Iterator<Item = (String, OsString)>,
) -> Result<(), ()> {
    let Ok(mut process) = std::process::Command::new(file_path)
        .args(args)
        .envs(envs)
        .stdout(Stdio::piped())
        .spawn()
    else {
        return Err(());
    };
    let stdout = process.stdout.as_mut().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            let separator = '=';
            let Some((name, value)) = line.split_once(separator) else {
                panic!(
                    "no {:?} present in line \"{}\" from {:?}",
                    separator, line, file_path
                );
            };
        }
    }
    let Ok(new_env) = std::fs::read_to_string(file_path) else {
        return Err(());
    };
    for (line_index, line) in new_env.lines().enumerate() {
        if !line.chars().next().is_some_and(|c| c != '#') {
            env_vars.insert(name.into(), value.into());
        }
    }
    Ok(())
}
fn populate_exec(executables: &mut Executables, dir_path: &Path) -> Result<(), ()> {
    let Ok(file_paths) = std::fs::read_dir(dir_path) else {
        return Err(());
    };
    for file_path in file_paths {
        let file = file_path.unwrap();
        executables.insert(file.file_name().into_string().unwrap(), file.path());
    }
    Ok(())
}
fn run(command: &mut std::process::Command) -> i32 {
    command.spawn().unwrap().wait().unwrap().code().unwrap()
}

const ENV_EXECUTABLE_FILE_NAME: &str = "one.config";
const COMMAND_DIR_NAME: &str = "one.config.commands";

fn main() {
    let mut args = std::env::args().skip(1);
    let command_name = args
        .next()
        .expect("first argument should contain command name");
    let mut env_vars = EnvVars::new();
    let mut executables = Executables::new();

    let global_config_dir = dirs::config_local_dir().unwrap();
    let _ = populate_env(
        &mut env_vars,
        &global_config_dir.join(ENV_EXECUTABLE_FILE_NAME),
    );
    let _ = populate_exec(&mut executables, &global_config_dir.join(COMMAND_DIR_NAME));
    for ancestor in std::env::current_dir().unwrap().ancestors() {
        let env_found =
            populate_env(&mut env_vars, &ancestor.join(ENV_EXECUTABLE_FILE_NAME)).is_ok();
        let exec_found = populate_exec(&mut executables, &ancestor.join(COMMAND_DIR_NAME)).is_ok();
        if env_found || exec_found {
            env_vars.insert("PROJECT_ROOT".into(), ancestor.into());
            break;
        }
    }

    let Some(executable_path) = executables.get(&command_name) else {
        let mut executables: Vec<_> = executables.iter().collect();
        executables.sort_by_key(|(name, _path)| {
            ordered_float::OrderedFloat::from(strsim::jaro(name, &command_name))
        });
        eprintln!("\"{command_name}\" was not found. Closest matches:");
        for (name, path) in executables.iter().rev().take(5) {
            eprintln!("* {name} at {path:?}");
        }
        std::process::exit(1);
    };

    std::process::exit(run(std::process::Command::new(&executable_path)
        .args(args)
        .envs(env_vars)));
}
