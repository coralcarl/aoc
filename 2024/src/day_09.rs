use aoclib::solution::Solution;

fn checksum(id: usize, start: usize, len: usize) -> usize {
    id * len * (2 * start + len - 1) / 2
}

pub fn part1(input: &str) -> Solution {
    let mut blocks: Vec<_> = input
        .trim_end()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .collect();
    let mut sum = 0;
    let mut pos = 0;
    let (mut i, mut j) = (0, blocks.len() - 1);

    while i <= j {
        if i % 2 == 0 {
            sum += checksum(i / 2, pos, blocks[i]);
            pos += blocks[i];
            i += 1;
        } else {
            if blocks[j] == blocks[i] {
                sum += checksum(j / 2, pos, blocks[j]);
                pos += blocks[j];
                i += 1;
                j -= 2;
            } else if blocks[j] < blocks[i] {
                sum += checksum(j / 2, pos, blocks[j]);
                pos += blocks[j];
                blocks[i] -= blocks[j];
                j -= 2;
            } else {
                sum += checksum(j / 2, pos, blocks[i]);
                pos += blocks[i];
                blocks[j] -= blocks[i];
                i += 1;
            }
        }
    }

    Solution::Usize(sum)
}

pub fn part2(input: &str) -> Solution {
    let mut files = Vec::with_capacity(100000);
    let mut blocks = Vec::with_capacity(99999);
    let mut pos = 0;
    for (id, len) in input
        .trim_end()
        .bytes()
        .map(|b| (b - b'0') as usize)
        .enumerate()
    {
        if id % 2 == 0 {
            files.push((id / 2, pos, len));
        } else {
            blocks.push((pos, len));
        }
        pos += len;
    }

    let mut sum = 0;

    'next: for (id, pos, len) in files.into_iter().rev() {
        for (pos2, len2) in blocks.iter_mut().take(id) {
            if len <= *len2 {
                sum += checksum(id, *pos2, len);
                *pos2 += len;
                *len2 -= len;
                continue 'next;
            }
        }
        sum += checksum(id, pos, len);
    }

    Solution::Usize(sum)
}
