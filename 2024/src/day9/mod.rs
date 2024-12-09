pub const SOLUTION: common::Solution = common::Solution {
    name: "Day 9: Disk Fragmenter",
    input: std::include_bytes!("input"),
    solve: self::solve,
};

use std::iter;

use itertools::Itertools;

pub fn solve(input: &[u8]) -> (String, String) {
    let input = String::from_utf8_lossy(input);
    let input = input.as_ref();

    let mut disk_map: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).expect("input should only contain digits"))
        .collect();

    // Disk map need to have equal number of files and free space.
    if disk_map.len() % 2 != 0 {
        disk_map.push(0);
    }
    let disk_map = disk_map;

    // Part 1
    let mut disk: Vec<Option<u32>> = disk_map
        .iter()
        .copied()
        .tuples()
        .zip(0..)
        .flat_map(|((file_len, padding), file_id)| {
            iter::repeat_n(Some(file_id), file_len as usize)
                .chain(iter::repeat_n(None, padding as usize))
        })
        .collect();

    // for block in disk.iter() {
    //     match block {
    //         Some(id) => print!("{id}"),
    //         None => print!("."),
    //     }
    // }
    // println!();

    let mut left = 0;
    let mut right = disk.len();

    while left != right {
        // Seek for next free block to move to.
        if disk[left].is_some() {
            left += 1;
            continue;
        }

        // Seek for next used block to move.
        if disk[right - 1].is_none() {
            right -= 1;
            continue;
        }

        // Move block to free block.
        disk[left] = disk[right - 1];
        disk[right - 1] = None;
        right -= 1;
    }

    let part1: u64 = (0..)
        .zip(disk.iter().flatten())
        .map(|(index, id)| (index * id) as u64)
        .sum();

    // Part 2
    let mut files_table: Vec<File> = disk_map
        .iter()
        .copied()
        .tuples()
        .zip(0..)
        .map(|((len, padding), id)| File { id, len, padding })
        .collect();

    // dbg!(&files_table);
    // for file in files_table.iter() {
    //     for _ in 0..file.len {
    //         print!("{}", file.id);
    //     }
    //     for _ in 0..file.padding {
    //         print!(".");
    //     }
    // }
    // println!();

    let last_id = files_table.last().unwrap().id;

    // TODO: Refactor this solution. This solution ended up being such a mess
    // with the logic being really hard to follow.
    for id in (0..=last_id).rev() {
        let (offset, file) = files_table
            .iter()
            .find_position(|file| file.id == id)
            .unwrap();
        let (offset, mut file) = (offset, file.clone());

        let found = files_table
            .iter_mut()
            .find_position(|other| other.padding >= file.len);

        if let Some((other_offset, other_file)) = found {
            if other_offset < offset {
                let total_size = file.len + file.padding;
                file.padding = other_file.padding - file.len;
                other_file.padding = 0;

                if other_offset + 1 == offset {
                    file.padding += total_size;
                } else if let Some(f) = files_table.get_mut(offset - 1) {
                    f.padding += total_size;
                }

                files_table.remove(offset);
                files_table.insert(other_offset + 1, file);
            }
        }
    }

    let mut index = 0;
    let mut part2: u64 = 0;
    for file in files_table {
        part2 += (index..index + file.len)
            .map(|i| (i * file.id) as u64)
            .sum::<u64>();
        index += file.len + file.padding;
    }

    (part1.to_string(), part2.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    id: u32,
    len: u32,
    padding: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{example, solution};

    // Part 1
    example!(p1, p1_example_1, "2333133121414131402", "1928");
    example!(p1, p1_example_2, "12345", "60");
    solution!(p1, p1_solution, "6519155389266");

    // Part 2
    example!(p2, p2_example_1, "2333133121414131402", "2858");
    example!(p2, p2_example_2, "12345", "132");
    example!(p2, p2_example_3, "1010101010101010101010", "385");
    example!(p2, p2_example_4, "354631466260", "1325");
    example!(p2, p2_example_5, "12235", "103");
    solution!(p2, p2_solution, "6547228115826");
}
