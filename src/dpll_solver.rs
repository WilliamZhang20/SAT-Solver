// DPLL - Davis Putnam Logeman-Loveland Algorithm
// Employs backtracking, and enables tracking of the satisfying model

pub type Literal = i32; // positive = true, negative = false
use std::collections::HashMap;
type Assignment = HashMap<Literal, bool>;

pub type Clause = Vec<Literal>;
pub type CNF = Vec<Clause>;

pub fn unit_resolution(cnf: &mut CNF, literal: Literal) {
    cnf.retain(|clause| !clause.contains(&literal)); // remove all clauses containing the unit literal
    for clause in cnf.iter_mut() {
        clause.retain(|x| x != &-literal); // remove all negations of the literal from clauses
    }
}

fn pick_unassigned_literal(cnf: &CNF, assignment: &Assignment) -> Option<Literal> {
    for clause in cnf {
        for &lit in clause {
            let var = lit.abs();
            if !assignment.contains_key(&var) {
                return Some(lit); // pick first unassigned literal
            }
        }
    }
    None
}

pub fn dpll_solve(mut cnf: CNF, assignment: &mut Vec<Literal>) -> bool {
    if cnf.is_empty() {
        return true; // no clauses = sat under any interpretation
    }
    if cnf.iter().any(|clause| clause.is_empty()) {
        return false; // if an empty clause exists, then UNSAT
    }
    // find unit literal
    if let Some(inner) = cnf.iter().find(|v| v.len() == 1) {
        let unit = inner[0];
        unit_resolution(&mut cnf, unit);
        assignment.push(unit);
        return dpll_solve(cnf, assignment);
    }

    let current_assignment: Assignment = assignment
        .iter()
        .map(|&lit| (lit.abs(), lit > 0)) // collect absolute values, esp if marked as false in CNF
        .collect();

    // pick an unassigned variable & recurse
    match pick_unassigned_literal(&cnf, &current_assignment) {
        Some(lit) => {
            // try assigning true
            let mut cnf_true = cnf.clone();
            let mut assigned_true = assignment.clone();
            assigned_true.push(lit);
            unit_resolution(&mut cnf_true, lit);
            if dpll_solve(cnf_true, &mut assigned_true) {
                *assignment = assigned_true;
                return true;
            }
            
            // backtrack to false
            let mut cnf_false = cnf;
            let mut assign_false = assignment.clone();
            assign_false.push(-lit);
            unit_resolution(&mut cnf_false, -lit);
            if dpll_solve(cnf_false, &mut assign_false) {
                *assignment = assign_false;
                return true;
            }

            return false; // neither work => UNSAT
        }
        None => true,
    }
}