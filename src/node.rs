use std::path::PathBuf;

use crate::CliArgs;

#[derive(Debug)]
pub enum DirTreeType {
    File,
    Dir(Vec<TreeNode>),
}

#[derive(Debug)]
pub struct TreeNode {
    path: PathBuf,
    tree_type: DirTreeType,
    depth: u8,
}

impl TreeNode {
    pub fn new(path: PathBuf, tree: DirTreeType, depth: u8) -> TreeNode {
        return TreeNode {
            path,
            tree_type: tree,
            depth,
        };
    }

    pub fn display_ascii(
        &self,
        writer: &mut impl std::io::Write,
        config: &CliArgs,
        is_last_child: bool,
    ) {
        let mut node_info_vec: Vec<String> = Vec::new();

        if self.depth > 0 {
            let mut separators = vec!["│"; (self.depth - 1).into()];
            match (is_last_child, &self.tree_type) {
                (true, DirTreeType::File) => separators.push("└──"),
                (true, DirTreeType::Dir(children)) => {
                    if children.len() > 0 {
                        separators.push("├──")
                    } else {
                        separators.push("└──")
                    }
                }
                _ => separators.push("├──"),
            }
            node_info_vec.push(separators.join("   "));
        }

        if config.display_emoji {
            node_info_vec.push(self.to_emoji().into());
        }

        node_info_vec.push(
            self.path
                .canonicalize()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        );

        writeln!(writer, "{}", node_info_vec.join(" ")).unwrap();

        match self.tree_type {
            DirTreeType::Dir(ref children) => {
                children.into_iter().enumerate().for_each(|(index, node)| {
                    node.display_ascii(writer, config, index == children.len() - 1)
                })
            }
            _ => {}
        }
    }

    pub fn to_emoji(&self) -> &str {
        match self.tree_type {
            DirTreeType::File => "📄",
            DirTreeType::Dir(ref children) => {
                if children.len() > 0 {
                    return "📁";
                }
                return "📂";
            }
        }
    }
}
