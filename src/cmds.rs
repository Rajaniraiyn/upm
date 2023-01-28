use std::collections::HashMap;

pub fn equivalents() -> HashMap<&'static str, HashMap<&'static str, Vec<&'static str>>> {
    let npm: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["install"]),
        ("rm", vec!["uninstall"]),
        ("up", vec!["update"]),
        ("run", vec!["run", "{-}", "--"]),
        ("fz", vec!["ci"]),
        ("init", vec!["init"]),
    ]);

    let yarn: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["install"]),
        ("rm", vec!["remove"]),
        ("up", vec!["upgrade-interactive"]),
        ("run", vec!["run"]),
        ("fz", vec!["install", "--frozen-lockfile"]),
    ]);

    let berry: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["install"]),
        ("rm", vec!["remove"]),
        ("up", vec!["up", "-i"]),
        ("run", vec!["run", "--"]),
        ("fz", vec!["install", "--immutable"]),
    ]);

    let pnpm: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["install"]),
        ("rm", vec!["remove"]),
        ("up", vec!["update"]),
        ("run", vec!["run", "--"]),
        ("fz", vec!["install", "--frozen-lockfile"]),
    ]);

    let bun: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["install"]),
        ("rm", vec!["remove"]),
        ("up", vec!["update", "-i"]),
        ("run", vec!["run", "--"]),
        ("fz", vec!["install", "--no-save"]),
    ]);

    let poetry: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["add"]),
        ("rm", vec!["remove"]),
        ("up", vec!["update"]),
        ("run", vec!["run"]),
        ("fz", vec!["install", "--sync"]),
    ]);

    let cargo: HashMap<&str, Vec<&str>> = HashMap::from([
        ("i", vec!["install"]),
        ("add", vec!["add"]),
        ("rm", vec!["remove"]),
        ("up", vec!["update"]),
        ("run", vec!["run"]),
        ("fz", vec!["install", "--frozen"]),
    ]);

    HashMap::from([
        ("npm", npm),
        ("yarn", yarn),
        ("berry", berry),
        ("pnpm", pnpm),
        ("bun", bun),
        ("poetry", poetry),
        ("cargo", cargo),
    ])
}

pub fn alias() -> HashMap<&'static str, Vec<&'static str>> {
    HashMap::from([
        ("i", vec!["install", "i"]),
        ("add", vec!["install", "add", "i"]),
        ("rm", vec!["uninstall", "remove", "rm"]),
        ("up", vec!["update", "upgrade", "up"]),
        ("run", vec!["run", "r"]),
        ("fz", vec!["frozen", "fz", "ci"]),
    ])
}

pub const SUPPORTED_PACKAGE_MANAGERS: [&str; 7] =
    ["npm", "yarn", "berry", "pnpm", "bun", "poetry", "cargo"];

pub fn install_links() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("npm", "https://nodejs.org/en/"),
        ("yarn", "https://classic.yarnpkg.com/en/docs/install"),
        ("berry", "https://yarnpkg.com/getting-started/install"),
        ("pnpm", "https://pnpm.io/installation"),
        ("bun", "https://bun.sh"),
        ("poetry", "https://python-poetry.org/docs/#installation"),
        (
            "cargo",
            "https://doc.rust-lang.org/cargo/getting-started/installation.html",
        ),
    ])
}
