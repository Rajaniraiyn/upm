use colored::Colorize;

use crate::cmds::*;

pub fn run_cmd_with_args(cmd: &str, args: Vec<&str>) {
    let cmd = cmd.replace("berry", "yarn"); // berry is an alias for yarn

    if is_in_path(cmd.clone()) {
        let mut command = std::process::Command::new(cmd)
            .args(args)
            .spawn()
            .expect("Failed to execute command");

        command.wait().expect("Failed to wait on command");
    } else {
        println!("{} is not installed", cmd);
        println!(
            "Install it first by following the steps from {}`",
            install_links()
                .get(&cmd.as_str())
                .unwrap()
                .underline()
                .blue()
        );
    }
}

pub fn install_all(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("i").unwrap();

    run_cmd_with_args(cmd, eq.to_vec());
}

pub fn install_deps(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("add").unwrap();

    let binding = std::env::args().skip(2).collect::<Vec<String>>();
    let deps = binding.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    run_cmd_with_args(cmd, eq.iter().copied().chain(deps).collect());
}

pub fn uninstall_deps(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("rm").unwrap();

    let binding = std::env::args().skip(2).collect::<Vec<String>>();
    let deps = binding.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    run_cmd_with_args(cmd, eq.iter().copied().chain(deps).collect());
}

pub fn update_deps(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("up").unwrap();

    let binding = std::env::args().skip(2).collect::<Vec<String>>();
    let deps = binding.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    run_cmd_with_args(cmd, eq.iter().copied().chain(deps).collect());
}

pub fn run_script(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("run").unwrap();

    let binding = std::env::args().skip(2).collect::<Vec<String>>();
    let script = binding.first().unwrap().as_str();

    let args = if cmd == "npm" {
        let mut eq = eq.to_vec();
        for element in eq.iter_mut() {
            if *element == "{-}" {
                *element = script;
            }
        }
        eq.to_vec()
    } else {
        eq.iter()
            .copied()
            .chain(vec![script])
            .collect::<Vec<&str>>()
    };

    if binding.len() > 1 {
        let binding = binding
            .iter()
            .skip(1)
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();
        run_cmd_with_args(cmd, args.iter().copied().chain(binding).collect());
    } else {
        run_cmd_with_args(cmd, args);
    }
}

pub fn run_script_special(package_manager: String) {
    let binding = std::env::args().skip(1).collect::<Vec<String>>();
    let args = binding.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    run_cmd_with_args(&package_manager, args)
}

pub fn install_frozen(package_manager: String) {
    let cmd = package_manager.as_str();

    let binding = equivalents();
    let eq = binding.get(cmd).unwrap().get("fz").unwrap();

    run_cmd_with_args(cmd, eq.to_vec());
}

pub fn is_in_path(cmd: String) -> bool {
    let path = std::env::var("PATH").unwrap();
    let paths = path.split(":");
    for path in paths {
        let path = std::path::Path::new(path);
        let path = path.join(cmd.as_str());
        if path.exists() {
            return true;
        }
    }
    false
}
