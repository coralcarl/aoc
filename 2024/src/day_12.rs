#![allow(unused_variables)]

fn neighbors(x: usize, y: usize) -> [(usize, usize); 4] {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .map(|(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
}

type Garden = Vec<Vec<char>>;
type Region = Vec<Vec<bool>>;

fn connected_plots(label: char, garden: &mut Garden, region: &mut Region, x: usize, y: usize) {
    region[y][x] = true;
    garden[y][x] = '.';

    for (x_next, y_next) in neighbors(x, y) {
        if y_next < garden.len() && x_next < garden.len() && garden[y_next][x_next] == label {
            connected_plots(label, garden, region, x_next, y_next);
        }
    }
}

fn generate_regions(input: &str) -> Vec<Region> {
    let mut garden: Garden = input.lines().map(|line| line.chars().collect()).collect();
    let mut regions = Vec::new();

    for y in 0..garden.len() {
        for x in 0..garden.len() {
            if garden[y][x] != '.' {
                let mut region = vec![vec![false; garden.len()]; garden.len()];
                connected_plots(garden[y][x], &mut garden, &mut region, x, y);
                regions.push(region);
            }
        }
    }

    regions
}

pub fn part1(input: &str) -> String {
    let regions = generate_regions(input);
    let mut price = 0;

    for region in &regions {
        let mut perimiter = 0;
        let mut area = 0;

        for y in 0..region.len() {
            for x in 0..region[0].len() {
                if region[y][x] {
                    for (nx, ny) in neighbors(x, y) {
                        if ny >= region.len() || nx >= region[0].len() || !region[ny][nx] {
                            perimiter += 1;
                        }
                    }
                    area += 1;
                }
            }
        }
        price += perimiter * area;
    }

    price.to_string()
}

pub fn part2(input: &str) -> String {
    let regions = generate_regions(input);
    let mut price = 0;

    for region in &regions {
        let mut sides = 0;
        let mut area = 0;

        for i in 0..region.len() {
            let mut is_fence_up = false;
            let mut is_fence_down = false;
            let mut is_fence_left = false;
            let mut is_fence_right = false;

            for j in 0..region.len() {
                if !region[i][j] {
                    is_fence_up = false;
                    is_fence_down = false;
                } else {
                    if i > 0 && region[i - 1][j] {
                        is_fence_up = false;
                    } else if !is_fence_up {
                        is_fence_up = true;
                        sides += 1;
                    }

                    if i + 1 < region.len() && region[i + 1][j] {
                        is_fence_down = false;
                    } else if !is_fence_down {
                        is_fence_down = true;
                        sides += 1;
                    }
                    area += 1;
                }

                if !region[j][i] {
                    is_fence_left = false;
                    is_fence_right = false;
                } else {
                    if i > 0 && region[j][i - 1] {
                        is_fence_left = false;
                    } else if !is_fence_left {
                        is_fence_left = true;
                        sides += 1;
                    }

                    if i + 1 < region[0].len() && region[j][i + 1] {
                        is_fence_right = false;
                    } else if !is_fence_right {
                        is_fence_right = true;
                        sides += 1;
                    }
                }
            }
        }
        price += sides * area;
    }

    price.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(&input), "140");
        assert_eq!(part2(&input), "80");
    }
}
