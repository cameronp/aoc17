

#[derive(Debug, Copy, Clone)]
enum Step {
    East,
    North,
    West,
    South
}

pub fn run(part: u32, extra: u32) {
    if part == 1 {
        println!("Part 1");
        let answer = compute (extra as usize);
        println!("{:?}", answer)
    }
    else {
        println!("Part 2");
        println!("{:?}", "answer");
    }
}

fn compute(n: usize) -> usize {
    let range = range_of_n(n);
    let walk = walk_of_n(range);

    let new_walk: Vec<(Step,usize)> =
        walk
        .iter()
        .flat_map(|&steps| {
            let (step, count) = steps;
            duplicate(step.clone(), count)
        })
        .rev()
        .take(n - 1)
        .collect();

    let (_, (x,y)) = 
        new_walk
        .iter()
        .fold((1, (0,0)), |state, &s| {
            let (count, location) = state;
            let (dir, _) = s;
            let new_loc = step(location, dir);
            ((count + 1), new_loc)
        });

    (x.abs() + y.abs()) as usize
}

fn duplicate(step: Step, n: usize) -> Vec<(Step, usize)> {
    let mut v = Vec::new();
    for _i in 0..n {
        v.push((step, 1));
    }
    v
}


fn step(point: (isize, isize), step: Step) -> (isize, isize) {
    let (x,y) = point;
    let new_loc = match step {
        Step::East => (x + 1, y),
        Step::North => (x, y - 1),
        Step::West => (x - 1, y),
        Step::South => (x, y + 1),
    };
    new_loc
}

fn walk_of_n( n: usize) -> Vec<(Step, usize)> {
    let mut walk = Vec::new();
    for i in (1..n+1).rev() {
        walk.push((Step::North, i));
        walk.push((Step::East, 2 * i + 1));
        walk.push((Step::South, 2 * i));
        walk.push((Step::West, 2 * i));
        walk.push((Step::North, i));
    };

    walk.push((Step::East, 1));
    walk
}


fn range_of_n( n: usize ) -> usize {
   _range_of_n(n, 0) 
}

fn _range_of_n( n: usize, curr: usize ) -> usize {
   if east_of_n(curr) >= n {
       curr
   } else {
       _range_of_n(n, curr + 1)
   }
}



fn east_of_n(n: usize) -> usize {
   match n {
       0 => 1,
       n => east_of_n(n - 1) + 8*n -7,
   }
}


