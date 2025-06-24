// davis putnam (DP) SAT Solver

pub type Literal = i32; // positive = true, negative = false

pub type Clause = Vec<Literal>;
pub type CNF = Vec<Clause>;

pub fn simplify(cnf: &CNF, literal: Literal) -> Option<CNF> {
    let mut new_cnf = Vec::new();
    for clause in cnf {
        if clause.contains(&literal) {
            continue; // clause satisfied
        }
        let new_clause: Clause = clause.iter()
            .cloned() // clone concrete value from references
            .filter(|&lit| lit != -literal) // filter negation of the literal from given ref
            .collect(); // collect into new vec
        if new_clause.is_empty() {
            return None; // empty = unsat
        }
        new_cnf.push(new_clause);
    }
    Some(new_cnf)
}

pub fn dp_solve(cnf: CNF, assignment: &mut Vec<Literal>) -> bool {
    if cnf.is_empty() {
        return true; // no clauses = sat under any interpretation
    }
    if cnf.iter().any(|clause| clause.is_empty()) {
        return false; // some clause unsat
    }

    let var = cnf[0][0].abs();

    for &val in &[var as Literal, -(var as Literal)] {
        if let Some(simplified) = simplify(&cnf, val) {
            assignment.push(val);
            if dp_solve(simplified, assignment) {
                return true;
            }
            assignment.pop();
        }
    }

    false
}