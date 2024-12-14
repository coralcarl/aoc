#![allow(unused_variables)]

pub fn part1(input: &str) -> u64 {
    let files = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .enumerate()
        .map(|(i, v)| (i / 2, v))
        .collect::<Vec<(usize, u64)>>();
    let mut files_it = files.iter();

    let mut checksum = 0;
    let mut id = 0;

    let (mut blk_back, mut size_back) = files_it.next_back().unwrap();

    let mut even = true;

    'outer: while let Some((blk, size)) = files_it.next() {
        if even {
            for _ in id..id + size {
                checksum += id * *blk as u64;
                id += 1;
            }
        } else {
            for _ in id..id + size {
                if size_back == 0 {
                    if files_it.next_back().is_none() {
                        break 'outer;
                    }
                    (blk_back, size_back) = match files_it.next_back() {
                        Some(value) => *value,
                        None => {
                            break 'outer;
                        }
                    };
                }

                checksum += id * blk_back as u64;
                size_back -= 1;
                id += 1;
            }
        }
        even ^= true;
    }

    for id in id..id + size_back {
        checksum += id * blk_back as u64;
    }

    checksum
}

pub fn part2(input: &str) -> u64 {
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    let mut blk_offset = 0;

    let mut even = true;
    for (blk_id, c) in input.trim().chars().enumerate() {
        let blk_size = c.to_digit(10).unwrap() as u64;
        if even {
            files.push((blk_offset, blk_size, blk_id / 2));
        } else {
            spaces.push((blk_offset, blk_size));
        }
        blk_offset += blk_size;
        even ^= true;
    }

    let mut files_it = files.iter().rev();

    let mut checksum = 0;

    while let Some(&(blk_offset, blk_size, blk_id)) = files_it.next() {
        let mut moved = false;
        for (space_offset, space_size) in spaces.iter_mut() {
            if *space_offset > blk_offset {
                break;
            }
            if *space_size >= blk_size {
                for file_id in *space_offset..*space_offset + blk_size {
                    checksum += file_id * blk_id as u64;
                }
                *space_size -= blk_size;
                *space_offset += blk_size;

                moved = true;
                break;
            }
        }
        if !moved {
            for file_id in blk_offset..blk_offset + blk_size {
                checksum += file_id * blk_id as u64;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "2333133121414131402";
        assert_eq!(part1(&input), 1928);
        assert_eq!(part2(&input), 2858);
    }
}
