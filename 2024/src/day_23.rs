#![allow(unused_variables)]

use std::collections::{BTreeSet, HashSet};

fn parse(input: &str) -> (HashSet<usize>, Vec<&str>, Vec<Vec<bool>>) {
    let mut names = Vec::new();
    let mut conns: Vec<Vec<_>> = Vec::new();

    let mut dim = 0;

    for row in input.lines() {
        let (a, b) = row.split_once('-').unwrap();

        let a_idx = names.iter().position(|&s| s == a).unwrap_or_else(|| {
            names.push(a);
            names.len() - 1
        });
        let b_idx = names.iter().position(|&s| s == b).unwrap_or_else(|| {
            names.push(b);
            names.len() - 1
        });

        let max_a_b = a_idx.max(b_idx);
        if dim < max_a_b + 1 {
            dim = max_a_b + 1;
            for row in conns.iter_mut() {
                row.extend(vec![false; dim - row.len()]);
            }
            while conns.len() < dim {
                conns.push(vec![false; dim]);
            }
        }

        conns[a_idx][b_idx] = true;
        conns[b_idx][a_idx] = true;
    }

    let relevant_ids = names
        .iter()
        .enumerate()
        .filter_map(|(i, &name)| match name.starts_with('t') {
            true => Some(i),
            false => None,
        })
        .collect::<HashSet<_>>();
    (relevant_ids, names, conns)
}

pub fn part1(input: &str) -> String {
    let (relevant_ids, _, conns) = parse(input);

    let mut result = 0;

    for c1 in 0..conns.len() {
        for c2 in c1 + 1..conns.len() {
            for c3 in c2 + 1..conns.len() {
                if conns[c1][c2]
                    && conns[c2][c3]
                    && conns[c1][c3]
                    && (relevant_ids.contains(&c1)
                        || relevant_ids.contains(&c2)
                        || relevant_ids.contains(&c3))
                {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

fn find_clique(
    current: Vec<usize>,
    remaining: Vec<usize>,
    conns: &Vec<Vec<bool>>,
    visited: &mut HashSet<BTreeSet<usize>>,
) -> Vec<Vec<usize>> {
    let current_hash = BTreeSet::from_iter(current.clone());
    if visited.contains(&current_hash) {
        return Vec::new();
    } else {
        visited.insert(current_hash);
    }
    let remaining: Vec<usize> = remaining
        .into_iter()
        .filter(|&id| current.iter().all(|&cid| conns[cid][id]))
        .collect();

    if remaining.len() == 0 {
        return vec![current];
    }

    let mut branches = Vec::new();

    for rem_id in remaining.iter() {
        let mut new_current = current.clone();
        new_current.push(*rem_id);
        let mut new_remain = remaining.clone();
        new_remain.retain(|id| id != rem_id);
        branches.extend(find_clique(new_current, new_remain, conns, visited));
    }

    branches
}

pub fn part2(input: &str) -> String {
    let (relevant_ids, names, conns) = parse(input);

    let mut all_clusters = Vec::new();
    let mut visited = HashSet::new();
    for id in relevant_ids.iter() {
        all_clusters.extend(find_clique(
            vec![*id],
            Vec::from_iter(0..conns.len()),
            &conns,
            &mut visited,
        ));
    }

    let mut result = all_clusters
        .into_iter()
        .max_by_key(|x| x.len())
        .unwrap()
        .into_iter()
        .map(|id| names[id])
        .collect::<Vec<_>>();
    result.sort();

    result.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE1), "7");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE1), "co,de,ka,ta");
    }
}
