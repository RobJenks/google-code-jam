pub mod qualifier_1;
pub mod qualifier_2;

#[allow(dead_code)]
pub fn run_all() {
    let solutions : Vec<fn()> = vec![
        qualifier_1::run,
        qualifier_2::run
    ];

    solutions.iter().for_each(|x| x());
}
