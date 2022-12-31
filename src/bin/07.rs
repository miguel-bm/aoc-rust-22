use std::collections::HashMap;

#[derive(Debug)]
struct FileStructure {
    files: HashMap<String, u32>,
    directories: HashMap<String, FileStructure>,
}
impl FileStructure {
    fn files_size(&self) -> u32 {
        self.files.values().sum()
    }
}
impl FileStructure {
    fn total_size(&self) -> u32 {
        let dirs_size = self
            .directories
            .values()
            .map(|dir| dir.total_size())
            .sum::<u32>();
        dirs_size + self.files_size()
    }
}

fn parse_input(input: &str) -> FileStructure {
    let mut file_structure = FileStructure {
        files: HashMap::new(),
        directories: HashMap::new(),
    };
    let mut current_dir = Vec::new();
    let mut reading_ls_output = false;
    for instruction in input.lines() {
        let first_char = &instruction[0..1];
        if first_char == "$" {
            let command = &instruction[2..4];
            if command == "cd" {
                reading_ls_output = false;
                let dir = &instruction[5..];
                if dir == ".." {
                    current_dir.pop();
                } else if dir != "/" {
                    current_dir.push(dir);
                }
            } else if command == "ls" {
                reading_ls_output = true;
            }
        } else {
            assert!(reading_ls_output);
            // navigate to current dir
            let mut dir = &mut file_structure;
            for subdir_name in current_dir.iter() {
                let next_dir_name = subdir_name.to_string();
                dir = dir.directories.get_mut(&next_dir_name).unwrap();
            }

            let mut parts = instruction.split_whitespace();
            let first_part = parts.next().unwrap();
            if first_part == "dir" {
                let dir_name = parts.next().unwrap().to_string();
                dir.directories.insert(
                    dir_name,
                    FileStructure {
                        files: HashMap::new(),
                        directories: HashMap::new(),
                    },
                );
            } else {
                let file_size: u32 = first_part.parse().unwrap();
                let file_name = parts.next().unwrap().to_string();
                dir.files.insert(file_name, file_size);
            }
        }
    }
    file_structure
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_structure = parse_input(input);
    let limit_size = 100000 as u32;
    let mut total_size = 0 as u32;
    let mut dirs_to_check = Vec::new();

    dirs_to_check.push(&file_structure);
    while let Some(dir) = dirs_to_check.pop() {
        let dir_size = dir.total_size();
        if dir_size <= limit_size {
            total_size += dir_size;
        }
        for subdir_name in dir.directories.keys() {
            dirs_to_check.push(dir.directories.get(subdir_name).unwrap());
        }
    }
    // sum the size of all files in the small directories
    Some(total_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_structure = parse_input(input);
    let disk_size = 70000000 as u32;
    let update_size = 30000000 as u32;
    let used_disk_size = file_structure.total_size();
    let free_disk_size = disk_size - used_disk_size;
    let space_to_free = update_size - free_disk_size;

    let mut dirs_to_check = Vec::new();
    let mut all_dirs = Vec::new();
    dirs_to_check.push(&file_structure);
    while let Some(dir) = dirs_to_check.pop() {
        all_dirs.push(dir);
        for subdir_name in dir.directories.keys() {
            dirs_to_check.push(dir.directories.get(subdir_name).unwrap());
        }
    }
    all_dirs.sort_by_key(|dir| dir.total_size());
    let mut dir_size = 0;
    for dir in all_dirs.iter() {
        dir_size = dir.total_size();
        if dir_size >= space_to_free {
            break;
        }
    }

    // sum the size of all files in the small directories
    Some(dir_size)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
