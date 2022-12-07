use std::iter::Peekable;

use nom::{branch, bytes, character, sequence, IResult};

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

#[derive(Debug)]
enum Node<'input> {
    Directory(&'input str),
    File(File<'input>),
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
                            todo!();
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

pub fn solve_task1(file_content: &str) -> usize {
    for io in parse_io(file_content) {
        println!("{io:?}")
    }
    0
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    0
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
        assert_eq!(format!("{}", solve_task1(INPUT)), "0");
    }
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
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
