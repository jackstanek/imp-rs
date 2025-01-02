use std::collections::HashMap;

use ast::{AExpr, BExpr};

use lalrpop_util::lalrpop_mod;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod ast;
lalrpop_mod!(grammar);

#[derive(Debug, Clone)]
enum Error {
    UninitVar,
}

type State<'prog> = HashMap<&'prog str, i64>;

fn eval_aexpr(aexpr: &AExpr, state: &State) -> std::result::Result<i64, Error> {
    match aexpr {
        AExpr::Val(val) => Ok(*val),
        AExpr::Var(var) => {
            if let Some(val) = state.get(var.as_str()) {
                Ok(*val)
            } else {
                Err(Error::UninitVar)
            }
        }
        AExpr::BinOp(bin_op, lhs, rhs) => {
            let lhs = eval_aexpr(&lhs, state)?;
            let rhs = eval_aexpr(&rhs, state)?;
            let op = bin_op.into_fn();
            Ok(op(lhs, rhs))
        }
        AExpr::Neg(aexpr) => {
            let res = eval_aexpr(&aexpr, state)?;
            Ok(-res)
        }
    }
}

fn eval_bexpr(bexpr: &BExpr, state: &State) -> std::result::Result<bool, Error> {
    match bexpr {
        BExpr::Bool(val) => Ok(*val),
        BExpr::And(lhs, rhs) => {
            let lhs = eval_bexpr(&lhs, state)?;
            let rhs = eval_bexpr(&rhs, state)?;
            Ok(lhs && rhs)
        }
        BExpr::Or(lhs, rhs) => {
            let lhs = eval_bexpr(&lhs, state)?;
            let rhs = eval_bexpr(&rhs, state)?;
            Ok(lhs || rhs)
        }
        BExpr::Not(expr) => {
            let val = eval_bexpr(&expr, state)?;
            Ok(!val)
        }
        BExpr::Cmp(cmp, lhs, rhs) => {
            let lhs = eval_aexpr(lhs, state)?;
            let rhs = eval_aexpr(rhs, state)?;
            Ok(cmp.compare(lhs, rhs))
        }
    }
}

fn eval_stmt<'prog>(
    stmt: &'prog ast::Stmt,
    state: &mut State<'prog>,
) -> std::result::Result<(), Error> {
    match stmt {
        ast::Stmt::Skip => Ok(()),
        ast::Stmt::Asgn(var, aexpr) => {
            let val = eval_aexpr(aexpr, &state)?;
            state.insert(&var, val);
            Ok(())
        }
        ast::Stmt::Seq(stmts) => {
            for stmt in stmts {
                eval_stmt(stmt, state)?;
            }
            Ok(())
        }
        ast::Stmt::If(if_, then_, else_) => {
            let test = eval_bexpr(if_, state)?;
            eval_stmt(if test { then_ } else { else_ }, state)
        }
        ast::Stmt::While(test, body) => loop {
            let test = eval_bexpr(test, state)?;
            if !test {
                break Ok(());
            }
            eval_stmt(body, state)?;
        },
    }
}

fn run_prog<'prog>(prog: &'prog ast::Stmt) -> std::result::Result<State<'prog>, Error> {
    let mut state = HashMap::new();
    for stmt in prog {
        eval_stmt(stmt, &mut state)?;
    }
    Ok(state)
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let parser = grammar::StmtParser::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let exec = parser.parse(&line);
                match exec {
                    Ok(parse) => {
                        let final_state = run_prog(&parse);
                        println!("{:?}", final_state)
                    }
                    Err(err) => {
                        println!("error: {}", err)
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
