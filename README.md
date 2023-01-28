<div align="center">

# 📦 Universal Package Manager
Reimagining package management in a simplified manner

</div>

## Supports

1. [NPM](https://docs.npmjs.com/cli/v9/commands/npm)
2. [YARN(classic)](https://classic.yarnpkg.com)
3. [YARN](https://yarnpkg.com)
4. [PNPM](https://pnpm.io)
5. [Bun](https://bun.sh)
6. [Poetry](https://python-poetry.org)
7. [Cargo](https://doc.rust-lang.org/cargo/index.html)

> many on the way.
> Contribuitons are welcome 🤗


## Features

- Very Fast and Performant
- Memory Safety
- Simplified commands
- Lightweight
- Easy to use
- Cross Platform
- Automatic arguments passing
- Highly Scalable

## Motivation 🤔

> This project is intented to simplify the overwhelming and confusing package manager world(wars).
> This makes the job of CL/CI and Automation developers to write automations in a simplified manner with very less to no overhead.
> UPM will be an life changer for people woking with multiple package managers in multiple forms.

## Usage

### Install all dependencies
```sh
upm i
```

### Install a dependency
```sh
upm i <package>
```

### Remove a dependency
```sh
upm rm <package>
```

### Upgrade all dependencies
```sh
upm up
```

### Locked(Frozen) install
```sh
upm fz
```

### Run an automation script
```sh
upm run <automation>
```
or
```sh
upm <automation>
```

## Plans

- Adding support for more package managers like PIP, Go, Flutter and more
- Customized project scaffolding with professional custom templates
- Adding installation scripts and wrappers to easy installaiton on various platforms

## Contributions

Any other package manager can be added by adding proper alias and command equivalents in [`cmds.rs`](/src/cmds.rs)

## LICENSE

[MIT](/LICENSE)


<div align="center">

Made with ❤️ by [Rajaniraiyn](https://rajaniraiyn.github.io)

</div>
