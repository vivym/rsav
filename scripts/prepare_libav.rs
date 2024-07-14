#!/bin/sh
#![allow(unused_attributes)] /*
OUT=/tmp/tmp && rustc "$0" -o ${OUT} && exec ${OUT} $@ || exit $? #*/

use std::io::Result;
use std::path::PathBuf;

fn mkdir(dir_name: &str) -> Result<()> {
    std::fs::create_dir_all(dir_name)
}

fn pwd() -> Result<PathBuf> {
    std::env::current_dir()
}

fn cd(dir_name: &str) -> Result<()> {
    std::env::set_current_dir(dir_name)
}

fn main() -> Result<()> {
    mkdir("tmp")?;
    Ok(())
}
