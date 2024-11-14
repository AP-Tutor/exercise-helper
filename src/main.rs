use std::env;
use std::io::Read;
use std::process::{exit, Command};
use std::sync::Arc;

use orchestrator::{default_memory::DefaultMemory, prelude::*};
use rust_default::prelude::*;
pub mod stateless_cli;

use stateless_cli::StatelessCLIPlugin;
use tokio::task::JoinSet;
use zip::ZipArchive;

GenerateState!(
    RustExercise2,
    GeneratedFiles2,
    RustCompiled2,
    ExerciseResult
);

fn from_zip()->Vec<(String, String)>{
    let zip = include_bytes!(concat!(env!("OUT_DIR"), "/exercises.zip"));
    let mut z  =ZipArchive::new(std::io::Cursor::new(zip)).unwrap();
    let names: Vec<String> = z.file_names().map(|x| x.to_string()).collect();
    names.iter().map(|name|  {
        let file = z.by_name_decrypt(name, "sheet1/ex06.rs".as_bytes()).unwrap();
        let read_file = file.bytes().collect::<Result<Vec<u8>, _>>().unwrap();
        let s = String::from_utf8(read_file).unwrap();
        (name.clone(), s)
    }).collect()
}
async fn add(o: Orchestrator<State>, to_add: Vec<(String, String)>)->Orchestrator<State>{
    let o = Arc::new(o);
    let mut set = JoinSet::new();
    to_add.into_iter().for_each(|(path, content)| {
        let o = o.clone();
        set.spawn(async move {
            let x = o.add_exercise::<RustExercise2>(&path, &content).await;
            if let Err(x) = x {
                panic!("Got an error while parsing {}: {}", path, x);
            }
            if env::var("DEBUG_MODE").is_ok(){
                println!("added: {}", path);
            }
            
        });
    });
    set.join_all().await;
    Arc::try_unwrap(o).map_err(|_|"impossible to retrive arc, this error should never come up").unwrap()
}

#[tokio::main]
async fn main() {
    let t = Command::new("cargo")
        .arg("+nightly")
        .arg("-V")
        .output()
        .unwrap();
    if !t.status.success() {
        println!("Impossible to execute cargo nightly. Be sure you have cargo installed with the nightly toolchain. \nIf you have installed it with rustup you could simply execute \"rustup toolchain install nightly\"");
        exit(-1);
    }

    let check_on = env::var("DEBUG_MODE").is_ok();
    let mut o: Orchestrator<State> = Orchestrator::new(2, check_on, DefaultMemory::init());
    //#[cfg(not(debug_assertions))]
    //let mut o: Orchestrator<State> = Orchestrator::new(5, false, DefaultMemory::init());

    // ADDING FUNCTIONALITY
    o.add_plugin(RustDefaultPlugin2::default().set_activate_default())
        .await
        .unwrap();
    o.add_plugin(StatelessCLIPlugin).await.unwrap();
    
    let to_add = from_zip();
    let o = add(o, to_add).await;
    #[cfg(debug_assertions)]
    println!("all exercise added: ");
    // execute all
    let _o = o.run().await;
}
