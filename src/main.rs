use dialoguer::Select;
use std::{
    path::{Path, PathBuf},
    process::Output,
};

fn main() {
    println!("Starting LaunchKit...\nLoaded.");

    ///// Getting Project Details

    println!("\nEnter a project name.");

    let mut project_name: String = String::new();

    std::io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to get project name.");

    println!("\nCreating {}...\n", project_name.trim());

    ///// Navigation to location

    // ~/Documents/Code/My_Sites
    // let navigation_output: Output = std::process::Command::new("cmd")
    //     .arg(format!("cd ~/Documents/Code/My_Sites"))
    //     .output()
    //     .expect("Failed to run execution command");

    let path: &Path = Path::new(r"C:\Users\SebCy\Documents\Code\My_Sites");
    std::env::set_current_dir(&path).expect("Failed to set current dir.");

    let current_dir: PathBuf = std::env::current_dir().expect("Failed to get cwd.");
    println!("CWD: {}\n", current_dir.display());

    ///// Creating Template

    let template_creation_command: String = format!(
        "npm create vite@latest {} -- --template react-ts",
        project_name
    );
    let template_creation_output: Output = std::process::Command::new("cmd")
        .arg(template_creation_command)
        .output()
        .expect("Failed to run execution command");

    println!(
        "Template creation stdout:\n{}\n",
        String::from_utf8_lossy(&template_creation_output.stdout)
    );
    println!(
        "Template creation stderr:\n{}\n",
        String::from_utf8_lossy(&template_creation_output.stderr)
    );

    ////// Packages Installation
    let new_template_path_string: String =
        format!(r"C:\Users\SebCy\Documents\Code\My_Sites\{}", project_name);

    println!("\nnew_template_path_string: {}\n", new_template_path_string);

    let new_template_path: &Path = Path::new(&new_template_path_string);
    std::env::set_current_dir(&new_template_path).expect("Failed to set current dir.");

    let new_current_dir: PathBuf = std::env::current_dir().expect("Failed to get cwd.");
    println!("New CWD: {}\n", new_current_dir.display());

    let installation_output: Output = std::process::Command::new("cmd")
        .arg("npm install")
        .output()
        .expect("Failed to run execution command");

    println!(
        "Installation stdout:\n{}\n",
        String::from_utf8_lossy(&installation_output.stdout)
    );
    println!(
        "Installation stderr:\n{}\n",
        String::from_utf8_lossy(&installation_output.stderr)
    );

    ////// General Output

    let output = std::process::Command::new("ls")
        .arg("-la")
        .output()
        .expect("Failed to run execution command");

    println!(
        "General Output stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "General Output stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    println!("\nComplete.\n");

    ///// Optional - Open Project

    // let open_now_options: Vec<&'static str> = vec!["Yes", "No"];

    // let selection: usize = Select::new()
    //     .with_prompt("Open Now?")
    //     .items(&open_now_options)
    //     .interact()
    //     .unwrap();

    // let selected_option = open_now_options[selection];
    // println!("You chose: {:?}", selected_option);

    // match selected_option {
    //     "Yes" => open_template(),
    //     "No" => println!("Closing LaunchKit."),
    //     _ => println!("Invalid selection"),
    // }
}

fn open_template() {}
