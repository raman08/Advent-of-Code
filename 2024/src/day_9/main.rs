use aoc_2024::{get_file_path, read_file};
use itertools::Itertools;
use std::env;

enum FileSystem {
    FreeSpace { size: usize },
    File { id: usize, size: usize },
}

fn part_1(lines: &str) -> usize {
    let mut memory = lines
        .trim()
        .char_indices()
        .map(|(idx, ch)| {
            let ch = ch.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                let id = idx / 2;

                return FileSystem::File { id, size: ch };
            }

            FileSystem::FreeSpace { size: ch }
        })
        .collect_vec();

    let mut clean_memory = Vec::new();
    let mut write_idx = 0;

    while write_idx < memory.len() {
        let block = &memory[write_idx];

        match *block {
            FileSystem::File { id, size } => clean_memory.push(FileSystem::File { id, size }),

            FileSystem::FreeSpace { size } => {
                fill_freespace(&mut memory, size, write_idx, &mut clean_memory);
            }
        }

        write_idx += 1;
    }
    checksum(&clean_memory)
}

fn fill_freespace(
    memory: &mut Vec<FileSystem>,
    mut freesize: usize,
    write_idx: usize,
    clean_memory: &mut Vec<FileSystem>,
) {
    let mut read_idx = memory.len() - 1;

    while freesize > 0 && read_idx > write_idx {
        if let FileSystem::File { id, size: filesize } = memory[read_idx] {
            if filesize <= freesize {
                clean_memory.push(FileSystem::File { id, size: filesize });
                freesize -= filesize;
                memory.remove(read_idx);
                read_idx -= 1;
            } else {
                clean_memory.push(FileSystem::File { id, size: freesize });
                memory[read_idx] = FileSystem::File {
                    id,
                    size: filesize - freesize,
                };
                freesize = 0;
            }
        } else {
            read_idx -= 1;
        }
    }
}

fn checksum(memory: &Vec<FileSystem>) -> usize {
    let mut position = 0;
    let mut result = 0;

    for block in memory {
        match *block {
            FileSystem::FreeSpace { size } => position += size,
            FileSystem::File { id, size } => {
                for _ in 0..size {
                    result += id * position;
                    position += 1;
                }
            }
        }
    }

    result
}

fn part_2(lines: &str) -> usize {
    let mut memory = lines
        .trim()
        .char_indices()
        .map(|(idx, ch)| {
            let ch = ch.to_digit(10).unwrap() as usize;
            if idx % 2 == 0 {
                let id = idx / 2;

                return FileSystem::File { id, size: ch };
            }

            FileSystem::FreeSpace { size: ch }
        })
        .collect_vec();

    let mut i = memory.len() - 1;
    while i > 0 {
        if let FileSystem::File {
            id,
            size: file_size,
        } = memory[i]
        {
            let mut insert_idx = 0;

            loop {
                if let FileSystem::FreeSpace { size: free_space } = memory[insert_idx] {
                    if free_space > file_size {
                        memory[i] = FileSystem::FreeSpace { size: file_size };
                        memory[insert_idx] = FileSystem::File {
                            id,
                            size: file_size,
                        };
                        memory.insert(
                            insert_idx + 1,
                            FileSystem::FreeSpace {
                                size: free_space - file_size,
                            },
                        );
                        break;
                    }

                    if free_space == file_size {
                        memory[i] = FileSystem::FreeSpace { size: file_size };
                        memory[insert_idx] = FileSystem::File {
                            id,
                            size: file_size,
                        };
                        break;
                    }
                }

                if insert_idx == i {
                    break;
                }

                insert_idx += 1;
            }
        }
        i -= 1;
    }

    checksum(&memory)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_sample = args.contains(&String::from("--sample"));
    let path = get_file_path(9, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
