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
        "UBUNTU" => " apt install ".to_string(),
        "DEBIAN" => " apt-get install ".to_string(),
        "ARCH" => " pacman -S ".to_string(),
        "MANJARO" => " pacman -S ".to_string(),
        "FEDORA" => " dnf install ".to_string(),
        "CENTOS" => " dnf install ".to_string(),
        "RED" => " dnf install ".to_string(),
        "SUSE" => " zypper install ".to_string(),
        _ => " apt install ".to_string(),
    }
}

pub fn uninstall_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => " apt remove ".to_string(),
        "DEBIAN" => " apt-get remove ".to_string(),
        "ARCH" => " pacman -R ".to_string(),
        "MANJARO" => " pacman -R ".to_string(),
        "FEDORA" => " dnf remove ".to_string(),
        "CENTOS" => " dnf remove ".to_string(),
        "RED" => " dnf remove ".to_string(),
        "SUSE" => " zypper remove ".to_string(),
        _ => " apt remove ".to_string(),
    }
}

pub fn update_all_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => " apt update ".to_string(),
        "DEBIAN" => " apt-get update ".to_string(),
        "ARCH" => " pacman -Sy ".to_string(),
        "MANJARO" => " pacman -Sy ".to_string(),
        "FEDORA" => " dnf check-update ".to_string(),
        "CENTOS" => " dnf check-update ".to_string(),
        "RED" => " dnf check-update ".to_string(),
        "SUSE" => " zypper refresh ".to_string(),
        _ => " apt update ".to_string(),
    }
}

pub fn upgrade_all_process(so: &str) -> String {
    match so.to_uppercase().as_str() {
        "UBUNTU" => " apt upgrade -y ".to_string(),
        "DEBIAN" => " apt-get upgrade -y ".to_string(),
        "ARCH" => " pacman -Syu --noconfirm ".to_string(),
        "MANJARO" => " pacman -Syu --noconfirm ".to_string(),
        "FEDORA" => " dnf upgrade -y ".to_string(),
        "CENTOS" => " dnf upgrade -y ".to_string(),
        "RED" => " dnf upgrade -y ".to_string(),
        "SUSE" => " zypper update -y ".to_string(),
        _ => " apt upgrade -y ".to_string(),
    }
}

pub fn copy_to_remote_string(username: String, ip: String, dst_path: String) -> String {
    format!("{}@{}:{}", username, ip, dst_path)
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
