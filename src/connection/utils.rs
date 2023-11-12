use regex::Regex;

pub fn parse_os(data: String) -> String {
    let re = Regex::new(r#"^PRETTY_NAME="([^"]+)""#).unwrap();
    let result = re.replace(&data, "$1");
    result.to_string()
}

pub fn parse_sudo(check_sudo: bool) -> String {
    match check_sudo {
        true => " sudo ".to_string(),
        _ => " ".to_string(),
    }
}

pub fn install_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => return " apt install ".to_string(),
        "DEBIAN" => return " apt-get install ".to_string(),
        "ARCH" => return " pacman -S ".to_string(),
        "MANJARO" => return " pacman -S ".to_string(),
        "FEDORA" => return " dnf install ".to_string(),
        "CENTOS" => return " dnf install ".to_string(),
        "RED" => return " dnf install ".to_string(),
        "SUSE" => return " zypper install ".to_string(),
        _ => return " apt install ".to_string(),
    }
}

pub fn uninstall_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => return " apt remove ".to_string(),
        "DEBIAN" => return " apt-get remove ".to_string(),
        "ARCH" => return " pacman -R ".to_string(),
        "MANJARO" => return " pacman -R ".to_string(),
        "FEDORA" => return " dnf remove ".to_string(),
        "CENTOS" => return " dnf remove ".to_string(),
        "RED" => return " dnf remove ".to_string(),
        "SUSE" => return " zypper remove ".to_string(),
        _ => return " apt remove ".to_string(),
    }
}

pub fn update_all_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => return " apt update ".to_string(),
        "DEBIAN" => return " apt-get update ".to_string(),
        "ARCH" => return " pacman -Sy ".to_string(),
        "MANJARO" => return " pacman -Sy ".to_string(),
        "FEDORA" => return " dnf check-update ".to_string(),
        "CENTOS" => return " dnf check-update ".to_string(),
        "RED" => return " dnf check-update ".to_string(),
        "SUSE" => return " zypper refresh ".to_string(),
        _ => return " apt update ".to_string(),
    }
}

pub fn upgrade_all_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => return " apt upgrade -y ".to_string(),
        "DEBIAN" => return " apt-get upgrade -y ".to_string(),
        "ARCH" => return " pacman -Syu --noconfirm ".to_string(),
        "MANJARO" => return " pacman -Syu --noconfirm ".to_string(),
        "FEDORA" => return " dnf upgrade -y ".to_string(),
        "CENTOS" => return " dnf upgrade -y ".to_string(),
        "RED" => return " dnf upgrade -y ".to_string(),
        "SUSE" => return " zypper update -y ".to_string(),
        _ => return " apt upgrade -y ".to_string(),
    }
}

pub fn copy_to_remote(username: String, ip: String, file_path: String, pem: String) -> String {
    let command = "-i ".to_string() + &pem + " " + &file_path + " " + &username + "@" + &ip + ":";
    command
}

pub fn representing_labels_as_string(vector: Vec<String>) -> String {
    let mut result = String::new();

    for (index, item) in vector.iter().enumerate() {
        result.push_str(item);

        if index < vector.len() - 1 {
            result.push_str(", ");
        }
    }

    result
}
