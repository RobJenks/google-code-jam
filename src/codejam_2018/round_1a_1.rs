#[path = "../util/mod.rs"] mod util;

type Grid = Vec<Vec<bool>>;

pub fn run() {
    let mut input = util::Input::create(true, "./src/codejam_2018/input-r1a-1");
    let tests = input.get_line_as::<usize>();
    (0..tests)
        .for_each(|_| {
            println!("Test!");
        });
}