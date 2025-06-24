mod dp_solver;

use dp_solver::{CNF, dp_solve};

fn main() {
    let cnf: CNF = vec![
        vec![1, -2, 3],
        vec![-1, 2],
        vec![-3],
    ];

    let mut assignment = Vec::new();
    if dp_solve(cnf, &mut assignment) {
        println!("SAT");
        println!("Assignment: {:?}", assignment);
    } else {
        println!("UNSAT");
    }
}