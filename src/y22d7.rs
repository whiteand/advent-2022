use std::{array::IntoIter, iter::Peekable, path::Iter, thread::current};

use nom::{
    branch, bytes,
    character::{self, complete::space1},
    sequence, IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum ChangeDirArgument<'input> {
    Root,
    Parent,
    Directory(&'input str),
}

#[derive(Debug, PartialEq, Eq)]
enum Query<'input> {
    List,
    ChangeDir(ChangeDirArgument<'input>),
}

#[derive(Debug)]
struct File<'input> {
    name: &'input str,
    size: usize,
}

impl File<'_> {
    fn has_name(&self, name: &str) -> bool {
        return self.name == name;
    }
}

#[derive(Debug)]
enum Node<'input> {
    Directory(&'input str),
    File(File<'input>),
}

#[derive(Debug)]
enum Tree<'input> {
    Root(Vec<Tree<'input>>),
    Directory(&'input str, Vec<Tree<'input>>),
    File(File<'input>),
}

impl<'input> Default for Tree<'input> {
    fn default() -> Self {
        Tree::Root(Vec::default())
    }
}

impl<'input> Tree<'input> {
    fn is_dir(&self) -> bool {
        match self {
            Tree::Root(_) => false,
            Tree::Directory(_, _) => true,
            Tree::File(_) => false,
        }
    }
    fn has_name(&self, name: &str) -> bool {
        match self {
            Tree::Root(_) => false,
            Tree::Directory(dir_name, _) => *dir_name == name,
            Tree::File(f) => f.has_name(name),
        }
    }
    fn add_file(self, path: &[&'input str], f: File<'input>) -> Self {
        if path.is_empty() {
            return match self {
                Tree::Root(mut children) => {
                    children.push(Tree::File(f));
                    Tree::Root(children)
                }
                Tree::Directory(dir_name, mut children) => {
                    children.push(Tree::File(f));
                    Tree::Directory(dir_name, children)
                }
                Tree::File(_) => todo!(),
            };
        }
        match self {
            Tree::Root(mut children) => {
                let maybe_index = {
                    children
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x.has_name(&path[0]) && x.is_dir())
                        .map(|(ind, _)| ind)
                };

                match maybe_index {
                    Some(index) => {
                        let new_child = { children.remove(index).add_file(&path[1..], f) };
                        children.push(new_child)
                    }
                    None => unreachable!(),
                }
                Tree::Root(children)
            }
            Tree::Directory(dir_name, mut children) => {
                let maybe_index = {
                    children
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x.has_name(&path[0]) && x.is_dir())
                        .map(|(ind, _)| ind)
                };

                match maybe_index {
                    Some(index) => {
                        let new_child = { children.remove(index).add_file(&path[1..], f) };
                        children.push(new_child)
                    }
                    None => unreachable!(),
                }
                Tree::Directory(dir_name, children)
            }
            Tree::File(f) => Tree::File(f),
        }
    }
    fn add_directory(self, path: &[&'input str]) -> Self {
        if path.is_empty() {
            return self;
        }
        if path.len() == 1 {
            let dir_name = path[0];
            return match self {
                Tree::Root(mut dirs) => {
                    if dirs
                        .iter()
                        .find(|x| x.has_name(dir_name) && x.is_dir())
                        .is_some()
                    {
                        return Tree::Root(dirs);
                    }
                    dirs.push(Tree::Directory(dir_name, Vec::new()));
                    Tree::Root(dirs)
                }
                Tree::Directory(parent, mut children) => {
                    if children
                        .iter()
                        .find(|x| x.has_name(dir_name) && x.is_dir())
                        .is_some()
                    {
                        return Tree::Directory(parent, children);
                    }
                    children.push(Tree::Directory(dir_name, Vec::new()));
                    Tree::Directory(parent, children)
                }
                Tree::File(_) => unreachable!(),
            };
        }
        let first_name = path[0];
        match self {
            Tree::Root(elements) => {
                let mut res = Vec::new();
                let mut added = false;
                for x in elements {
                    match x {
                        Tree::Root(_) => unreachable!(),
                        Tree::Directory(dir_name, grand_children) => {
                            if dir_name == first_name {
                                let new_child = Tree::Directory(dir_name, grand_children)
                                    .add_directory(&path[1..]);
                                res.push(new_child);
                                added = true;
                            } else {
                                res.push(Tree::Directory(dir_name, grand_children));
                            }
                        }
                        Tree::File(f) => res.push(Tree::File(f)),
                    }
                }
                if !added {
                    res.push(Tree::Directory(first_name, Vec::new()));
                }
                Tree::Root(res)
            }
            Tree::Directory(current_name, children) => {
                let mut res = Vec::new();
                let mut added = false;
                for x in children {
                    match x {
                        Tree::Root(_) => unreachable!(),
                        Tree::Directory(dir_name, grand_children) => {
                            if dir_name == first_name {
                                let new_child = Tree::Directory(dir_name, grand_children)
                                    .add_directory(&path[1..]);
                                res.push(new_child);
                                added = true;
                            } else {
                                res.push(Tree::Directory(dir_name, grand_children));
                            }
                        }
                        Tree::File(f) => res.push(Tree::File(f)),
                    }
                }
                if !added {
                    res.push(Tree::Directory(first_name, Vec::new()));
                }
                Tree::Directory(current_name, res)
            }
            Tree::File(_) => todo!(),
        }
    }

    fn total_size(&self) -> usize {
        match self {
            Tree::Root(dirs) => dirs.iter().fold(0, |a, b| a + b.total_size()),
            Tree::Directory(_, children) => children.iter().fold(0, |a, b| a + b.total_size()),
            Tree::File(f) => f.size,
        }
    }
}

#[derive(Debug)]
enum IO<'input> {
    List(Vec<Node<'input>>),
    ChangeDir(ChangeDirArgument<'input>),
}

fn parse_change_dir_query(line: &str) -> IResult<&str, Query> {
    nom::combinator::map(
        sequence::preceded(
            bytes::complete::tag("cd "),
            nom::branch::alt((
                nom::combinator::map(character::complete::alpha1, |name| {
                    ChangeDirArgument::Directory(name)
                }),
                nom::combinator::map(bytes::complete::tag(".."), |_| ChangeDirArgument::Parent),
                nom::combinator::map(bytes::complete::tag("/"), |_| ChangeDirArgument::Root),
            )),
        ),
        |arg| Query::ChangeDir(arg),
    )(line)
}
fn parse_list_query(line: &str) -> IResult<&str, Query> {
    nom::combinator::map(bytes::complete::tag("ls"), |_| Query::List)(line)
}
fn parse_query(line: &str) -> IResult<&str, Query> {
    sequence::preceded(
        bytes::complete::tag("$ "),
        branch::alt((parse_change_dir_query, parse_list_query)),
    )(line)
}

fn parse_file(line: &str) -> IResult<&str, Node> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::character::complete::u32,
            space1,
            nom::character::complete::not_line_ending,
        ),
        |(size, name)| {
            Node::File(File {
                name,
                size: size as usize,
            })
        },
    )(line)
}
fn parse_directory(line: &str) -> IResult<&str, Node> {
    nom::combinator::map(
        nom::sequence::preceded(
            nom::bytes::complete::tag("dir "),
            nom::character::complete::not_line_ending,
        ),
        |c| Node::Directory(c),
    )(line)
}

fn parse_node(line: &str) -> IResult<&str, Node> {
    nom::branch::alt((parse_file, parse_directory))(line)
}

struct IOParser<'input, Lines>
where
    Lines: Iterator<Item = &'input str>,
{
    lines: Peekable<Lines>,
}

impl<'input, Lines> Iterator for IOParser<'input, Lines>
where
    Lines: Iterator<Item = &'input str>,
{
    type Item = IO<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            None => None,
            Some(line) => {
                let (_, query) = parse_query(line).expect("valid query is expected");
                match query {
                    Query::List => {
                        let mut res = Vec::new();
                        loop {
                            match self.lines.peek() {
                                None => return Some(IO::List(res)),
                                Some(line) => match parse_node(line) {
                                    Ok((_, r)) => {
                                        self.lines.next().unwrap();
                                        res.push(r);
                                    }
                                    Err(_) => return Some(IO::List(res)),
                                },
                            }
                        }
                    }
                    Query::ChangeDir(argument) => Some(IO::ChangeDir(argument)),
                }
            }
        }
    }
}

fn parse_io(file_content: &str) -> impl Iterator<Item = IO<'_>> {
    IOParser {
        lines: file_content.lines().peekable(),
    }
}

fn scan_tree<'input>(ios: impl Iterator<Item = IO<'input>>) -> Tree<'input> {
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
                            tree = tree.add_directory(&full_path);
                        }
                        Node::File(f) => tree = tree.add_file(&current_path, f),
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
                    tree = tree.add_directory(&current_path);
                }
            },
        }
    }
    tree
}

struct TreeIter<'input> {
    to_visit: Vec<&'input Tree<'input>>,
}

impl<'input> Iterator for TreeIter<'input> {
    type Item = &'input Tree<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.is_empty() {
            return None;
        }
        let new_visit = self.to_visit.pop().unwrap();
        match new_visit {
            Tree::Root(dirs) => self.to_visit.extend(dirs.iter()),
            Tree::Directory(_, children) => self.to_visit.extend(children.iter()),
            Tree::File(_) => {}
        }
        Some(new_visit)
    }
}

impl<'input> IntoIterator for &'input Tree<'input> {
    type Item = &'input Tree<'input>;

    type IntoIter = TreeIter<'input>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIter {
            to_visit: vec![self],
        }
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    let tree = scan_tree(parse_io(file_content));

    tree.into_iter()
        .filter_map(|x| {
            if !x.is_dir() {
                return None;
            }
            let size = x.total_size();
            if size > 100000 {
                return None;
            }
            Some(size)
        })
        .sum()
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    let tree = scan_tree(parse_io(file_content));

    const TOTAL: usize = 70000000;
    const REQUIRED: usize = 30000000;

    let currently_free = TOTAL - tree.total_size();

    let to_delete = REQUIRED - currently_free;

    let mut possible: Vec<usize> = tree
        .into_iter()
        .filter_map(move |x| {
            if !x.is_dir() {
                return None;
            }
            let size = x.total_size();
            if size < to_delete {
                return None;
            }
            Some(size)
        })
        .collect();

    possible.sort_by(|a, b| b.cmp(a));
    possible.pop().unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "95437");
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "24933642");
    }

    #[test]
    fn test_parse_list_query() {
        assert_eq!(parse_query("$ ls"), Ok(("", Query::List)));
    }

    #[test]
    fn test_parse_change_dir_query_parent() {
        assert_eq!(
            parse_query("$ cd .."),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Parent)))
        );
    }
    #[test]
    fn test_parse_change_dir_query_root() {
        assert_eq!(
            parse_query("$ cd /"),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Root)))
        );
    }
    #[test]
    fn test_parse_change_dir_query_directory() {
        assert_eq!(
            parse_query("$ cd a"),
            Ok(("", Query::ChangeDir(ChangeDirArgument::Directory("a"))))
        );
    }
}
