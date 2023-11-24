use std::error::Error;

const PROBLEM: &str = include_str!("input/day7.txt");

struct Directory {
    // subdirs: Vec<Box<Directory>>,
    subdirs: std::collections::HashMap<String, Box<Directory>>,
    files: Vec<File>,
    name: String,
    size: usize,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            subdirs: std::collections::HashMap::<String, Box<Directory>>::new(),
            files: Vec::<File>::new(),
            name: name.to_string(),
            size: 0
        }
    }

    fn add_directory(&mut self, name: &str) {
        self.subdirs.insert(name.to_string(), Box::new(Directory::new(name)));
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File::new(name, size));
    }

    fn mut_subdir(&mut self, name: &str) -> Option<&mut Box<Directory>> {
        self.subdirs.get_mut(name)
    }

    fn subdir(&self, name: &str) -> Option<&Box<Directory>> {
        self.subdirs.get(name)
    }

    fn calculate_size(&mut self) -> usize {
        let subdirs_size = self.subdirs.iter_mut()
            .map(|(_, subdir)| subdir.calculate_size())
            .sum::<usize>();
        let files_size = self.files.iter()
            .map(|file| file.size)
            .sum::<usize>();

        self.size = files_size + subdirs_size;
        self.size
    }
}

struct File {
    name: String,
    size: usize
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size
        }
    }
}

fn run_commands(dir: &mut Directory, cmd: &[&str]) -> usize {
    println!("entering {}", dir.name);

    if cmd.len() > 0 {
        let mut i: usize = 0;
        while i < cmd.len() {
            println!("{}: {}", dir.name, &cmd[i]);
            if &cmd[i][..4] == "$ ls" {
                // run everything up to next command in this dir
                i += 1;
                while i < cmd.len() && &cmd[i][0..1] != "$" {
                    if &cmd[i][0..3] == "dir" {
                        dir.add_directory(&cmd[i][4..])
                    } else {
                        let space = cmd[i].find(" ").expect("Missing ' '");
                        let size = cmd[i][0..space].parse::<usize>().expect("Failed to parse filesize");
                        let name = &cmd[i][space+1..];
                        dir.add_file(name, size);
                    }
                    i += 1;
                }
            } else if &cmd[i][..4] == "$ cd" {
                if &cmd[i][5..] == ".." {
                    return i + 1;
                } else {
                    let subdir = dir.mut_subdir(&cmd[i][5..])
                        .expect("Tried to cd into nonexisting directory");
                    i += run_commands(subdir.as_mut(), &cmd[i+1..]) + 1;
                }
            } else {
                return i;
            }
        }
        
    }

    0
}

fn count_less_than(root: &Directory, n: usize) -> usize {
    let mut size: usize = if root.size <= n {root.size} else { 0 };
    for (_, dir) in &root.subdirs {
        size += count_less_than(dir, n);
    }

    size
}

pub fn solution1() -> Result<(), Box<dyn Error + 'static>> {
    let mut root = Directory::new("/");

    let lines: Vec<&str> = PROBLEM.split("\n").collect();

    run_commands(&mut root, lines.as_slice());
    root.calculate_size();

    let answer = count_less_than(&root, 100000);
    println!("Answer: {}", answer);

    Ok(())
}

fn find_smallest_directory_bigger_than(root: &Directory, n: usize) -> Option<usize> {
    let min = root.size;
    let dirs_min = root.subdirs.values()
        .filter_map(|dir| find_smallest_directory_bigger_than(dir.as_ref(), n))
        .min();

    if min < n {
        None
    } else if dirs_min < Some(n) {
        Some(min)
    } else {
        dirs_min
    }
}

pub fn solution2() -> Result<(), Box<dyn Error + 'static>> {
    let mut root = Directory::new("/");

    let lines: Vec<&str> = PROBLEM.split("\n").collect();

    run_commands(&mut root, lines.as_slice());
    root.calculate_size();

    println!("Total space used: {}/{}", root.size, 70000000);
    let answer = find_smallest_directory_bigger_than(&root, 7808823)
        .expect("Failed to find directory");
    println!("Answer: {}", answer);

    Ok(())
}
