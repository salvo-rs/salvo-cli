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

Salvo CLI, a tool for the [Salvo](https://github.com/salvo-rs/salvo) web framework, creates clean, easy-to-read code, saving you time for the more enjoyable aspects of life.

If you've got a knack for making  CLI even better, or if you've noticed a few quirks that could use some attention, don't be shy! Drop us an issue, we welcome your insights. 
## Installation

```bash
cargo install salvo-cli
```

## Usage

To create a new Salvo project, use the new command followed by the name of your project:

```bash
//use the local language
salvo new project_name

// Use English
salvo new project_name --lang=en

// 使用简体中文
salvo new project_name --lang=zh

// 使用繁體中文
salvo new project_name --lang=zh_TW

// Utilisez le français
salvo new project_name --lang=fr

// 日本語を使用する
salvo new project_name --lang=ja

// Usa el español
salvo new project_name --lang=es

// Verwenden Sie Deutsch
salvo new project_name --lang=de

// Используйте русский
salvo new project_name --lang=ru

// Usa l `italiano
salvo new project_name --lang=it

// Use o português
salvo new project_name --lang=pt

// 한국어를 사용하십시오
salvo new project_name --lang=ko

// Bruk norsk
salvo new project_name --lang=no

// Notaðu íslensku
salvo new project_name --lang=is

// Використовуйте українську
salvo new project_name --lang=uk

// ใช้ภาษาไทย
salvo new project_name --lang=th

// Χρησιμοποιήστε την ελληνική
salvo new project_name --lang=el

// Brug dansk
salvo new project_name --lang=da
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
|   ✅   |                                jwt,cors... middleware                              |
|   ✅   |                                Support for MongoDB                                 |
|   ⏳   |                                command:salvo run                                   |
|   ⏳   |                                  Support for docker                                |
|   ⏳   | More integrations with good crates (validation, permissions or others?)            |
|   ⏳   | Split into multiple crates for clearer code organization                           |

## License

This project is licensed under the MIT OR Apache-2.0 License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
