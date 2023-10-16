use prettytable::{Cell, Row, Table};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub task: TaskType,
    pub command: String,
    pub package: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskType {
    Run,
    Install,
    Uninstall,
    UpdateAll,
    UpgradeAll,
    CopyToRemote,
    Nothing,
}

impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Run => write!(f, "Run"),
            TaskType::Install => write!(f, "Install"),
            TaskType::Uninstall => write!(f, "Uninstall"),
            TaskType::UpdateAll => write!(f, "UpdateAll"),
            TaskType::UpgradeAll => write!(f, "UpgradeAll"),
            TaskType::CopyToRemote => write!(f, "CopyToRemote"),
            TaskType::Nothing => write!(f, "Nothing"),
        }
    }
}

impl Task {
    pub fn list_task(tasks: &mut Vec<Task>) {
        let mut table = Table::new();

        table.add_row(prettytable::row![H3cb -> "TASKS"]);
        table.add_row(prettytable::row![cb -> "NAME", cb -> "TASK", cb -> "PACKAGE"]);

        if tasks.len() > 0 {
            for order_task in tasks {
                table.add_row(Row::new(vec![
                    Cell::new(&order_task.name),
                    // Cell::new(&order_task.task),
                    Cell::new(&order_task.task.to_string()),
                    Cell::new(&order_task.package),
                ]));
            }
        } else {
            table.add_row(Row::new(vec![
                Cell::new("exampleName"),
                Cell::new("Install"),
                Cell::new("tree"),
            ]));
        }

        table.printstd();
    }

    pub fn read_tasks(task_yaml: String) -> Vec<Task> {
        // Reads the YAML hosts file
        let mut file = File::open(task_yaml).expect("Couldn't open the file");

        // Reads the yaml and parses it to an String
        let mut yaml_string = String::new();
        file.read_to_string(&mut yaml_string)
            .expect("Couldn't read the file");

        // Deserializes the YAML file to a generic
        let yaml_value: Value =
            serde_yaml::from_str(&yaml_string).expect("Couldn't deserialize the YAML file");

        // Stores the tasks
        let mut tasks: Vec<Task> = Vec::new();

        if let Some(task_yaml) = yaml_value.as_sequence() {
            for task_order in task_yaml {
                let name = match task_order["name"].as_str() {
                    Some(name) => name.to_string(),
                    None => {
                        println!("You have to set a name!");
                        continue;
                    }
                };
                let task = match task_order["task"].as_str() {
                    Some("Run") => TaskType::Run,
                    Some("Install") => TaskType::Install,
                    Some("Uninstall") => TaskType::Uninstall,
                    Some("UpdateAll") => TaskType::UpdateAll,
                    Some("UpgradeAll") => TaskType::UpgradeAll,
                    Some("CopyToRemote") => TaskType::CopyToRemote,
                    None => TaskType::Nothing,
                    Some(&_) => TaskType::Nothing,
                };
                let command = match task_order["command"].as_str() {
                    Some(command) => command.to_string(),
                    None => " ".to_string(),
                };
                let package = match task_order["package"].as_str() {
                    Some(package) => package.to_string(),
                    None => " ".to_string(),
                };
                let file = match task_order["file"].as_str() {
                    Some(file) => file.to_string(),
                    None => " ".to_string(),
                };
                let task = Task {
                    name,
                    task,
                    command,
                    package,
                    file,
                };
                tasks.push(task);
            }
        }
        tasks
    }
}
