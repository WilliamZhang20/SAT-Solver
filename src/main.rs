mod dp_solver;
mod dpll_solver;

use std::time::Instant;

use dp_solver::dp_solve;
use dpll_solver::dpll_solve;
use dpll_solver::CNF; // reuse one CNF type (they must match)

fn main() {
    let mut cnf: CNF = vec![
        vec![1, -2, 3],
        vec![-1, 2],
        vec![-3],
    ];
    
    let cnf_copy = cnf.clone();

    // Run DPLL
    let mut dpll_assignment = Vec::new();
    println!("Running DPLL...");
    // let start = Instant::now();
    if dpll_solve(&mut cnf, &mut dpll_assignment) {
        println!("DPLL: SAT");
        println!("Assignment: {:?}", dpll_assignment);
    } else {
        println!("DPLL: UNSAT");
    }
    // let duration = start.elapsed();
    // println!("Elapsed time: {:?}", duration);

    // Run DP
    let mut dp_assignment = Vec::new();
    println!("Running DP...");
    if dp_solve(cnf_copy, &mut dp_assignment) {
        println!("DP: SAT");
        println!("Assignment: {:?}", dp_assignment);
    } else {
        println!("DP: UNSAT");
    }
}