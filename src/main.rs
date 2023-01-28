mod cmds;
mod detect;
mod execute;
mod init;
use cmds::*;
use detect::*;
use execute::*;
use init::*;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();

    if args[0] == "init" {
        init_project();
        return;
    }

    let package_manager = detect_package_manager();
    return if args.len() == 0 {
        install_all(package_manager)
    } else {
        let cmd = args[0].trim();
        if alias().get("i").iter().any(|x| x.contains(&cmd))
            || alias().get("add").iter().any(|x| x.contains(&cmd))
        {
            install_deps(package_manager)
        } else if alias().get("rm").iter().any(|x| x.contains(&cmd)) {
            uninstall_deps(package_manager)
        } else if alias().get("up").iter().any(|x| x.contains(&cmd)) {
            update_deps(package_manager)
        } else if alias().get("run").iter().any(|x| x.contains(&cmd)) {
            run_script(package_manager)
        } else if alias().get("fz").iter().any(|x| x.contains(&cmd)) {
            install_frozen(package_manager)
        } else {
            run_script_special(package_manager)
        }
    };
}
