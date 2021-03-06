// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn create_crate_template(path: PathBuf, name: &str) -> Result<(), std::io::Error> {
    create_directory_structure(&path)?;
    create_control_file(&path, name)?;
    create_cargo_toml(&path, name)?;
    create_dotcargo_config(&path, name)?;
    create_lib_rs(&path, name)?;
    create_git_ignore(&path, name)?;

    let cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&path)?;
    crate::generate_schema()?;
    std::env::set_current_dir(cwd)?;

    git_init(&path)?;
    git_add(&path)
}

fn create_directory_structure(path: &PathBuf) -> Result<(), std::io::Error> {
    let mut src_dir = path.clone();

    src_dir.push("src");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push(".cargo");
    std::fs::create_dir_all(&src_dir)?;

    src_dir.pop();
    src_dir.push("sql");
    std::fs::create_dir_all(&src_dir)
}

fn create_control_file(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(format!("{}.control", name));
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/control"), name = name).as_bytes())?;

    Ok(())
}

fn create_cargo_toml(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("Cargo.toml");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/cargo_toml"), name = name).as_bytes())?;

    Ok(())
}

fn create_dotcargo_config(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".cargo");
    filename.push("config");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(include_bytes!("../templates/cargo_config"))?;

    Ok(())
}

fn create_lib_rs(path: &PathBuf, name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push("src");
    filename.push("lib.rs");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(&format!(include_str!("../templates/lib_rs"), name = name).as_bytes())?;

    Ok(())
}

fn create_git_ignore(path: &PathBuf, _name: &str) -> Result<(), std::io::Error> {
    let mut filename = path.clone();

    filename.push(".gitignore");
    let mut file = std::fs::File::create(filename)?;

    file.write_all(include_bytes!("../templates/gitignore"))?;

    Ok(())
}

fn git_init(path: &PathBuf) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .arg("init")
        .arg(".")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        Err(std::io::Error::from_raw_os_error(
            output.status.code().unwrap(),
        ))
    } else {
        Ok(())
    }
}

fn git_add(path: &PathBuf) -> Result<(), std::io::Error> {
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        Err(std::io::Error::from_raw_os_error(
            output.status.code().unwrap(),
        ))
    } else {
        Ok(())
    }
}
