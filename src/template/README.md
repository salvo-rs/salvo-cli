# {{introduction}}
{{introduction_text}}
{{#if is_sqlite}}
{{/if}}
``` shell
//{{run_the_project}}
cargo run 
//{{run_the_tests}}
cargo test
```
# {{tip_title}}
- {{password_tip}}
{{# if is_sea_orm_or_sqlx}}
- {{config_tip}}
{{/if}}
# {{orm_title}}
{{#if is_sqlx}}
## sqlx_cli
{{sqlx_cli}} https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
## {{initialization}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{else}}
- {{initialization_sqlx_cli_not_sqlite}}
{{/if}}
{{/if}}
# {{project_dir_description}}
{{directory_contents}}
# {{about_salvo}}
{{about_salvo_text}}