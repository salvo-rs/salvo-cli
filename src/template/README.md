# {{introduction}}
{{introduction_text}}
{{{rust_version_tip}}}
{{#if is_sqlite}}
{{/if}}
``` shell
//{{run_the_project}}
cargo run 
//{{run_the_tests}}
cargo test
```
{{#if need_db_conn}}
# {{tip_title}}
- {{password_tip}}
{{# if is_sea_orm_or_sqlx}}
- {{config_tip}}
{{/if}}
# {{orm_title}}
{{#if is_sqlx}}
{{sqlx_website}}
## sqlx_cli
{{sqlx_cli}} https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
## {{initialization}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{else}}
- {{initialization_sqlx_cli_not_sqlite}}
{{/if}}
{{/if}}
{{#if is_sea_orm}}
{{sea_orm_website}}
## sea_orm_cli
{{sea_orm_cli_website}}
## {{initialization}}
{{/if}}
{{#if is_sea_orm}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{else}}
- {{initialization_seaorm_cli_not_sqlite}}
{{/if}}
{{/if}}
{{#if is_diesel}}
{{diesel_website}}
## diesel_cli
{{diesel_cli_website}}
## {{initialization}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{else}}
- {{initialization_diesel_cli_not_sqlite}}
{{/if}}
{{/if}}
{{#if is_rbatis}}
{{rbatis_website}}
## {{initialization}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{else}}
- {{initialization_rbatis_cli_not_sqlite}}
{{/if}}
{{/if}}
{{#if is_mongodb}}
{{mongodb_website}}
## {{initialization}}
- {{mongodb_usage_import_user_data}}
{{/if}}
{{/if}}
# {{project_dir_description}}
{{directory_contents}}
# cargo-deny
``` shell
cargo install --locked cargo-deny && cargo deny check
```
# git cliff
{{git_cliff}}
# {{about_salvo}}
{{about_salvo_text}}