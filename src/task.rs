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
    pub src_file: String,
    pub dst_file: String,
    pub matchlabels: Vec<String>,
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

        table.add_row(prettytable::row![H4cb -> "TASKS"]);
        table.add_row(
            prettytable::row![cb -> "NAME", cb -> "TASK", cb -> "PACKAGE", cb -> "MATCH LABELS"],
        );

        if !tasks.is_empty() {
            for order_task in tasks {
                table.add_row(Row::new(vec![
                    Cell::new(&order_task.name),
                    Cell::new(&order_task.task.to_string()),
                    Cell::new(&order_task.package),
                    Cell::new(&representing_matchlabels_as_string(
                        order_task.matchlabels.clone(),
                    )),
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
        // Reads the YAML tasks file
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
                let src_file = match task_order["srcFile"].as_str() {
                    Some(file) => file.to_string(),
                    None => " ".to_string(),
                };
                let dst_file = match task_order["dstFile"].as_str() {
                    Some(file) => file.to_string(),
                    None => " ".to_string(),
                };

                let matchlabels = match task_order["matchLabels"].as_sequence() {
                    Some(labels_yaml) => labels_yaml
                        .iter()
                        .map(|label| label.as_str().unwrap_or_default().to_string())
                        .collect(),
                    None => Vec::new(),
                };

                let task = Task {
                    name,
                    task,
                    command,
                    package,
                    src_file,
                    dst_file,
                    matchlabels,
                };
                tasks.push(task);
            }
        }
        tasks
    }
}

fn representing_matchlabels_as_string(vector: Vec<String>) -> String {
    let mut result = String::new();

    for (index, item) in vector.iter().enumerate() {
        result.push_str(item);

        if index < vector.len() - 1 {
            result.push_str(", ");
        }
    }

    result
}
