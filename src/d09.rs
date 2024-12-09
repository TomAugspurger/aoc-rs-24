/*

In the end, we'll have positions of integers.
These could *maybe* be run length encoded, but who cares.

I think we want a mapping from

009998111888
*/

// use std::{collections::HashMap, ops::Range};

// pub struct State {
//     block_ids: HashMap<>,
//     free: Vec<Range<u8>>,
// }

// use std::{collections::VecDeque, ops::Range};

use std::collections::VecDeque;

#[derive(Debug)]
struct Block {
    id: usize,
    size: u8,
    is_file: bool,
}

pub fn run(input: &str) -> u64 {
    let mut fs: VecDeque<Block> = VecDeque::with_capacity(input.len());
    let mut fs2: Vec<Block> = Vec::with_capacity(input.len());

    // let mut cursor: usize = 0;
    // this is totally broken when block_id > 9?

    for (i, char) in input.chars().enumerate() {
        let (id, is_file) = (i / 2, i % 2 == 0);
        let size = char.to_digit(10).unwrap() as u8;
        fs.push_back(Block { id, size, is_file });
    }

    let mut pending_block = Block {
        id: 0,
        size: 0,
        is_file: true,
    };
    while !fs.is_empty() {
        let mut block = fs.pop_front().unwrap();
        if block.is_file {
            // eprintln!("Noop   : id: {}, size: {}", block.id, block.size);
            fs2.push(block);
        } else {
            // We have an empty sector. Copy stuff over, starting with the pending block (if any)
            while block.size > 0 {
                let moving_block = if pending_block.size > 0 {
                    // copy what we can.
                    // If we still have stuff left over
                    Block {
                        id: pending_block.id,
                        size: pending_block.size,
                        is_file: true,
                    }
                } else {
                    let mut x = fs.pop_back().unwrap(); // this could err?
                    if !x.is_file {
                        x = fs.pop_back().unwrap(); // this could err?
                    }
                    x
                };

                if moving_block.size <= block.size {
                    // we can move everything
                    // eprintln!("Full   : id: {}, size: {}", moving_block.id, moving_block.size);
                    fs2.push(Block {
                        id: moving_block.id,
                        size: moving_block.size,
                        is_file: true,
                    });
                    block.size -= moving_block.size;
                    pending_block = Block { id: 0, size: 0, is_file: true};
                } else {
                    // move what we can. Stuff the rest in pending
                    // eprintln!("Partial: id: {}, size: {}/{}", moving_block.id, block.size, moving_block.size);
                    fs2.push(Block {
                        id: moving_block.id,
                        size: block.size,
                        is_file: true,
                    });
                    pending_block = Block {
                        id: moving_block.id,
                        size: moving_block.size - block.size,
                        is_file: true,
                    };
                    block.size = 0;
                }
            }
        }
    }

    if pending_block.size > 0 && fs2[fs2.len() - 1].id == pending_block.id {
        // we have a bit left. Do we *know* we have room? I think so, it should all be free till the end..
        let mut b = fs2.pop().unwrap();
        b.size += pending_block.size;
        fs2.push(b);
    } else if pending_block.size > 0 {
        fs2.push(pending_block);
    }

    let mut checksum = 0;
    let mut cursor:u64  = 0;

    for block in fs2 {
        if block.is_file {
            for i in cursor..cursor + block.size as u64 {
                checksum += i * block.id as u64
            }
            cursor += block.size as u64
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use crate::d09::run;

    const INPUT: &str = "2333133121414131402";
    #[test]
    fn test_example_1() {
        assert_eq!(run(INPUT), 1928);
    }
}
