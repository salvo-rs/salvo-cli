# {introduction}
{{introduction_text}}
{{#if is_sqlite}}
{{seleted_sqlite}}
{{/if}}
``` shell
//{{run_the_project}}
cargo run 
//{{run_the_tests}}
cargo test
```
{{#if is_sqlx}}
## sqlx_cli
{{sqlx_cli}} https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md
{{/if}}
# {{project_dir_description}}
{{directory_contents}}
# {{about_salvo}}
{{about_salvo_text}}