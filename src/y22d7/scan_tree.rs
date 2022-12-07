use super::io::ChangeDirArgument;
use super::io::Node;
use super::io::IO;
use super::tree::Tree;

pub(crate) fn scan_tree<'input>(ios: impl Iterator<Item = IO<'input>>) -> Tree<'input> {
    let mut tree = Tree::default();
    let mut current_path = Vec::new();
    for io in ios {
        match io {
            IO::List(children) => {
                for child in children {
                    match child {
                        Node::Directory(d) => {
                            let mut full_path = current_path.clone();
                            full_path.push(d);
                            tree.add_directory(&full_path);
                        }
                        Node::File(f) => tree.add_file(&current_path, f),
                    }
                }
            }
            IO::ChangeDir(arg) => match arg {
                ChangeDirArgument::Root => current_path.clear(),
                ChangeDirArgument::Parent => {
                    current_path.pop().unwrap();
                }
                ChangeDirArgument::Directory(d) => {
                    current_path.push(d);
                    tree.add_directory(&current_path);
                }
            },
        }
    }
    tree
}
