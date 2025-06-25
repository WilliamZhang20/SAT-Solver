mod dp_solver;
mod dpll_solver;

use dp_solver::dp_solve;
use dpll_solver::dpll_solve;
use dpll_solver::CNF; // reuse one CNF type (they must match)

fn main() {
    let cnf: CNF = vec![
        vec![1, -2, 3],
        vec![-1, 2],
        vec![-3],
    ];

    // Run DPLL
    let mut dpll_assignment = Vec::new();
    println!("Running DPLL...");
    if dpll_solve(cnf.clone(), &mut dpll_assignment) {
        println!("DPLL: SAT");
        println!("Assignment: {:?}", dpll_assignment);
    } else {
        println!("DPLL: UNSAT");
    }

    // Run DP
    let mut dp_assignment = Vec::new();
    println!("Running DP...");
    if dp_solve(cnf, &mut dp_assignment) {
        println!("DP: SAT");
        println!("Assignment: {:?}", dp_assignment);
    } else {
        println!("DP: UNSAT");
    }
}