use std::str::FromStr;

use regex::Regex;
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq)]
enum CubeSet {
    Empty,
    Single(Cube),
    Set(SmallVec<[Cube; 64]>),
}

impl CubeSet {
    fn volume(&self) -> usize {
        match self {
            CubeSet::Empty => 0,
            CubeSet::Single(cube) => cube.volume(),
            CubeSet::Set(cubes) => cubes.iter().map(Cube::volume).sum()
        }
    }


    fn _len(&self) -> usize {
        match self {
            CubeSet::Empty => 0usize,
            CubeSet::Single(_) => 1usize,
            CubeSet::Set(v) => v.len(),
        }
    }

    fn add(self, other: Cube) -> CubeSet {
        match self {
            CubeSet::Empty => CubeSet::Single(other),
            CubeSet::Single(cube) => cube.add(other),
            CubeSet::Set(_) => {
                match self.destroy(other.clone()) {
                    CubeSet::Empty => CubeSet::Single(other),
                    CubeSet::Single(cube) => CubeSet::Set(SmallVec::from_iter([cube, other])),
                    CubeSet::Set(mut cubes) => {
                        cubes.push(other);
                        CubeSet::Set(cubes)
                    }
                }
            }
        }
    }

    fn destroy(self, other: Cube) -> CubeSet {
        match self {
            CubeSet::Empty => CubeSet::Empty,
            CubeSet::Single(cube) => cube.destroy(other),
            CubeSet::Set(cubes) => {
                let mut new_cubes = cubes.clone();
                new_cubes.clear();
                for cube in cubes {
                    match cube.destroy(other.clone()) {
                        CubeSet::Empty => {}
                        CubeSet::Single(cube) => { new_cubes.push(cube); }
                        CubeSet::Set(cubes) => {
                            for cube in cubes {
                                new_cubes.push(cube);
                            }
                        }
                    }
                }
                CubeSet::Set(new_cubes)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Cube {
    x_min: i64,
    y_min: i64,
    z_min: i64,
    x_max: i64,
    y_max: i64,
    z_max: i64,
}

impl Cube {
    fn contains(&self, other: &Cube) -> bool {
        self.x_min <= other.x_min &&
            self.x_max >= other.x_max &&
            self.y_min <= other.y_min &&
            self.y_max >= other.y_max &&
            self.z_min <= other.z_min &&
            self.z_max >= other.z_max
    }

    fn disjoint(&self, other: &Cube) -> bool {
        self.x_max < other.x_min ||
            self.x_min > other.x_max ||
            self.y_max < other.y_min ||
            self.y_min > other.y_max ||
            self.z_max < other.z_min ||
            self.z_min > other.z_max
    }

    fn volume(&self) -> usize {
        ((self.x_max + 1 - self.x_min) *
            (self.y_max + 1 - self.y_min) *
            (self.z_max + 1 - self.z_min)) as usize
    }

    fn add(self, other: Cube) -> CubeSet {
        if self.contains(&other) {
            return CubeSet::Single(self);
        }

        if other.contains(&self) {
            return CubeSet::Single(other);
        }

        if self.disjoint(&other) {
            return CubeSet::Set(SmallVec::from_iter([self, other]));
        }

        let mut cubes = SmallVec::<[Cube; 64]>::new();

        let (x_possibilities, y_possibilities, z_possibilities) =
            self.get_zones_possibilities(&other);

        for &(x_min, x_max) in x_possibilities.iter() {
            if x_max < x_min {
                continue;
            }
            for &(y_min, y_max) in y_possibilities.iter() {
                if y_max < y_min {
                    continue;
                }
                if (self.x_max < x_min ||
                    self.x_min > x_max ||
                    self.y_max < y_min ||
                    self.y_min > y_max) &&
                    (other.x_max < x_min ||
                        other.x_min > x_max ||
                        other.y_max < y_min ||
                        other.y_min > y_max)
                {
                    continue;
                }
                for &(z_min, z_max) in z_possibilities.iter() {
                    if z_max < z_min {
                        continue;
                    }
                    if (self.x_max < x_min ||
                        self.x_min > x_max ||
                        self.y_max < y_min ||
                        self.y_min > y_max ||
                        self.z_max < z_max ||
                        self.z_min > z_max) &&
                        (other.x_max < x_min ||
                            other.x_min > x_max ||
                            other.y_max < y_min ||
                            other.y_min > y_max ||
                            other.z_max < z_max ||
                            other.z_min > z_max)
                    {
                        continue;
                    }

                    cubes.push(Cube {
                        x_min,
                        y_min,
                        z_min,
                        x_max,
                        y_max,
                        z_max,
                    });
                }
            }
        }

        CubeSet::Set(cubes)
    }

    fn get_zones_possibilities(&self, other: &Cube) -> ([(i64, i64); 3], [(i64, i64); 3], [(i64, i64); 3]) {
        let x_possibilities = [
            (self.x_min.min(other.x_min), self.x_min.max(other.x_min) - 1),
            (self.x_min.max(other.x_min), self.x_max.min(other.x_max)),
            (self.x_max.min(other.x_max) + 1, self.x_max.max(other.x_max))
        ];

        let y_possibilities = [
            (self.y_min.min(other.y_min), self.y_min.max(other.y_min) - 1),
            (self.y_min.max(other.y_min), self.y_max.min(other.y_max)),
            (self.y_max.min(other.y_max) + 1, self.y_max.max(other.y_max))
        ];

        let z_possibilities = [
            (self.z_min.min(other.z_min), self.z_min.max(other.z_min) - 1),
            (self.z_min.max(other.z_min), self.z_max.min(other.z_max)),
            (self.z_max.min(other.z_max) + 1, self.z_max.max(other.z_max))
        ];
        (x_possibilities, y_possibilities, z_possibilities)
    }

    fn destroy(self, other: Cube) -> CubeSet {
        if other.contains(&self) {
            return CubeSet::Empty;
        }

        if self.disjoint(&other) {
            return CubeSet::Single(self);
        }

        let mut cubes = SmallVec::<[Cube; 64]>::new();

        let (x_possibilities, y_possibilities, z_possibilities) =
            self.get_zones_possibilities(&other);

        for &(x_min, x_max) in x_possibilities.iter() {
            if x_max < x_min {
                continue;
            }
            for &(y_min, y_max) in y_possibilities.iter() {
                if y_max < y_min {
                    continue;
                }
                if self.x_max < x_min ||
                    self.x_min > x_max ||
                    self.y_max < y_min ||
                    self.y_min > y_max
                {
                    continue;
                }
                for &(z_min, z_max) in z_possibilities.iter() {
                    if z_max < z_min {
                        continue;
                    }
                    if (self.x_max < x_min ||
                        self.x_min > x_max ||
                        self.y_max < y_min ||
                        self.y_min > y_max ||
                        self.z_max < z_min ||
                        self.z_min > z_max) ||
                        (other.x_max >= x_max &&
                            other.x_min <= x_min &&
                            other.y_max >= y_max &&
                            other.y_min <= y_min &&
                            other.z_max >= z_max &&
                            other.z_min <= z_min)
                    {
                        continue;
                    }

                    cubes.push(Cube {
                        x_min,
                        y_min,
                        z_min,
                        x_max,
                        y_max,
                        z_max,
                    });
                }
            }
        }

        CubeSet::Set(cubes)
    }
}


fn compute_on_cubes(s: &str, filter_small_area: bool) -> usize {
    let reg = Regex::new(r"(on|off) x=(-?[0-9]+)\.\.(-?[0-9]+),y=(-?[0-9]+)\.\.(-?[0-9]+),z=(-?[0-9]+)\.\.(-?[0-9]+)").unwrap();

    let mut cubes_set = CubeSet::Empty;

    for line in s.lines() {
        let results = reg.captures(line).unwrap();

        let cube = Cube {
            x_min: i64::from_str(&results[2]).unwrap(),
            y_min: i64::from_str(&results[4]).unwrap(),
            z_min: i64::from_str(&results[6]).unwrap(),
            x_max: i64::from_str(&results[3]).unwrap(),
            y_max: i64::from_str(&results[5]).unwrap(),
            z_max: i64::from_str(&results[7]).unwrap(),
        };

        if filter_small_area {
            if cube.x_min < -50 || cube.x_max > 50 ||
                cube.y_min < -50 || cube.y_max > 50 ||
                cube.z_min < -50 || cube.z_max > 50 {
                continue;
            }
        }

        cubes_set = match &results[1] {
            "on" => cubes_set.add(cube),
            "off" => cubes_set.destroy(cube),
            _ => panic!()
        }
    }

    cubes_set.volume()
}

pub fn _p1(s: &'static str) -> usize {
    compute_on_cubes(s, true)
}

pub fn p1() -> usize {
    _p1(include_str!("j22.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    compute_on_cubes(s, false)
}

pub fn p2() -> usize {
    _p2(include_str!("j22.txt"))
}

#[cfg(test)]
mod j22_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(590784, _p1(include_str!("j22_test.txt")));
        assert_eq!(650099, _p1(include_str!("j22.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2758514936282235, _p2(include_str!("j22_test_p2.txt")));
        assert_eq!(1254011191104293, _p2(include_str!("j22.txt")));
    }
}