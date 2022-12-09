use std::fmt::Display;
use super::super::day::Day;

pub struct Day7
{
    #[allow(dead_code)]
    history: Vec<HistoryItem>,
}

impl Day7 {
    pub fn new() -> Day7
    {
        let input = include_str!("input7");
        //let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

        // Splitting on $ to get a command and its arguments
        let history = input.split("$ ").map(str::trim).filter_map(HistoryItem::from_lines).collect();

        //while lines.peekable

        Day7 { history }
    }
}

impl Day for Day7 {
    fn day_name(&self) -> String { String::from("07") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        let _ = self.build_file_system();

        //println!("{}", fileRoot);
        
        let ans1 = 0;
        let ans2 = 0;

        //println!("{}, {}", ans1, ans2);
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day7 {
    fn build_file_system(&self) -> FileSystemItem {
        let root = FileSystemItem { name: "/".to_string(), size: 0, children: vec![] };
        root

        // This sucks!
        /* 
        let directories = self.find_all_directories();

        for command in &self.history {
            match &command.command {
                Command::ChangeDir(target) => {
                    // Can we trust target to always exist?
                    let new_child = working_dir.find_child(target);
                    working_dir = working_dir.find_child(target); 
                }
                Command::List => {
                    // Do we know we won't ls the same dir twice?
                    for item in &command.output {
                        match item {
                            OutputItem::File(name, size) => {
                                working_dir.children.push(FileSystemItem {name: name.to_string(), size: *size, children: vec![]})
                            },
                            OutputItem::Directory(name) => {
                                working_dir.children.push(FileSystemItem {name: name.to_string(), size: 0, children: vec![]})
                            },
                        }
                    }
                },
            }
        }

        root
        */
    }
}

struct FileSystemItem {
    name: String,
    size: usize,
    children: Vec<FileSystemItem>,
}

impl FileSystemItem {
    /* 
    fn find_child(mut self, target: &str) -> &mut FileSystemItem {
        for child in &mut self.children {
            if child.name.eq(target) {
                return child.borrow_mut();
            }
        }
        panic!("didn't find child");
    }*/
}

struct HistoryItem {
    command: Command,
    output: Vec<OutputItem>,
}

impl HistoryItem {
    fn from_lines(input: &str) -> Option<HistoryItem> {
        // need to handle empty input as the first split will be before the first $ in the input
        if input.is_empty() { return None }
        //println!("History Item:");
        //println!("'{}'", input);
        let mut lines = input.split('\n');
        let command_str = lines.next().expect("Got sent empty lines!");
        let command = Command::from_str(command_str);

        let mut output = vec![];
        for line in lines {
            output.push(OutputItem::from_str(line));
        }

        Some(HistoryItem {
            command, output
        })
    }
}

enum Command {
    ChangeDir(String),
    List,
}

impl Command {
    fn from_str(input: &str) -> Command {   
        //println!("Command: {}", input);
        let parts = input.split(' ').collect::<Vec<_>>();
        match parts[0] {
            "cd" => Self::ChangeDir(parts[1].to_string()),
            "ls" => Self::List,
            _ => panic!("Invalid Command!"),
        }
    }
}

enum OutputItem {
    File(String, usize),
    Directory(String),
}

impl OutputItem {
    fn from_str(input: &str) -> OutputItem {
        let parts = input.split(' ').collect::<Vec<_>>();
        if parts[0].eq("dir") {
            return Self::Directory(parts[1].to_string());
        }

        let size = parts[0].parse::<usize>().expect("Invalid file size");

        Self::File(parts[1].to_string(), size)
    }
}

impl Display for HistoryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.command).expect("!");
        for o in &self.output {
            writeln!(f, "  {o}").expect("!");
        }
        Ok(())
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::ChangeDir(name) => write!(f, "cd {name}"),
            Self::List => write!(f, "ls"),
        }
    }
}

impl Display for OutputItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Directory(name) => write!(f, "dir {name}"),
            Self::File(name, size) => write!(f, "{name} ({size})"),
        }
    }
}

impl Display for FileSystemItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ({})", self.name, if self.size == 0 { "dir".to_string() } else { self.size.to_string() }).expect("!");
        for child in &self.children {
            writeln!(f, "  - {child}").expect("!");
        }

        Ok(())
    }
}