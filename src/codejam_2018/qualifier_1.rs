#[path = "../util/mod.rs"] mod util;

const CHG: u8 = 'C' as u8;
const SHT: u8 = 'S' as u8;

pub fn run() {
    let input = util::stdin_all();
    let mut it = input.iter();

    let cases = it.next().unwrap().parse::<i32>().unwrap();
    (0..cases)
        .for_each(|x| {
            let result = solve(it.next().unwrap());
            println!("Case #{}: {}", x+1, match result {
                Some(x) => format!("{}", x),
                None => "IMPOSSIBLE".to_string()
            });
        });
}

fn solve(case: &String) -> Option<i32> {
    let comp = case.split_whitespace().collect::<Vec<&str>>();
    let strength = comp[0].parse::<i32>().unwrap();
    let st = comp[1].to_string();

    let mut adj = 0;
    let mut bytes = st.as_bytes().to_owned();
    while eval(&bytes) > strength {

        // Index of latest CS pair that can be switched
        let ix = bytes.iter()
            .enumerate()
            .rposition(|(i, x)| *x == SHT && i != 0 && bytes[i-1] == CHG);

        // Swap latest eligible pair, which has highest potential impact on overall value
        match ix {
            None => return None,
            Some(x) => {
                bytes[x] = CHG;
                bytes[x-1] = SHT;
            }
        }

        adj += 1;
    }

    Some(adj)
}

// Evaluates string and returns the total value output
fn eval(st: &Vec<u8>) -> i32 {
    st.iter()
        .fold((1, 0), |(strength, dmg), x| {
            match *x {
                CHG => (strength * 2, dmg),
                SHT => (strength, dmg + strength),
                _ => panic!("Unknown input")
            }
        }).1
}