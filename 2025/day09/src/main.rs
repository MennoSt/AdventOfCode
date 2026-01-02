use lib::filereader;
use lib::utils;
use std::os::raw::c_char;
// use lib::utils::Coordinate;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct Coordinate {
    pub x: i128,
    pub y: i128,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct Area {
    coordinate1: Coordinate,
    coordinate2: Coordinate,
    area: i128,
}
static INPUT: &str = "../input/day09";

fn parse_data(input: &str) -> Vec<Coordinate> {
    filereader::_input(input)
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            Coordinate {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn area(a: &Coordinate, b: &Coordinate) -> i128 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn p1(input: &str) -> i128 {
    let coordinates = parse_data(input);
    coordinates
        .iter()
        .flat_map(|a| coordinates.iter().map(move |b| area(a, b)))
        .max()
        .unwrap_or(0)
}

fn p2(input: &str) -> i128 {
    let coordinates = parse_data(input);
    let mut areas: Vec<Area> = coordinates
        .iter()
        .flat_map(|a| {
            coordinates.iter().map(move |b| {
                let ar = area(a, b);
                Area {
                    coordinate1: a.clone(),
                    coordinate2: b.clone(),
                    area: ar,
                }
            })
        })
        .collect();

    areas.sort_by(|a, b| b.area.cmp(&a.area));

    for a in &areas {
        if is_area_valid(a, &coordinates) {
            return a.area;
        }
    }
    println!("{:?}", areas);

    0
}

fn corners_to_check(c1: &Coordinate, c2: &Coordinate) -> (Coordinate, Coordinate) {
    let mut coor1 = Coordinate { x: 0, y: 0 };
    let mut coor2 = Coordinate { x: 0, y: 0 };

    coor1.x = c1.x;
    coor1.y = c2.y;

    coor2.x = c2.x;
    coor2.y = c1.y;

    (coor1, coor2)
}

fn is_area_valid(area: &Area, coordinates: &Vec<Coordinate>) -> bool {
    let (c1, c2) = corners_to_check(&area.coordinate1, &area.coordinate2);

    let validc1 = is_coordinate_valid(coordinates, c1);
    let validc2 = is_coordinate_valid(coordinates, c2);

    validc1 && validc2
}

fn is_coordinate_valid(coordinates: &Vec<Coordinate>, coordinate_to_check: Coordinate) -> bool {
    let mut x_valid = false;
    let mut y_valid = false;

    let mut xmin = 9999999;
    let mut xmax = -1;

    let mut ymin = 9999999;
    let mut ymax = -1;

    for c in coordinates {
        if coordinate_to_check.y == c.y {
            if coordinate_to_check.x < xmin {
                xmin = c.x;
            }
            if coordinate_to_check.x > xmax {
                xmax = c.x;
            }
        }
    }

    for c in coordinates {
        if coordinate_to_check.x == c.x {
            if coordinate_to_check.y < ymin {
                ymin = c.y;
            }
            if coordinate_to_check.y > ymax {
                ymax = c.y;
            }
        }
    }

    if coordinate_to_check.x <= xmax && coordinate_to_check.x >= xmin {
        x_valid = true;
    }

    if coordinate_to_check.y <= ymax && coordinate_to_check.y >= ymin {
        y_valid = true;
    }

    x_valid && y_valid
}

fn main() {
    let start = Instant::now();

    parse_data(INPUT);
    let part1 = p1(INPUT);
    println!("{}", part1);
    assert_eq!(part1, 4781377701);
    // let part2 = p2(INPUT);

    // utils::answer((part1, 0), (part2, 0));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[cfg(test)]
    mod tests {
        use super::*;
        static INPUT_EXAMPLE: &str = "example";

        #[test]
        fn test1() {
            let p1 = p1(INPUT_EXAMPLE);
            assert_eq!(p1, 50);
        }

        #[test]
        fn test2() {
            let p2 = p2(INPUT_EXAMPLE);
            assert_eq!(p2, 24);
        }

        #[test]
        fn test3() {
            let area = Area {
                coordinate1: Coordinate { x: 1, y: 1 },
                coordinate2: Coordinate { x: 2, y: 3 },
                area: 2,
            };

            let corners = corners_to_check(&area.coordinate1, &area.coordinate2);
            assert_eq!(
                corners,
                (Coordinate { x: 1, y: 3 }, Coordinate { x: 2, y: 1 })
            );
        }

        #[test]
        fn test4() {
            let area = Area {
                coordinate1: Coordinate { x: 11, y: 1 },
                coordinate2: Coordinate { x: 2, y: 5 },
                area: 2,
            };

            let corners = corners_to_check(&area.coordinate1, &area.coordinate2);
            assert_eq!(
                corners,
                (Coordinate { x: 11, y: 5 }, Coordinate { x: 2, y: 1 })
            );
        }

        #[test]
        fn test5() {
            let area = Area {
                coordinate1: Coordinate { x: 1, y: 1 },
                coordinate2: Coordinate { x: 3, y: 2 },
                area: 2,
            };
            let coordinates = vec![
                Coordinate { x: 1, y: 1 },
                Coordinate { x: 1, y: 2 },
                Coordinate { x: 3, y: 1 },
                Coordinate { x: 3, y: 2 },
            ];

            let valid = is_area_valid(&area, &coordinates);
            assert_eq!(valid, true);
        }

        #[test]
        fn test6() {
            let coordinate = Coordinate {x:9, y:3};
            let coordinates = vec![
                Coordinate { x: 7, y: 1 },
                Coordinate { x: 11, y: 1 },
                Coordinate { x: 11, y: 7 },
                Coordinate { x: 9, y: 7 },
                Coordinate { x: 9, y: 5 },
                Coordinate { x: 2, y: 5 },
                Coordinate { x: 2, y: 3 },
                Coordinate { x: 7, y: 3 },
            ];

            let valid = is_coordinate_valid( &coordinates, coordinate);
            assert_eq!(valid, true);
        }
    }
}
