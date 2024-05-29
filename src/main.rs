use core::panic;
use ignore::gitignore::Gitignore;
use node::{DirTreeType, TreeNode};
use std::{
    fs::DirEntry,
    io::{self, BufWriter, Write},
    path::Path,
    vec,
};

use crate::cli::CliArgs;

mod cli;
mod node;

fn main() {
    let args = &CliArgs::parse_opts();

    let main_path = &args.path;

    let mut out = BufWriter::new(std::io::stdout());

    match (main_path.try_exists(), main_path.is_dir()) {
        (Ok(false), _) => panic!(
            "The specified path {} does not exist",
            main_path.as_os_str().to_str().unwrap()
        ),
        (Err(err), _) => panic!("Error reading dir {}", err),
        (Ok(true), false) => panic!("The specified path is not a directory"),
        _ => (),
    }

    let giti = {
        if args.use_git_ignore {
            let (g, error) = Gitignore::new(".gitignore");
            if let Some(err) = error {
                panic!("{}", err);
            };
            Some(g)
        } else {
            None
        }
    };

    let tree = TreeNode::new(
        main_path.clone(),
        DirTreeType::Dir(recursive_get(&args, main_path, &giti, 1).unwrap()),
        0,
    );

    tree.display_ascii(&mut out, args, false);

    out.flush().unwrap();
}

fn recursive_get(
    cmd: &CliArgs,
    path: impl AsRef<Path>,
    gitignore: &Option<Gitignore>,
    current_depth: u8,
) -> io::Result<Vec<TreeNode>> {
    if current_depth > cmd.depth {
        return Ok(vec![]);
    }
    let paths = std::fs::read_dir(path)?;

    let nodes: Vec<TreeNode> = paths
        .into_iter()
        .filter_map(|node| node.ok())
        .filter(filter_hidden_files)
        .filter(|node| {
            if !cmd.show_hidden {
                return !node
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .to_owned()
                    .unwrap()
                    .starts_with(".");
            }
            return true;
        })
        // Ignore .gitignore matched files and dirs
        .filter(move |node| {
            if let Some(g) = gitignore {
                return !g.matched(node.path(), node.path().is_dir()).is_ignore();
            }
            return true;
        })
        .filter(|node| {
            let blacklists = &cmd.blacklist_patterns;
            return !blacklists.into_iter().any(|ex| {
                return node
                    .path()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    .contains(ex.as_str());
            });
        })
        .map(|node| {
            if node.path().is_file() {
                return Some(TreeNode::new(node.path(), DirTreeType::File, current_depth));
            } else if node.path().is_dir() {
                return Some(TreeNode::new(
                    node.path(),
                    DirTreeType::Dir(
                        recursive_get(cmd, node.path(), gitignore, current_depth + 1).unwrap(),
                    ),
                    current_depth,
                ));
            }
            return None;
        })
        .flat_map(|node| node)
        .collect();

    return Ok(nodes);
}

#[cfg(target_os = "windows")]
fn filter_hidden_files(node: &DirEntry) -> bool {
    use std::os::windows::prelude::*;

    let metadata = std::fs::metadata(node.path());

    return match metadata {
        Ok(meta) => !((meta.file_attributes() & 0x2) > 0),
        Err(_) => true,
    };
}

#[cfg(target_os = "linux")]
fn filter_hidden_files(node: &DirEntry) -> bool {
    return true;
}

#[cfg(target_os = "macos")]
fn filter_hidden_files(node: &DirEntry) -> bool {
    return true;
}
