
# Salvo CLI

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

|  Status |Plan   |   
|:---:|:---:| 
|✅| web api template |    
|✅| web site template |   
|✅|Template with SQLx, SeaORM, Diesel, Rbatis (support for SQLite, PostgreSQL, MySQL)| 
|✅|Basic middleware |
|⏳|More middleware|
|⏳|Support for MongoDB|   
## License
This project is licensed under the MIT OR Apache-2.0 License.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.
