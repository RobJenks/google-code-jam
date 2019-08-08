#[path = "../util/mod.rs"] mod util;

pub fn run() {
    let input = util::stdin_all();
    let mut it = input.iter();

    let cases = it.next().unwrap().parse::<i32>().unwrap();
    (0..cases)
        .for_each(|x| {
            it.next();  // Don't care about value count, we get it from the values themselves
            println!("Case #{}: {}", x + 1,
                     match solve(it.next().unwrap()) {
                         None => "OK".to_string(),
                         Some(res) => res.to_string()
                     }
            );
        });
}

// Returns index of the first element out-of-order following sort, or
// no value if the result list is completely ordered
fn solve(case: &String) -> Option<usize> {
    let list = case
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    unsorted_index(&tsort_fast(&list))
}

// Returns a t-sorted list (replaced by tsort_fast to handle large input set 2)
#[allow(dead_code)]
fn tsort(l: &Vec<i32>) -> Vec<i32> {
    let mut done = false;
    let mut list = l.clone();

    while !done {
        done = true;
        (0..l.len()-2)
            .for_each(|x| {
                if list[x] > list[x+2] {
                    done = false;

                    let tmp = list[x];
                    list[x] = list[x+2];
                    list[x+2] = tmp;
                }
            });
    }

    list
}


// Performs more efficient test required for part two; independently sort the two
// interleaved sets of data, then recombine and test whether the full list is sorted
fn tsort_fast(l: &Vec<i32>) -> Vec<i32> {
    // Not supported in code jam compiler version...
    //let mut v0 = l.iter().step_by(2).map(|x| *x).collect::<Vec<i32>>();
    //let mut v1 = l.iter().skip(1).step_by(2).map(|x| *x).collect::<Vec<i32>>();

    let mut v0: Vec<i32> = vec![];
    let mut v1: Vec<i32> = vec![];
    (0..l.len()).for_each(|x| {
        let val = *l.get(x).unwrap();
        if x % 2 == 0 { v0.push(val) } else { v1.push(val) };
    });

    v0.sort_unstable();
    v1.sort_unstable();

    let mut v: Vec<i32> = vec![];

    v0.iter()
        .zip(v1.iter())
        .for_each(|(x0, x1)| {
            v.push(*x0);
            v.push(*x1);
        });

    if v0.len() > v1.len() { v.push(*v0.last().unwrap()) }
    else if v1.len() > v0.len() { v.push(*v1.last().unwrap()) };

    v
}


// Returns the index at which a list is not sorted, or None if it full-sorted
fn unsorted_index(list: &Vec<i32>) -> Option<usize> {
    let ub = list.len() - 1;
    match list
        .iter()
        .enumerate()
        .take_while(|&(i, x)| i == ub || *x <= list[i+1])
        .last() {

        Some(x) => if x.0 == ub { None } else { Some(x.0 + 1) },
        None => Some(0)
    }
}
