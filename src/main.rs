use dip::{bevy::log::LogPlugin, prelude::*};
// use std::path::Path;
use cmd_lib::run_fun;

fn main() {
    App::new()
    .add_plugin(CliPlugin::<NoAsyncAction>::oneshot())
    .add_plugin(LogPlugin)
    .add_plugin(ActionPlugin)
    .add_system(handle_check)
    .add_system(handle_install)
    .run();
}

#[derive(CliPlugin, clap::Parser)]
#[clap(author, version, about="Detox tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}


#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)] // attribute macro
pub enum Action {
    #[clap(about = "Checks your React Native and Detox installation")]
    Check,

    #[clap(about = "Install an android emulator")]
    Install
}

fn handle_check(mut actions: EventReader<CheckAction>) {
    actions.iter().for_each(|_action| {
        println!("Checking!");

        match run_fun!(which node) {
            Ok(path) => {
                println!("Node installs at: {path}");

                match run_fun!(node -v) {
                    Ok(version) => { println!("Node version: {version}");}
                    Err(e) => {println!("Couldn't find Node version: {e}");}
                }
            }      
            Err(e) => {println!("Error: {e}")}
        }
    });
}

fn handle_install(mut actions: EventReader<InstallAction>) {
    println!("hello");
    actions.iter().for_each(|_action| {
        println!("Installing!");
    });
}