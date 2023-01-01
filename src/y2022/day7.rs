use std::{fmt::Display, rc::Rc, collections::HashMap, cell::RefCell};
use super::super::day::Day;

pub struct Day7
{
    #[allow(dead_code)]
    history: Vec<HistoryItem>,
    file_root: Option<Rc<FileSystemItem>>,
}

impl Day7 {
    pub fn new() -> Day7
    {
        let input = include_str!("input7");
        //let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";

        // Splitting on $ to get a command and its arguments
        let history = input.split("$ ").map(str::trim).filter_map(HistoryItem::from_lines).collect();

        Day7 { history, file_root: None }
    }
}

impl Day for Day7 {
    fn day_name(&self) -> String { String::from("07") }
    fn answer1(&self) -> String { String::from("1315285") }
    fn answer2(&self) -> String { String::from("9847279") }

    fn part1(&mut self) -> String {
        self.file_root = Some(self.build_file_system());
        self.count_small_dirs().to_string()
    }

    fn part2(&mut self) -> String {
        self.find_deletion_target().to_string()
    }
}

impl Day7 {
    fn build_file_system(&self) -> Rc<FileSystemItem> {
        let root = Rc::new(FileSystemItem { name: "/".to_string(), size: 0, children: RefCell::new(HashMap::new()), parent: None, depth: 0 });
        let mut working_dir = Rc::clone(&root);

        for command in &self.history {
            match &command.command {
                Command::ChangeDir(target) => {
                    // Can we trust target to always exist?
                    if target.eq("/") {
                        working_dir = Rc::clone(&root)
                    } else {
                        working_dir = working_dir.find_child(target); 
                    }
                }
                Command::List => {
                    // Do we know we won't ls the same dir twice?
                    for item in &command.output {
                        match item {
                            OutputItem::File(name, size) => {
                                working_dir.children.borrow_mut().insert(name.to_string(), 
                                    Rc::new(FileSystemItem {name: name.to_string(), size: *size, depth: working_dir.depth + 1, children: RefCell::new(HashMap::new()), parent: Some(Rc::clone(&working_dir))})
                                );
                            },
                            OutputItem::Directory(name) => {
                                working_dir.children.borrow_mut().insert(name.to_string(),
                                    Rc::new(FileSystemItem {name: name.to_string(), size: 0, depth: working_dir.depth + 1, children: RefCell::new(HashMap::new()), parent: Some(Rc::clone(&working_dir))})
                                );
                            },
                        }
                    }
                },
            }
        }

        root
    }

    fn count_small_dirs(&self) -> usize {
        let dir = self.file_root.as_ref().unwrap();
        let mut sizes = vec![];
        Self::get_size(dir, &mut sizes);
        sizes.sort(); // OPTIMISATION I bet we don't need to do this here

        sizes.iter().filter(|&s| *s < 100000).sum()
    }

    fn find_deletion_target(&self) -> usize {
        let dir = self.file_root.as_ref().unwrap();
        // OPTIMISATION: calculate this above already, save it and reuse it
        let mut sizes = vec![];
        let total_size = Self::get_size(dir, &mut sizes);
        sizes.sort();

        let free_space = 70000000 - total_size;
        let deletion_needed = 30000000 - free_space;

        let deleted_size = *sizes.iter().find(|&size| *size >= deletion_needed).expect("Nothing was big enough");

        deleted_size
    }

    fn get_size(item: &FileSystemItem, sizes: &mut Vec<usize>) -> usize {
        // Is populating a vec the sensible way to do this? Not sure, but it works and its fast
        if item.size > 0 {
            return item.size;
        }

        let size = item.children.borrow()
            .values()
            .map(|child| Self::get_size(child, sizes))
            .sum();
        
        sizes.push(size);

        size
    }
}

// RefCell<HashMap<Rc> structure suggested by 'Uncle Scientist' on youtube. Thanks!
struct FileSystemItem {
    name: String,
    size: usize,
    parent: Option<Rc<FileSystemItem>>,
    children: RefCell<HashMap<String, Rc<FileSystemItem>>>,
    depth: usize,
}

impl FileSystemItem {
    fn find_child(&self, target: &str) -> Rc<FileSystemItem> {
        if target.eq("..") {
            Rc::clone(self.parent.as_ref().unwrap())
        } else {
            Rc::clone(self.children.borrow().get(target).unwrap())
        }
    }
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
        for child in self.children.borrow().values() {
            write!(f, "{}- {child}", " ".repeat(self.depth)).expect("!");
        }

        Ok(())
    }
}