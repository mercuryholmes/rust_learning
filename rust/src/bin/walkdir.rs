use std::env;
use anyhow::{Result, Context};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
enum Error {
    #[error("ArgmentError: {0}")]
    ArgmentError(String),
}

fn main() -> Result<()>{
  // 引数の値を取得
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => {
      Err(Error::ArgmentError("no argumnt".to_string()))?;
    },
    2 => {
      let directory = &args[1];
      ls(&directory).with_context(|| format!(" No such file or directory: {}", directory))?;
    },
    _ => {
      Err(Error::ArgmentError("too many argument".to_string()))?;
    }
  }
  Ok(())
}

// 対象ディレクトリを探索する関数
fn ls(directory: &String) -> Result<()> {
  for entry in WalkDir::new(directory) {
    println!("{}", entry?.path().display());
  }
  Ok(())
}