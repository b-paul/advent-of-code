use std::collections::HashSet;

pub fn part_1(input: &str) -> impl std::fmt::Display {
    let mut set = HashSet::new();
    for nums in input.lines().map(|l| l.split(',').collect::<Vec<&str>>()) {
        set.insert((
            nums[0].parse::<isize>().unwrap(),
            nums[1].parse::<isize>().unwrap(),
            nums[2].parse::<isize>().unwrap(),
        ));
    }
    let mut visited = HashSet::new();
    let mut total = 0;
    for (x, y, z) in set.clone() {
        let target = (x + 1, y, z);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
        let target = (x - 1, y, z);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
        let target = (x, y + 1, z);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
        let target = (x, y - 1, z);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
        let target = (x, y, z + 1);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
        let target = (x, y, z - 1);
        if !visited.contains(&(target, (x, y, z))) {
            visited.insert(((x, y, z), target));
            if !set.contains(&target) {
                total += 1;
            }
        }
    }

    total.to_string()
}

#[test]
fn testp1() {
    assert_eq!(
        part_1(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
        ).to_string(),
        64.to_string()
    );
}

pub fn part_2(input: &str) -> impl std::fmt::Display {
    // do a floodfill around the outside of the object
    // first we need a bounding box which is 1 extra width than the furthest the thingos stretch
    let mut set = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for nums in input.lines().map(|l| l.split(',').collect::<Vec<&str>>()) {
        let x = nums[0].parse::<i32>().unwrap();
        let y = nums[1].parse::<i32>().unwrap();
        let z = nums[2].parse::<i32>().unwrap();

        set.insert((x, y, z));
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        max_z = max_z.max(z);
    }
    max_x += 1;
    max_y += 1;
    max_z += 1;

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push((-1, -1, -1));

    let mut surface_area = 0;

    while let Some((x, y, z)) = stack.pop() {
        const DIFFS: [(i32, i32, i32); 6] = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        for (dx, dy, dz) in DIFFS {
            let (x, y, z) = (x + dx, y + dy, z + dz);
            if x < -1 || x > max_x || y < -1 || y > max_y || z < -1 || z > max_z {
                continue;
            }
            let point = (x, y, z);
            if set.contains(&point) {
                surface_area += 1;
            } else if !visited.contains(&point) {
                stack.push(point);
                visited.insert(point);
            }
        }
    }

    surface_area.to_string()
}

#[test]
fn testp2() {
    assert_eq!(
        part_2(
            "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
        ).to_string(),
        58.to_string()
    );
}
