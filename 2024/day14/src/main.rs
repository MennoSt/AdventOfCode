use lib::filereader;
use lib::utils;
use std::time::Instant;
use regex::Regex;

static INPUT: &str = "../input/day14";

#[derive(Debug, Default, Clone, PartialEq)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32
}

struct Area {
    width : i32,
    height : i32
}

fn parse_data(file:&str) -> Vec<Robot>
{
    let mut robots = Vec::new();
    let content = filereader::_input(file);
    let re = Regex::new(r"-?\d+").unwrap();
    for line in content.lines() {
        let numbers: Vec<i32> = re
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect();
        let robot = Robot {px:numbers[0],py:numbers[1],vx:numbers[2],vy:numbers[3]};
        robots.push(robot);

    }
    
    robots
}

fn mutate_robot(robot:&mut Robot, area:&Area) {

    robot.px += robot.vx;
    robot.py += robot.vy;

    if robot.px >= area.width {
        robot.px -= area.width;
    }
    
    if robot.py >= area.height {
        robot.py -= area.height;
    }

    if robot.px  < 0 {
        robot.px += area.width;
    }
    
    if robot.py < 0 {
        robot.py += area.height;
    }

}

fn calculate_safety_factor(robots:&Vec<Robot>, area:&Area) -> i32 {
    let mut quadrants = vec![0,0,0,0];
    let splitx = area.width/2;
    let splity = area.height/2;

    for robot in robots {
        if robot.px < splitx && robot.py < splity {
            quadrants[0] += 1;
        } else if robot.px > splitx && robot.py < splity {
            quadrants[1] += 1;
        } else if robot.px  > splitx && robot.py > splity {
            quadrants[2] += 1;
        } else if robot.px < splitx && robot.py > splity {
            quadrants[3] += 1;
        }
    }
    
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]    
}

fn part1 (input:&str, area:Area) -> i32{
    let mut robots = parse_data(input);
    
    iterate_robots(&mut robots, area, 100)
}

fn part2 (input:&str, area:Area) -> i32{
    let mut robots = parse_data(input);
    
    iterate_robots(&mut robots, area, 100)
    
}

fn iterate_robots(robots: &mut Vec<Robot>, area: Area, iterations:i32) -> i32 {

    for _ in 0..iterations {
        for robot in &mut *robots{
            mutate_robot(robot, &area);
        }
    }
    println!("{:?}",robots);
    calculate_safety_factor(robots, &area)
}

fn main() {
    let start = Instant::now();
    let part1 = part1(INPUT, Area {width:101, height:103});
    println!("{}", part1);
    // utils::answer((part1(INPUT), 26005),(part2(INPUT), 105620095782547));
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;
   
    #[test]
    fn test1() {
        let area = Area { width:11, height:7};
        let mut robot = Robot {px:0, py:0, vx:4, vy:0};

        mutate_robot(&mut robot, &area);

        assert_eq!(robot, Robot {px:4, py:0, vx:4, vy:0});
    }

    #[test]
    fn test2() {
        let area = Area { width:11, height:7};
        let mut robot = Robot {px:10, py:5, vx:4, vy:5};

        mutate_robot(&mut robot, &area);

        assert_eq!(robot, Robot {px:3, py:3, vx:4, vy:5});
    }
 
     #[test]
    fn test3() {
        let area = Area { width:3, height:3};
        let robots = vec![Robot {px:0, py:0, vx:4, vy:5}, 
                                      Robot {px:0, py:0, vx:4, vy:6},
                                      Robot {px:1, py:0, vx:4, vy:6},
                                      Robot {px:1, py:1, vx:4, vy:6},
                                      Robot {px:0, py:2, vx:4, vy:6},
                                      Robot {px:2, py:2, vx:4, vy:6},
                                      Robot {px:2, py:0, vx:4, vy:6}];

        let safety_factor = calculate_safety_factor(&robots, &area);

        assert_eq!(safety_factor, 2);
    }

    #[test]
    fn test4() {
        let part1 = part1(TESTINPUT, Area{width:11, height:7});
        assert_eq!(part1, 12);
    }

    #[test]
    fn test5() {
        let area = Area { width:11, height:7};
        let mut robots = vec![Robot {px:2, py:4, vx:2, vy:-3}];

        iterate_robots(&mut robots, area,5);

        assert_eq!(robots,vec![Robot {px:1, py:3, vx:2, vy:-3}]);

    }

    #[test]
    fn test6() {
        let area = Area { width:11, height:7};
        let mut robots = vec![Robot {px:9, py:6, vx:-3, vy:-3}];

        iterate_robots(&mut robots, area,5);

        assert_eq!(robots,vec![Robot {px:5, py:5, vx:-3, vy:-3}]);

    }
 
}