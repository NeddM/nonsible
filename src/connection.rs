mod task;
mod utils;
use crate::task::Task;
use crate::task::TaskType;

use colored::Colorize;
use prettytable::{Cell, Row, Table};
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    ip: Ipv4Addr,
    username: String,
    name: String,
    status: bool,
    sudo: bool,
    sudo_password: String,
    os: String,
    pem: String,
    labels: Vec<String>,
}

impl Connection {
    pub fn create_connection() -> Connection {
        let stdin = io::stdin();

        println!("New ip (Eg: 10.148.02.2): ");
        let mut ip = String::new();
        let _ = stdin.read_line(&mut ip);
        let ip_adress: Ipv4Addr = ip.trim().parse().expect("Parse failed");

        println!("New username: ");
        let mut username: String = String::new();
        let _ = stdin.read_line(&mut username);
        username = username.trim().to_string();

        println!("New name: ");
        let mut name: String = String::new();
        let _ = stdin.read_line(&mut name);
        name = name.trim().to_string();

        println!("Pem file? y/n: ");
        let mut r_pemfile: String = String::new();
        let _ = stdin.read_line(&mut r_pemfile);

        let pemfile = if r_pemfile.trim().to_uppercase() == "Y" {
            println!("Pem file: ");
            std::io::stdout().flush().unwrap();
            read_password().unwrap()
        } else {
            String::from("")
        };

        println!("Sudo? y/n: ");
        let mut r_sudo: String = String::new();
        let _ = stdin.read_line(&mut r_sudo);

        let sudo = if r_sudo.trim().to_uppercase() == "Y" {
            true
        } else {
            false
        };

        let sudo_password = if r_sudo.trim().to_uppercase() == "Y" {
            println!("Type the root password: ");
            std::io::stdout().flush().unwrap();
            read_password().unwrap()
        } else {
            String::from("")
        };

        println!("Adding labels? y/n: ");
        let mut res_labels: String = String::new();
        let _ = stdin.read_line(&mut res_labels);

        let labels = if res_labels.trim().to_uppercase() == "Y" {
            println!("Add the labels separating by commas");
            println!("E.g: label1,label2,label3");
            let mut prev_labels = String::new();
            let _ = stdin.read_line(&mut prev_labels);

            let split_labels: Vec<String> = prev_labels
                .split(",")
                .map(|s| s.trim().to_string())
                .collect();
            split_labels
        } else {
            let split_labels: Vec<String> = vec![];
            split_labels
        };

        let conn: Connection = Connection {
            ip: ip_adress,
            username,
            name,
            status: false,
            os: "Unknown".to_string(),
            sudo,
            sudo_password,
            pem: pemfile,
            labels,
        };
        conn
    }

    pub fn list_connection(connections: &mut Vec<Connection>) {
        let mut table = Table::new();

        table.add_row(prettytable::row![H5cb -> "TARGETS"]);
        table
            .add_row(prettytable::row![cb -> "NAME", cb -> "CONNECTION", cb -> "SUDO", cb -> "OS", cb -> "LABELS"]);

        if connections.len() > 0 {
            for conn in connections {
                let connection = conn.username.clone() + "@" + &conn.ip.to_string();
                table.add_row(Row::new(vec![
                    Cell::new(&conn.name),
                    Cell::new(&connection),
                    Cell::new(&conn.sudo.to_string()),
                    Cell::new(&conn.os),
                    Cell::new(&utils::representing_labels_as_string(conn.labels.clone())),
                ]));
            }
        } else {
            table.add_row(Row::new(vec![
                Cell::new("thisIsAnExample"),
                Cell::new("user@1.2.3.4"),
                Cell::new("true"),
                Cell::new("Ubuntu"),
                Cell::new("label1, label2"),
            ]));
        }

        table.printstd();
    }

    pub fn test_connection(connections: &mut Vec<Connection>) {
        for conn in connections {
            let built_ssh = conn.username.clone() + "@" + &conn.ip.to_string();
            let os_info = "grep '^PRETTY_NAME' /etc/os-release";
            let skip_fingerprint_check = "-o StrictHostKeyChecking=no";

            let test = if conn.ip.to_string() == "localhost" || conn.ip.to_string() == "127.0.0.1" {
                Command::new("grep")
                    .args([&skip_fingerprint_check, "'^PRETTY_NAME' /etc/os-release"])
                    .output()
                    .unwrap()
            } else if conn.pem.is_empty() {
                Command::new("ssh")
                    .args([&built_ssh, skip_fingerprint_check, os_info])
                    .output()
                    .unwrap()
            } else {
                Command::new("ssh")
                    .args([
                        "-i",
                        conn.pem.as_str(),
                        &built_ssh,
                        skip_fingerprint_check,
                        os_info,
                    ])
                    .output()
                    .unwrap()
            };

            let data_parsed = utils::parse_os(String::from_utf8_lossy(&test.stdout).to_string());

            if conn.ip.to_string() == "localhost" || conn.ip.to_string() == "127.0.0.1" {
                println!("Connection to {} succeeded", conn.name.green());
                conn.status = true;
                conn.os = data_parsed;
            } else {
                match test.status.code() {
                    Some(0) => {
                        println!("Connection to {} succeeded", conn.name.green());
                        conn.status = true;
                        conn.os = data_parsed;
                    }
                    _ => {
                        println!("Error connecting to {}", conn.name.red());
                        conn.os = "Connection failed!".to_string();
                    }
                }
            }
        }
    }

    pub fn install_package_no_tty(
        connections: &mut Vec<Connection>,
        packages: &mut Vec<Task>,
        args: &Vec<String>,
    ) {
        for conn in connections {
            // Breaks the loop if a connection fails
            if conn.os == "Connection failed!".to_string() {
                // Forces the installs if a connection fails
                if args.contains(&"--force".to_string()) {
                } else {
                    let warning_red = "  WARNING!!".to_string();
                    println!("\n{}", warning_red.red());
                    println!("A connection failed!");
                    println!("You can keep running Nonsible anyways using the argument --force \n");
                    return;
                }
            }
            for package in &mut *packages {
                // Skips the processes on the failed connections
                if conn.os == "Connection failed!".to_string() {
                    if args.contains(&"--continueonerror".to_string()) {
                    } else {
                        let warning_red = "  WARNING!!".to_string();
                        println!("\n{}", warning_red.red());
                        println!("A connection failed!");
                        println!("Nonsible wont do any task on failed connections");
                        println!(
                            "You can keep running the tasks anyways using the argument --continueonerror \n"
                        );
                        break;
                    }
                }
                for label in &conn.labels {
                    if package.matchlabels.contains(&label) || package.matchlabels.len() == 0 {
                        let sudo = utils::parse_sudo(conn.sudo);
                        let built_ssh = conn.username.clone() + "@" + &conn.ip.to_string();

                        let repo = if package.task == TaskType::Install {
                            utils::install_process(&conn.os)
                        } else if package.task == TaskType::Run {
                            let sudo = utils::parse_sudo(conn.sudo);
                            let built_ssh = conn.username.clone() + "@" + &conn.ip.to_string();
                            let pass_command = "echo '".to_owned() + &conn.sudo_password + "' | ";
                            let command = sudo + " -S " + &package.command;

                            let install = if conn.pem.is_empty() {
                                Command::new("ssh")
                                    .args([&built_ssh, &pass_command, &command])
                                    .output()
                                    .unwrap()
                            } else {
                                Command::new("ssh")
                                    .args([
                                        "-i",
                                        conn.pem.as_str(),
                                        &built_ssh,
                                        &pass_command,
                                        &command,
                                    ])
                                    .output()
                                    .unwrap()
                            };

                            // Just for debug the installation
                            if args.contains(&"--no-color".to_string()) {
                                println!("{:?}", install);
                            }

                            match install.status.code() {
                                Some(0) => println!(
                                    "{} succeessfully processsed on {}",
                                    package.name.green(),
                                    conn.name.green()
                                ),
                                _ => {
                                    println!("{} failed on {}", package.name.red(), conn.name.red())
                                }
                            }

                            continue;
                        } else if package.task == TaskType::Uninstall {
                            utils::uninstall_process(&conn.os)
                        } else if package.task == TaskType::UpdateAll {
                            utils::update_all_process(&conn.os)
                        } else if package.task == TaskType::UpgradeAll {
                            utils::upgrade_all_process(&conn.os)
                        } else if package.task == TaskType::CopyToRemote {
                            let scp = utils::copy_to_remote(
                                conn.username.to_string(),
                                conn.ip.to_string(),
                                package.file.to_string(),
                                conn.pem.to_string(),
                            );
                            println!("{}", scp);
                            let copy_to_remote = Command::new("scp").args([scp]).output().unwrap();
                            match copy_to_remote.status.code() {
                                Some(0) => println!(
                                    "{} succesfully processed on {}",
                                    package.name.green(),
                                    conn.name.green()
                                ),
                                _ => {
                                    println!("{} failed on {}", package.name.red(), conn.name.red())
                                }
                            }
                            println!("{:?}", copy_to_remote);

                            continue;
                        } else {
                            utils::install_process(&conn.os)
                        };

                        let pass_command = "echo '".to_owned() + &conn.sudo_password + "' | ";
                        let command = sudo + " -S " + &repo + &package.package.trim() + " -y ";

                        let install = if conn.ip.to_string() == "localhost"
                            || conn.ip.to_string() == "127.0.0.1"
                        {
                            Command::new(utils::install_process(&conn.os))
                                .args([&package.package.trim()])
                                .output()
                                .unwrap()
                        } else if conn.pem.is_empty() {
                            Command::new("ssh")
                                .args([&built_ssh, &pass_command, &command])
                                .output()
                                .unwrap()
                        } else {
                            Command::new("ssh")
                                .args([
                                    "-i",
                                    conn.pem.as_str(),
                                    &built_ssh,
                                    &pass_command,
                                    &command,
                                ])
                                .output()
                                .unwrap()
                        };

                        // Just for debug the installation
                        if args.contains(&"--no-color".to_string()) {
                            println!("{:?}", install);
                        }

                        match install.status.code() {
                            Some(0) => println!(
                                "{} succesfully processed on {}",
                                package.name.green(),
                                conn.name.green()
                            ),
                            _ => println!("{} failed on {}", package.name.red(), conn.name.red()),
                        }
                        break;
                    }
                }
            }
        }
    }

    pub fn install_package(connection: &mut Vec<Connection>) {
        let stdin = io::stdin();
        let mut package = String::new();
        println!("Name of the package to install: ");
        let _ = stdin.read_line(&mut package);

        for conn in connection {
            let sudo = utils::parse_sudo(conn.sudo);
            let built_ssh = conn.username.clone() + "@" + &conn.ip.to_string();
            let repo = utils::install_process(&conn.os);
            let pass_command = "echo '".to_owned() + &conn.sudo_password + "' | ";
            let command = sudo + " -S " + &repo + &package.trim() + " -y ";

            let install = if conn.pem.is_empty() {
                Command::new("ssh")
                    .args([&built_ssh, &pass_command, &command])
                    .output()
                    .unwrap()
            } else {
                Command::new("ssh")
                    .args(["-i", conn.pem.as_str(), &built_ssh, &pass_command, &command])
                    .output()
                    .unwrap()
            };

            // Just for debug the installation
            // println!("{:?}", install);

            match install.status.code() {
                Some(0) => println!("Installation succeeded on {}", conn.name.green()),
                _ => println!("Installation failed on {}", conn.name.red()),
            }
        }
    }

    pub fn uninstall_package(connection: &mut Vec<Connection>) {
        let stdin = io::stdin();
        let mut package = String::new();
        println!("Name of the package to install: ");
        let _ = stdin.read_line(&mut package);

        for conn in connection {
            let sudo = utils::parse_sudo(conn.sudo);
            let built_ssh = conn.username.clone() + "@" + &conn.ip.to_string();
            let repo = utils::uninstall_process(&conn.os);
            let pass_command = "echo '".to_owned() + &conn.sudo_password + "' | ";
            let command = sudo + " -S " + &repo + &package.trim() + " -y ";

            let install = if conn.pem.is_empty() {
                Command::new("ssh")
                    .args([&built_ssh, &pass_command, &command])
                    .output()
                    .unwrap()
            } else {
                Command::new("ssh")
                    .args(["-i", conn.pem.as_str(), &built_ssh, &pass_command, &command])
                    .output()
                    .unwrap()
            };

            // Just for debug the uninstallation
            // println!("{:?}", install);

            match install.status.code() {
                Some(0) => println!("Package removed on {}", conn.name.green()),
                _ => println!("Uninstallation failed on {}", conn.name.red()),
            }
        }
    }

    pub fn read_target(host_file: String) -> Vec<Connection> {
        // Reads the YAML hosts file
        let mut file = File::open(host_file).expect("Couldn't open the file");

        // Reads the yaml and parses it to an String
        let mut yaml_string = String::new();
        file.read_to_string(&mut yaml_string)
            .expect("Couldn't read the file");

        // Deserializes the YAML file to a generic
        let yaml_value: Value =
            serde_yaml::from_str(&yaml_string).expect("Couldn't deserialize the YAML file");

        // Initializes a vector to store connections
        let mut connections: Vec<Connection> = Vec::new();

        // Gets the data from the YAML hosts file
        if let Some(connetions_yaml) = yaml_value.as_sequence() {
            for connection_yaml in connetions_yaml {
                let name = match connection_yaml["name"].as_str() {
                    Some(name) => name.to_string(),
                    None => {
                        println!("You have to set a name!");
                        continue;
                    }
                };
                let username = match connection_yaml["username"].as_str() {
                    Some(username) => username.to_string(),
                    None => {
                        println!("You have to set a username!");
                        continue;
                    }
                };
                let ip = match connection_yaml["ip"].as_str() {
                    Some(ip) => ip.to_string(),
                    None => {
                        println!("You have to set an IP!");
                        continue;
                    }
                };
                let sudo = match connection_yaml["sudo"].as_bool() {
                    Some(sudo) => sudo,
                    None => false,
                };
                let pem = match connection_yaml["pem"].as_str() {
                    Some(pem) => pem.to_string(),
                    None => "".to_string(),
                };
                let sudo_password = match connection_yaml["sudo_password"].as_str() {
                    Some(sudo_password) => sudo_password.to_string(),
                    None => String::new(),
                };

                let labels = match connection_yaml["labels"].as_sequence() {
                    Some(labels_yaml) => labels_yaml
                        .iter()
                        .map(|label| label.as_str().unwrap_or_default().to_string())
                        .collect(),
                    None => Vec::new(),
                };

                let connection = Connection {
                    name,
                    username,
                    ip: ip.trim().parse().expect("Parse failed"),
                    sudo,
                    status: false,
                    os: "Unknown".trim().to_string(),
                    sudo_password,
                    pem,
                    labels,
                };

                connections.push(connection);
            }
        }
        connections
    }
}
