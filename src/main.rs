use std::env;
use std::io;

mod connection;
mod task;
use connection::Connection;

use task::Task;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.contains(&"help".to_string()) {
        help_argument();
    } else if args.len() == 3 {
        let mut connections: Vec<Connection> = Connection::read_target(args[1].to_string());
        Connection::test_connection(&mut connections);
        Connection::list_connection(&mut connections);
        let mut tasks: Vec<Task> = Task::read_tasks(args[2].to_string());
        Task::list_task(&mut tasks);

        Connection::install_package_no_tty(&mut connections, &mut tasks);
    } else if args.len() == 2 {
        let stdin = io::stdin();
        println!("This reads the args");
        let mut connections: Vec<Connection> = Connection::read_target(args[1].to_string());

        loop {
            // clearscreen::clear().expect("Failed to clear screen");
            let mut option_menu: String = String::new();
            Connection::list_connection(&mut connections);
            println!(" ");
            println!(" - - - - MENU - - - -");
            println!("1. Test connections");
            println!("2. Install package");
            println!("3. Uninstall package");
            println!("4. Read package YAML");
            println!("0. Exit");
            println!(" ");

            println!("Option: ");
            let _ = stdin.read_line(&mut option_menu);
            if option_menu.trim() == "1".to_string() {
                Connection::test_connection(&mut connections);
            } else if option_menu.trim() == "2".to_string() {
                Connection::install_package(&mut connections);
            } else if option_menu.trim() == "3".to_string() {
                Connection::uninstall_package(&mut connections);
            } else if option_menu.trim() == "4".to_string() {
                let mut yaml_task_name = String::new();
                println!("Type the task YAML file name:");
                let _ = stdin.read_line(&mut yaml_task_name);
                let mut tasks: Vec<Task> = Task::read_tasks(yaml_task_name.trim().to_string());
                Connection::install_package_no_tty(&mut connections, &mut tasks);
            } else if option_menu.trim() == "0".to_string() {
                println!("Exiting...");
                break;
            }
        }
    } else {
        let stdin = io::stdin();
        let mut connections: Vec<Connection> = Vec::new();

        loop {
            // clearscreen::clear().expect("Failed to clear screen");
            let mut option_menu: String = String::new();
            Connection::list_connection(&mut connections);
            println!(" ");
            println!(" - - - - MENU - - - -");
            println!("1. Create connection");
            println!("2. List connections");
            println!("3. Test connections");
            println!("4. Install package");
            println!("5. Uninstall package");
            println!("6. Read package YAML");
            println!("0. Exit");
            println!(" ");

            println!("Option: ");
            let _ = stdin.read_line(&mut option_menu);
            if option_menu.trim() == "1".to_string() {
                let new_connection = Connection::create_connection();
                connections.push(new_connection);
            } else if option_menu.trim() == "2".to_string() {
                Connection::list_connection(&mut connections);
            } else if option_menu.trim() == "3".to_string() {
                Connection::test_connection(&mut connections);
            } else if option_menu.trim() == "4".to_string() {
                Connection::install_package(&mut connections);
            } else if option_menu.trim() == "5".to_string() {
                Connection::uninstall_package(&mut connections);
            } else if option_menu.trim() == "6".to_string() {
                let mut yaml_task_name = String::new();
                println!("Type the task YAML file name:");
                let _ = stdin.read_line(&mut yaml_task_name);
                let mut tasks: Vec<Task> = Task::read_tasks(yaml_task_name.trim().to_string());
                Connection::install_package_no_tty(&mut connections, &mut tasks);
            } else if option_menu.trim() == "0".to_string() {
                println!("Exiting...");
                break;
            }
        }
    }
}

fn help_argument() {
    println!("- - - -  NONSIBLE HELP - - - -");
    println!("There are 3 ways to use Nonsible: ");
    println!(" ");
    println!("With no arguments (0):");
    println!("This will open a menu, where you can add manually the targets, and install or remove packages");
    println!(" ");
    println!("With one argument (1):");
    println!("The argument will be a YAML file, that will have all the necesary data in arrays. In this repo exists one file called 'example-hosts.yaml' that is a good example.");
    println!("The connections will be scanned directly, and you will see a menu to install and remove packages");
    println!("E.g: nonsible example-hosts.yaml");
    println!(" ");
    println!("With two arguments (2):");
    println!("Ideal to run on a pipeline (Github Actions, Azure DevOps, etc...)");
    println!("The first argument will be the target YAML file, and the second argument is the list of jobs that they have to do");
    println!("E.g: nonsible example-hosts.yaml workflow");
}
