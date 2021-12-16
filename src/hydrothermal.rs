use std::ops::{Add, Sub, Div};

#[derive(Debug)]
pub struct Map {
    pub overlappings : Vec<Vec<u32>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn normalize(&self) -> Point {
        (self.end - self.start).normalize()
    }
    fn norm(&self) -> f64 {
        (self.end - self.start).norm()
    }
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}
impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { 
            speed : self.normalize(),
            position : self.start - self.normalize(), //We start at one step before the start so that the first next() is x1,y2
            end_position: self.end,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x : i32,
    pub y : i32,
}
impl Point {
    fn normalize(&self) -> Point {
        Point{x: i32::signum(self.x), y: i32::signum(self.y)}
    }
    fn norm(&self) -> f64 {
        f64::sqrt( ((self.x).pow(2) + (self.y).pow(2)) as f64)
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}
impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point{ x: self.x / rhs as i32, y: self.y / rhs as i32}
    }
}
#[test]
fn norm_test() {
    let p = Point{x: 3, y:3};
    assert_eq!(p.normalize(), Point{x: 1, y: 1});
}

#[derive(Debug)]
pub struct LineIntoIterator { 
    speed: Point, //This is more or less a vector that is a normalized vector of the line
    position: Point,
    end_position: Point,
}

impl Iterator for LineIntoIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        let is_end = self.position != self.end_position;
        self.position = self.position + self.speed;
        is_end.then(|| self.position)
    }
}

impl Map {
    pub fn draw_points_that_cover_line(&mut self, line: Line) {
        for p in line.into_iter() {
            self.overlappings[p.x as usize ][p.y as usize] += 1;
        }
    }
    pub fn count_above(&self, number : u32) -> u32 {
        self.overlappings.clone().into_iter().flatten().filter(|x| *x >= number).count() as u32
    }
}
#[test]
fn day5_1_example() {
    let lines = vec![
        Line{start: Point{x:0,y:9}, end: Point{x:5,y:9}},
        Line{start: Point{x:8,y:0}, end: Point{x:0,y:8}},
        Line{start: Point{x:9,y:4}, end: Point{x:3,y:4}},
        Line{start: Point{x:2,y:2}, end: Point{x:2,y:1}},
        Line{start: Point{x:7,y:0}, end: Point{x:7,y:4}},
        Line{start: Point{x:6,y:4}, end: Point{x:2,y:0}},
        Line{start: Point{x:0,y:9}, end: Point{x:2,y:9}},
        Line{start: Point{x:3,y:4}, end: Point{x:1,y:4}},
        Line{start: Point{x:0,y:0}, end: Point{x:8,y:8}},
        Line{start: Point{x:5,y:5}, end: Point{x:8,y:2}}
    ];
    let mut map: Map = Map{overlappings: vec![vec![0;10];10] };
    for line in lines.into_iter().filter(|l| l.is_horizontal() || l.is_vertical()) {
        map.draw_points_that_cover_line(line);
    }
    let result = map.count_above(2);
    assert_eq!(result, 5);
}
#[test]
fn line_iterator_test() {
    let line : Line = Line{start: Point{x:0, y:0}, end: Point{x:2, y:2}};
    let line_vec : Vec<Point> = line.into_iter().collect();
    assert_eq!(line_vec, vec![Point{x: 0,y: 0},Point{x: 1,y: 1},Point{x:2,y:2}])
}
#[test]
fn line_iterator_negative_test() {
    let line : Line = Line{start: Point{x:607, y:899}, end: Point{x:607, y:786}};
    dbg!(line, line.normalize());
    dbg!(line.into_iter());
    let line_vec : Vec<Point> = line.into_iter().collect();
    assert_ne!(line_vec.into_iter().count(), 0);
}