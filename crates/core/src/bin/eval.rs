//! Local eval runner - CLI entry point
//! Usage: cargo run -p idea-engine-core --bin eval -- --db path --recipe id --problems id1,id2

use idea_engine_core::eval::run_eval;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db_path = String::new();
    let mut recipe_id = String::new();
    let mut problem_ids: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db" => {
                i += 1;
                if i < args.len() {
                    db_path = args[i].clone();
                }
            }
            "--recipe" => {
                i += 1;
                if i < args.len() {
                    recipe_id = args[i].clone();
                }
            }
            "--problems" => {
                i += 1;
                if i < args.len() {
                    problem_ids = args[i].split(',').map(String::from).collect();
                }
            }
            _ => {}
        }
        i += 1;
    }

    if db_path.is_empty() || recipe_id.is_empty() {
        eprintln!("Usage: eval --db <path> --recipe <id> --problems <id1,id2,...>");
        std::process::exit(1);
    }

    match run_eval(&db_path, &recipe_id, &problem_ids) {
        Ok(reports) => {
            println!("Eval complete: {} reports", reports.len());
            for r in reports {
                println!("  {} {}: total={}", r.recipe_id, r.bundle_id, r.score_card.total);
            }
        }
        Err(e) => {
            eprintln!("Eval error: {}", e);
            std::process::exit(1);
        }
    }
}
