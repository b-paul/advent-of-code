use crate::helper::prelude::*;
use itertools::Itertools;
use std::collections::*;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let p = input.lines().map(read_point).collect_vec();

    p.iter()
        .cartesian_product(p.iter())
        .map(|(&a, &b)| Rect::new(a, b))
        .map(|r| r.area())
        .max()
        .unwrap()
}

#[test]
fn test() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    let output = 24;
    assert_eq!(part_2(input).to_string(), output.to_string());
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    let p = input.lines().map(read_point).collect_vec();
    let lines = p
        .iter()
        .cycle()
        .map_windows(|[&a, &b]| StraightLine::from_points(a, b).unwrap())
        .take(p.len())
        .collect_vec();
    let boundary = lines
        .iter()
        .flat_map(|l| l.points())
        .collect::<HashSet<_>>();

    // This computes the side of each line that the polygon is contained in.
    let dir = if lines[0].vertical() {
        Direction4::Left
    } else {
        Direction4::Down
    };
    let dirs = lines
        .iter()
        .tuple_windows()
        .fold((vec![dir], dir), |(mut v, d), (l1, l2)| {
            let d2 = match (l1.backwards() == l2.backwards(), l1.horizontal()) {
                (true, true) => d.clockwise(),
                (true, false) => d.anticlockwise(),
                (false, true) => d.anticlockwise(),
                (false, false) => d.clockwise(),
            };
            v.push(d2);

            (v, d2)
        })
        .0;
    let lines = lines.into_iter().zip(dirs).collect_vec();

    // Notice that any rectangle that is not contained in the polygon must still contain points on
    // the boundary of the polygon. Since it also contains points not in the polygon, it must
    // contain at least one point directly adjacent to the boundary outside the polygon. We compute
    // all of such points, then check each rectangle to find ones that do not contain any of these
    // points, and these are precisely the interior rectangles.
    let exterior = lines
        .iter()
        .flat_map(|(l, d)| {
            l.points()
                .into_iter()
                .map(|p| p.move_dir(d.opposite()).unwrap())
        })
        .filter(|p| !boundary.contains(p))
        .collect_vec();

    p.iter()
        .cartesian_product(p.iter())
        .map(|(&a, &b)| Rect::new(a, b))
        .sorted_by_key(|r| std::cmp::Reverse(r.area()))
        .filter(|&r| !exterior.iter().any(|&p| r.contains_point(p)))
        .map(|r| r.area())
        .next()
        .unwrap()
}
