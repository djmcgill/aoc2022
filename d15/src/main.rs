use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{Cursor, Read},
    time::Instant,
};

// const INPUT: &str = TEST;
// const TARGET_ROW: isize = 10;

const INPUT: &str = REAL;
const TARGET_ROW: isize = 2_000_000;

fn main() {
    // P1: for each sensor, we work out how its scan area overlaps the target row, making sure to subtract
    //     the beacon if applicable, and keep track of all those 1D areas, combining when possible
    let start = Instant::now();
    let mut range_set = DisjointRangeSet::default();
    for line in parse_input(INPUT) {
        let ((sx, sy), (bx, by)) = line;
        let dist_from_beacon = (bx - sx).abs() + (by - sy).abs();
        let dist_from_target = (sy - TARGET_ROW).abs();
        if dist_from_beacon >= dist_from_target {
            // could instead just keep a hashset of beacons we've seen so far
            if by == TARGET_ROW {
                // if the beacon is the only cell in the target row we can see, then we can't count
                // any cells out, otherwise we have two ranges: one to the left and one to the right
                // of the beacon
                if dist_from_beacon != dist_from_target {
                    let obscured_range_l = (sx - dist_from_beacon + dist_from_target, bx - 1);
                    let obscured_range_r = (bx + 1, sx + dist_from_beacon - dist_from_target);
                    if obscured_range_l.0 <= obscured_range_l.1 {
                        range_set.insert(obscured_range_l);
                    }
                    if obscured_range_r.0 <= obscured_range_r.1 {
                        range_set.insert(obscured_range_r);
                    }
                }
            } else {
                // if the beacon is 5 away, and we're on the target row, then (y-5,y+5) is the range
                // if the beacon is 5 away, and we're next to the target row, then (y-4,y+4)
                let obscured_range = (
                    sx - dist_from_beacon + dist_from_target,
                    sx + dist_from_beacon - dist_from_target,
                );
                range_set.insert(obscured_range);
            }
        }
    }
    // sum up the distinct ranges
    let mut p1 = 0;
    for (l, u) in &range_set.0 {
        p1 += u - l + 1;
    }
    let p1_end = Instant::now();

    // P2: first we shift everything 45 degrees so we have a bunch of overlapping squares
    //     then, we traverse left to right, at `xmax+1` for each square,
    //     while maintaining the list of "active" squares, and for each we
    //     see if the whole y range is covered by the relevant squares.
    //     x' =  x + y
    //     y' = -x + y
    let mut squares: Vec<(isize, isize)> = Vec::new();
    let mut left_edges: Vec<(isize, usize)> = Vec::new();
    let mut right_edges: Vec<(isize, usize)> = Vec::new();

    for line in parse_input(INPUT) {
        let ((sx, sy), (bx, by)) = line;
        let dist_from_beacon = (bx - sx).abs() + (by - sy).abs();
        let x_prime = sx + sy;
        let y_prime = -sx + sy;

        squares.push((y_prime, dist_from_beacon));
        left_edges.push((x_prime - dist_from_beacon, squares.len() - 1));
        right_edges.push((x_prime + dist_from_beacon, squares.len() - 1));
    }
    let p2_mid = Instant::now();
    // we go along the list of end edges, and maintain which squares we're currently
    // in the x-range of
    let mut active_xs = HashSet::new();
    let mut left_edges_ix = 0;
    left_edges.sort_by_key(|x| x.0);
    right_edges.sort_by_key(|x| x.0);

    let mut p2 = 0;
    for (x_prime, square_ix) in &right_edges {
        // we're only interested in looking 1 spare further on from an edge
        let target_x = x_prime + 1;

        // insert any starts we missed
        while left_edges_ix < left_edges.len() && left_edges[left_edges_ix].0 <= target_x {
            active_xs.insert(left_edges[left_edges_ix].1);
            left_edges_ix += 1;
        }
        // remove this end from consideration
        active_xs.remove(&square_ix);

        let mut disjoint_set = DisjointRangeSet::default();
        for x in &active_xs {
            let (y_prime, dist_from_beacon) = squares[*x];
            disjoint_set.insert((y_prime - dist_from_beacon, y_prime + dist_from_beacon));
        }
        // fortunately for us, the target is never on the edge or just looking for a gap
        // in the middle wouldn't work
        if disjoint_set.0.len() > 1 {
            // now we convert back into the original coord system
            //     x' =  x + y
            //     y' = -x + y
            let x_prime = target_x;
            let y_prime = disjoint_set.0[0].1 + 1;
            let y = (x_prime + y_prime) / 2;
            let x = y - y_prime;

            p2 = x as u64 * 4_000_000 + y as u64;
            break;
        }
    }
    let p2_end = Instant::now();

    println!("{} {}", p1, p2);
    // p1: 32.3µs
    // p2: 37.8µs
    println!("p1: {:?}", p1_end - start);
    println!("p2 parsing: {:?}", p2_mid - p1_end);
    println!("p2 thinking: {:?}", p2_end - p2_mid);
    println!("p2 total: {:?}", p2_end - p1_end);
}

fn parse_input(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    let mut cursor = Cursor::new(input.as_bytes());
    let mut vec = vec![];
    while cursor.position() < cursor.get_ref().len() as u64 {
        vec.push(parse_line(&mut cursor));
    }
    vec
}

fn parse_line(cursor: &mut Cursor<&[u8]>) -> ((isize, isize), (isize, isize)) {
    cursor.set_position(cursor.position() + 12);
    let sx = parse_isize(cursor);
    cursor.set_position(cursor.position() + 3);

    let sy = parse_isize(cursor);
    cursor.set_position(cursor.position() + 24);
    let bx = parse_isize(cursor);
    cursor.set_position(cursor.position() + 3);
    let by = parse_isize(cursor);
    ((sx, sy), (bx, by))
}
fn parse_isize(cursor: &mut Cursor<&[u8]>) -> isize {
    let mut sign = 1;
    let mut int = 0;
    let mut cs = vec![0];
    loop {
        cursor.read_exact(&mut cs).unwrap();
        let c = cs[0];
        if c == b'-' {
            sign *= -1
        } else if (b'0'..=b'9').contains(&c) {
            int *= 10;
            int += (c - b'0') as isize;
        } else {
            break;
        }
    }
    sign * int
}

const REAL: &str = include_str!("real.txt");
const TEST: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

// a sorted vec of inclusive (lower,upper) bounds that merge if overlapping or adjacent
#[derive(Debug, Default, Clone)]
struct DisjointRangeSet(Vec<(isize, isize)>);
impl DisjointRangeSet {
    fn insert(&mut self, (l, u): (isize, isize)) {
        debug_assert!(l <= u);
        let insertion_point = self.0.partition_point(|x| x.0 < l);

        let mut left_cursor = insertion_point;
        while left_cursor > 0 && self.0[left_cursor - 1].1 + 1 >= l {
            left_cursor -= 1;
        }

        let mut right_cursor = insertion_point;
        while right_cursor < self.0.len() && self.0[right_cursor].0 - 1 <= u {
            right_cursor += 1;
        }
        // todo: does it make more sense to insert first?
        let (l2, u2) = if left_cursor == insertion_point && right_cursor == insertion_point {
            (l, u)
        } else {
            (
                min(l, self.0[left_cursor].0),
                max(u, self.0[right_cursor - 1].1),
            )
        };

        // todo: calling remove in a loop is bad
        for _ in left_cursor..right_cursor {
            self.0.remove(left_cursor);
        }
        self.0.insert(left_cursor, (l2, u2));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let line = "Sensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=16, y=7: closest beacon is at x=15, y=3\n";
        assert_eq!(
            parse_input(line),
            vec![((9, 16), (10, 16)), ((16, 7), (15, 3)),]
        );
    }

    #[test]
    fn disjoint_range() {
        let mut v = DisjointRangeSet(vec![(0, 2), (16, 17), (19, 112)]);
        v.insert((5, 6));
        assert_eq!(v.0, vec![(0, 2), (5, 6), (16, 17), (19, 112)]);

        let mut v = DisjointRangeSet(vec![(0, 2), (6, 8), (10, 12)]);
        v.insert((1, 4));
        assert_eq!(v.0, vec![(0, 4), (6, 8), (10, 12)]);

        let mut v = DisjointRangeSet(vec![(0, 2), (5, 8), (10, 12)]);
        v.insert((3, 4));
        assert_eq!(v.0, vec![(0, 8), (10, 12)]);

        let mut v = DisjointRangeSet(vec![(5, 8), (10, 12)]);
        v.insert((0, 2));
        assert_eq!(v.0, vec![(0, 2), (5, 8), (10, 12)]);

        let mut v = DisjointRangeSet(vec![(5, 8), (10, 12)]);
        v.insert((0, 4));
        assert_eq!(v.0, vec![(0, 8), (10, 12)]);

        let mut v = DisjointRangeSet(vec![(5, 8), (10, 12)]);
        v.insert((0, 5));
        assert_eq!(v.0, vec![(0, 8), (10, 12)]);

        let mut v = DisjointRangeSet(vec![(5, 8), (11, 12)]);
        v.insert((0, 10));
        assert_eq!(v.0, vec![(0, 12)]);

        let mut v = DisjointRangeSet(vec![(5, 8), (9, 12)]);
        v.insert((0, 13));
        assert_eq!(v.0, vec![(0, 13)]);

        let mut v = DisjointRangeSet(vec![(0, 2), (5, 7), (9, 12)]);
        v.insert((14, 15));
        assert_eq!(v.0, vec![(0, 2), (5, 7), (9, 12), (14, 15)]);
    }

    #[test]
    fn disjoint_error_case() {
        let mut v = DisjointRangeSet(vec![(-1_247_727, 1_714_867), (3_897_333, 4_052_128)]);
        v.insert((-337_469, 548_225));
        assert_eq!(v.0, vec![(-1_247_727, 1_714_867,), (3_897_333, 4_052_128,)]);
    }
}
