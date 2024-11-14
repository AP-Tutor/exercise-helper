use clap::{Parser, Subcommand};
use colored::Colorize;
use orchestrator::prelude::*;
use std::{error::Error, path::PathBuf, process::exit, sync::Arc};
use tokio::fs;

/// Simple program to exercitate for the Advanced programming course
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Submits an exercise
    Submit {
        /// the name of the exercise you are submitting
        exercise_name: String,

        /// your source code path
        file_path: PathBuf,
    },
    /// Explains exercise, and how it is structured
    Explain {
        /// the name of the exercise you want to know more
        exercise_name: String,
    },
}
fn pretty_tab(s: &str) -> String {
    let tmp = s.chars().flat_map(|x| {
        let t: Box<dyn Iterator<Item = char>> = if x == '\n' {
            Box::new("\n   ".chars())
        } else {
            Box::new(std::iter::once(x))
        };
        t
    });
    "   ".chars().chain(tmp).collect()
}
pub struct StatelessCLIPlugin;
impl<S: ExecutorGlobalState> Plugin<S> for StatelessCLIPlugin {
    fn name(&self) -> &str {
        "Stateless Cli"
    }

    fn desctiption(&self) -> &str {
        ""
    }

    async fn run(self, o: OrchestratorReference<S>, should_stop: Arc<Notify>) {
        let _ = o.memory().register("cli_plugin", "cli_plugin").await;
        let login = o.memory().login("cli_plugin", "cli_plugin").await.unwrap();
        let a = Args::parse();
        let names = o.memory().list_exercise_names().await.unwrap();
        match a.command {
            Commands::Submit {
                exercise_name,
                file_path,
            } => {
                if !names.contains(&exercise_name) {
                    println!("Exercise not found, choose from the following:");
                    for x in names {
                        println!("\t {x}");
                    }
                    exit(-1);
                }
                let test = async {
                    let file = fs::read(file_path).await?;
                    let file = String::from_utf8(file)?;
                    let result = o.process_exercise(exercise_name, file, login).await?;
                    Ok::<ExerciseResult, Box<dyn Error + Send + Sync>>(result)
                };
                match test.await {
                    Ok(v) => {
                        println!("Ok, got: {}", v);
                    }
                    Err(x) => {
                        println!("got error: {x}")
                    }
                }
            }
            Commands::Explain { exercise_name } => {
                if !names.contains(&exercise_name) {
                    println!("Exercise not found, choose from the following:");
                    for x in names {
                        println!("\t {x}");
                    }
                    exit(-1);
                }
                let t = o.get_exercise_info(exercise_name).await.unwrap();
                let description = t.description();
                let def = t.list();
                println!("{}\n{}", "Description:".green(), pretty_tab(description));
                println!("{} ", "Tests:".green());
                for t in def {
                    println!("    - {}({}): {}", t.name, t.points, t.description);
                }
            } //_ => {}
        }
        should_stop.notify_one();
    }
    async fn on_add<'a>(
        &'a mut self,
        _o: &'a mut Orchestrator<S>,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
}
