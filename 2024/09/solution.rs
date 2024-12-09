use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Content {
    Free,
    File(u64),
}

use Content::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Segment {
    length: u8,
    content: Content,
}

type Input = Vec<u8>;

fn parse_input(file_path: OsString) -> Input {
    fs::read_to_string(file_path)
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn part1(input: &Input) -> u64 {
    let mut checksum = 0u64;
    let mut block_position = 0u64;
    let mut end_file_index = ((input.len() - 1) / 2) * 2;
    let mut end_file_moved_length = 0;
    let mut end_file_total_length;
    let mut end_file_id;
    let mut end_file_remaining_length;
    for (file_id, chunk) in input.chunks(2).enumerate() {
        if file_id == end_file_index / 2 {
            checksum += (block_position
                ..(block_position + (chunk[0] - end_file_moved_length) as u64))
                .sum::<u64>()
                * file_id as u64;
            break;
        }
        checksum +=
            (block_position..(block_position + (chunk[0] as u64))).sum::<u64>() * file_id as u64;
        block_position += chunk[0] as u64;

        if chunk.len() < 2 {
            break;
        }
        let mut free_space_remaining_length = chunk[1];
        while free_space_remaining_length > 0 && end_file_index > file_id * 2 {
            end_file_total_length = input[end_file_index];
            end_file_id = end_file_index / 2;
            end_file_remaining_length = end_file_total_length - end_file_moved_length;
            if end_file_remaining_length > free_space_remaining_length {
                checksum += (block_position
                    ..(block_position + (free_space_remaining_length as u64)))
                    .sum::<u64>()
                    * end_file_id as u64;
                block_position += free_space_remaining_length as u64;
                end_file_moved_length += free_space_remaining_length;
                free_space_remaining_length = 0;
            } else {
                checksum += (block_position..(block_position + (end_file_remaining_length as u64)))
                    .sum::<u64>()
                    * end_file_id as u64;
                block_position += end_file_remaining_length as u64;
                end_file_index -= 2;
                end_file_moved_length = 0;
                free_space_remaining_length -= end_file_remaining_length;
            }
        }
    }
    checksum
}

fn part2(input: &Input) -> u64 {
    let mut segments: Vec<Segment> = input
        .chunks(2)
        .enumerate()
        .flat_map(|(id, chunks)| {
            if chunks.len() < 2 {
                vec![Segment {
                    length: chunks[0],
                    content: File(id as u64),
                }]
            } else {
                vec![
                    Segment {
                        length: chunks[0],
                        content: File(id as u64),
                    },
                    Segment {
                        length: chunks[1],
                        content: Free,
                    },
                ]
            }
        })
        .collect();
    let mut tail = vec![];
    let mut latest_id = u64::MAX;
    while let Some(moving_segment) = segments.pop() {
        if let File(moving_file_id) = moving_segment.content {
            if moving_file_id >= latest_id {
                tail.push(moving_segment);
                continue;
            }
            let insert_at = segments
                .iter()
                .enumerate()
                .find(|(_, seg)| match seg.content {
                    File(_) => false,
                    Free => seg.length >= moving_segment.length,
                });
            if let Some((index, &insert_at_segment)) = insert_at {
                segments.remove(index);
                tail.push(Segment {
                    length: moving_segment.length,
                    content: Free,
                });
                if insert_at_segment.length > moving_segment.length {
                    segments.insert(
                        index,
                        Segment {
                            length: insert_at_segment.length - moving_segment.length,
                            content: Free,
                        },
                    );
                }
                segments.insert(index, moving_segment);
            } else {
                tail.push(moving_segment);
            }
            latest_id = moving_file_id;
        } else {
            tail.push(moving_segment);
        }
    }
    tail.into_iter()
        .rev()
        .scan(0, |block_pos, segment| {
            let length = segment.length as u64;
            let checksum = match segment.content {
                Free => 0,
                File(id) => (length * (length - 1) / 2 + length * *block_pos) * id,
            };
            *block_pos += length;
            Some(checksum)
        })
        .sum()
}
