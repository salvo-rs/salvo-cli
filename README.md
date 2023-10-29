<div align="center">
    <img alt="Savlo" width="132" src="https://p.sda1.dev/13/9268fb110f27611fa143c7aafbac61ab/monkeybread4352_a_technology_software_logo_for_windseabird_with_8fb4a0df-e233-414e-80a3-cf144ef44209.png" />
</div>
<div align="center">
    <a href="https://github.com/salvo-rs/salvo-cli/actions">
        <img alt="build status" src="https://github.com/salvo-rs/salvo-cli/actions/workflows/rust.yml/badge.svg?branch=main" />
    </a>
    <a href="https://crates.io/crates/salvo-cli">
        <img alt="crates.io" src="https://img.shields.io/crates/v/salvo-cli" />
    </a>
    <a href="https://crates.io/crates/salvo-cli">
        <img alt="Download" src="https://img.shields.io/crates/d/salvo-cli.svg" />
    </a>
    <img alt="License" src="https://img.shields.io/crates/l/salvo-cli.svg" />
</div>

## Introduction

Salvo CLI is a command-line interface tool for the [Salvo](https://github.com/salvo-rs/salvo) web framework. It helps streamline the process of setting up a new Salvo project by generating a template structure.

## Installation

```bash
cargo install salvo-cli
```

## Usage

To create a new Salvo project, use the new command followed by the name of your project:

```bash
salvo-cli new project_name
```

## Update

```bashs
cargo install --force salvo-cli
```

### Feature Development Plan

| Status |                                        Plan                                        |
| :----: | :--------------------------------------------------------------------------------: |
|   ✅   |                                  web api template                                  |
|   ✅   |                                 web site template                                  |
|   ✅   | Template with SQLx, SeaORM, Diesel, Rbatis (support for SQLite, PostgreSQL, MySQL) |
|   ✅   |                                  Basic middleware                                  |
|   ⏳   |                                  More middleware                                   |
|   ⏳   |                                Support for MongoDB                                 |

## License

This project is licensed under the MIT OR Apache-2.0 License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
