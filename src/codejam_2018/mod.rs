pub mod qualifier_1;
pub mod qualifier_2;
pub mod qualifier_3;
pub mod round_1a_1;

#[allow(dead_code)]
pub fn run_all() {
    let solutions : Vec<fn()> = vec![
        qualifier_1::run,
        qualifier_2::run,
        qualifier_3::run,
        round_1a_1::run
    ];

    solutions.iter().for_each(|x| x());
}
