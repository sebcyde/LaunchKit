// use dialoguer::Select;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn stream_command(cmd: &str) {
    let mut child = Command::new("cmd")
        .args(["/C", cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_handle = std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
    });

    let stderr_handle = std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("{}", line);
            }
        }
    });

    child.wait().expect("Failed to wait on child");
    stdout_handle.join().unwrap();
    stderr_handle.join().unwrap();
}

fn main() {
    println!("Starting LaunchKit...\nLoaded.");
    ///// Getting Project Details
    println!("\nEnter a project name:");

    let mut project_name: String = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to get project name.");
    let project_name: &str = project_name.trim(); // trim newline

    println!("\nCreating {}...\n", project_name);

    // Set working directory
    let base_path: &Path = Path::new(r"C:\Users\SebCy\Documents\Code\My_Sites");
    std::env::set_current_dir(base_path).expect("Failed to set current dir.");

    let current_dir: PathBuf = std::env::current_dir().expect("Failed to get cwd.");
    println!("Working directory: {}\n", current_dir.display());

    // Run Vite template creation
    let template_command: String = format!(
        "npm create vite@latest {} -- --template react-ts",
        project_name
    );
    println!("Running: {}\n", template_command);
    stream_command(&template_command);

    // Change to new project dir
    let new_project_path: PathBuf = base_path.join(project_name);
    std::env::set_current_dir(&new_project_path).expect("Failed to set project dir.");

    let new_cwd: PathBuf = std::env::current_dir().expect("Failed to get cwd.");
    println!("\nInstalling dependencies at: {}\n", new_cwd.display());

    // Install dependencies
    stream_command("npm install");

    // Directory listing
    println!("\nProject directory contents:\n");
    stream_command("dir");

    println!("\n✅ Complete.\n");

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
