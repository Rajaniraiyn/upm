use crate::cmds::SUPPORTED_PACKAGE_MANAGERS;
use crate::execute::run_cmd_with_args;
use inquire::Select;

// TODO: Add support for custom boilerplate and more options
pub fn init_project() {
    let package_manager = Select::new(
        "Choose Package Manager: ",
        SUPPORTED_PACKAGE_MANAGERS.to_vec(),
    )
    .prompt()
    .unwrap();

    run_cmd_with_args(package_manager, vec!["init"])
}
