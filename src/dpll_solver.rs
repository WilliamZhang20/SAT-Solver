// DPLL - Davis Putnam Logeman-Loveland Algorithm
// Employs backtracking, and enables tracking of the satisfying model

pub type Literal = i32; // positive = true, negative = false
use std::collections::{HashMap, HashSet};
type Assignment = HashMap<Literal, bool>;

pub type Clause = Vec<Literal>;
pub type CNF = Vec<Clause>;

// Represents a modification to the CNF (for backtracking)
#[derive(Debug)]
enum Change {
    ClauseRemoved(Clause),
    LiteralRemoved { clause_index: usize, literal: Literal },
}

fn pure_literal_elimination(cnf: &mut CNF, assignment: &mut Vec<Literal>) {
    let mut literal_counts: HashMap<Literal, i32> = HashMap::new();

    for clause in cnf.iter() {
        for &lit in clause {
            *literal_counts.entry(lit).or_insert(0) += 1;
        }
    }

    let mut assigned = HashSet::new();

    for &lit in literal_counts.keys() {
        let var = lit.abs();
        if assigned.contains(&var) {
            continue;
        }

        let pos = literal_counts.get(&var).unwrap_or(&0);
        let neg = literal_counts.get(&-var).unwrap_or(&0);

        if *pos > 0 && *neg == 0 {
            assigned.insert(var);
            assignment.push(var);
            cnf.retain(|clause| !clause.contains(&var));
        } else if *neg > 0 && *pos == 0 {
            assigned.insert(var);
            assignment.push(-var);
            cnf.retain(|clause| !clause.contains(&-var));
        }
    }
}

// unit propagation = forcing unit resolution as much as possible
fn unit_propagate(cnf: &mut CNF, literal: Literal) -> Vec<Change> {
    let mut changes = Vec::new();

    let mut i = 0;
    while i < cnf.len() {
        if cnf[i].contains(&literal) {
            let clause = cnf.remove(i);
            changes.push(Change::ClauseRemoved(clause));
        } else {
            if cnf[i].contains(&-literal) {
                cnf[i].retain(|&x| x != -literal);
                changes.push(Change::LiteralRemoved {
                    clause_index: i,
                    literal: -literal,
                });

                if cnf[i].is_empty() {
                    // Will be caught by DPLL as unsat base case
                }
            }
            i += 1;
        }
    }

    changes
}

// Undo all tracked changes to CNF
fn undo_changes(cnf: &mut CNF, changes: Vec<Change>) {
    for change in changes.into_iter().rev() {
        match change {
            Change::ClauseRemoved(clause) => cnf.push(clause),
            Change::LiteralRemoved {
                clause_index,
                literal,
            } => {
                if clause_index < cnf.len() {
                    cnf[clause_index].push(literal);
                }
            }
        }
    }
}

fn pick_unassigned_literal(cnf: &CNF, assignment: &Assignment) -> Option<Literal> {
    let mut literal_counts: HashMap<Literal, usize> = HashMap::new();

    for clause in cnf {
        for &lit in clause {
            let var = lit.abs();
            if !assignment.contains_key(&var) {
                *literal_counts.entry(lit).or_insert(0) += 1;
            }
        }
    }

    // Return the literal with highest count
    literal_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(lit, _)| lit)
}

pub fn dpll_solve(cnf: &mut CNF, assignment: &mut Vec<Literal>) -> bool {
    if cnf.is_empty() {
        return true; // no clauses = sat under any interpretation
    }
    if cnf.iter().any(|clause| clause.is_empty()) {
        return false; // if an empty clause exists, then UNSAT
    }

    // Exhaust unit propagation
    loop {
        let unit = cnf.iter().find(|c| c.len() == 1).map(|c| c[0]);
        if let Some(unit_lit) = unit {
            let changes = unit_propagate(cnf, unit_lit);
            assignment.push(unit_lit);
            if !dpll_solve(cnf, assignment) {
                undo_changes(cnf, changes);
                assignment.pop();
                return false;
            }
            return true;
        } else {
            break;
        }
    }

    // Run pure literal elimination
    pure_literal_elimination(cnf, assignment);

    // Build current assignment map
    let current_assignment: HashMap<i32, bool> = assignment
        .iter()
        .map(|&lit| (lit.abs(), lit > 0))
        .collect();

    // Branch on unassigned literal
    if let Some(lit) = pick_unassigned_literal(cnf, &current_assignment) {
        // Try true
        let changes_true = unit_propagate(cnf, lit);
        assignment.push(lit);
        if dpll_solve(cnf, assignment) {
            return true;
        }
        undo_changes(cnf, changes_true);
        assignment.pop();

        // Try false
        let changes_false = unit_propagate(cnf, -lit);
        assignment.push(-lit);
        if dpll_solve(cnf, assignment) {
            return true;
        }
        undo_changes(cnf, changes_false);
        assignment.pop();
    }

    false
}