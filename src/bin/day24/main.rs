const ACTUAL_INPUT: &str = include_str!("../../../actual_inputs/2023/24/input.txt");

#[derive(Debug, Clone, Copy)]
struct Vec3f {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3f {
    fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn dot(&self, other: &Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn drop_z(&self) -> Vec3f {
        Vec3f {
            x: self.x,
            y: self.y,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Vec3f,
    vel: Vec3f,
}

impl Hailstone {
    fn intersects_p1(&self, other: &Hailstone, test_area: &(f64, f64)) -> bool {
        fn mxc(hailstone: &Hailstone) -> (f64, f64) {
            let m = hailstone.vel.y as f64 / hailstone.vel.x as f64;
            let c = hailstone.pos.y as f64 - m * hailstone.pos.x as f64;
            (m, c)
        }
        let mxc1 = mxc(self);
        let mxc2 = mxc(other);

        if (mxc1.0 - mxc2.0).abs() < 0.001 {
            false
        } else {
            let det = -mxc1.0 + mxc2.0;
            let x = (mxc1.1 - mxc2.1) / det;
            let y = (mxc1.1 * mxc2.0 - mxc2.1 * mxc1.0) / det;

            let intersects =
                x >= test_area.0 && x <= test_area.1 && y >= test_area.0 && y <= test_area.1;

            if !intersects {
                false
            } else {
                let intersect_pos = Vec3f { x, y, z: 0.0 };
                let inter1 = intersect_pos.sub(&self.pos.drop_z());
                let inter2 = intersect_pos.sub(&other.pos.drop_z());

                let dot1 = self.vel.drop_z().dot(&inter1);
                let dot2 = other.vel.drop_z().dot(&inter2);

                dot1 > 0.0 && dot2 > 0.0
            }
        }
    }
}

fn parse(input: &str) -> Vec<Hailstone> {
    fn parse_vec(input: &str) -> Vec3f {
        let mut iter = input
            .trim()
            .split(',')
            .map(|x| x.trim().parse::<f64>().unwrap());
        Vec3f {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        }
    }

    input
        .trim()
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            Hailstone {
                pos: parse_vec(pos),
                vel: parse_vec(vel),
            }
        })
        .collect()
}

fn solve_p1(input: &str, test_area: (f64, f64)) -> String {
    let input = parse(input);

    (0..input.len())
        .map(|first| {
            ((first + 1)..input.len())
                .filter(|second| {
                    let first = input[first];
                    let second = input[*second];

                    first.intersects_p1(&second, &test_area)
                })
                .count()
        })
        .sum::<usize>()
        .to_string()
}

fn p1(input: &str) -> String {
    solve_p1(input, (200000000000000.0, 400000000000000.0))
}

fn p2(input: &str) -> String {
    let _input = input.trim();
    "".to_string()
}

fn main() {
    println!("{}", p1(ACTUAL_INPUT));
    println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(solve_p1(SAMPLE_INPUT, (7.0, 27.0)), "2");
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), "27732");
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), "");
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), "");
    }
}
