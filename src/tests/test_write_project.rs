#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use itertools::Itertools;

    use crate::Project;
    use crate::templates::classic;
    use crate::templates::classic::selection::{DbLib, DbType, Selected};

    fn render_project(name: String, lang: &str, user_selected: Selected) -> String {
        let proj = Project {
            name,
            lang: lang.to_string(),
        };
        println!("Testing combination: {:?}", proj.name);
        let path_str = format!("target/{}", proj.name);
        fs::remove_dir_all(&path_str).unwrap_or(());
        let path = Path::new(&path_str);
        classic::create_files(path, user_selected, &proj)
            .unwrap_or_else(|e| panic!("project '{}' should render: {e}", proj.name));
        path_str
    }

    fn cleanup(path_str: &str) {
        fs::remove_dir_all(path_str).unwrap_or(());
    }

    #[test]
    fn test_write_project_all_combinations() {
        // let db_types = [DbType::Sqlite, DbType::Mysql, DbType::Postgres, DbType::Mssql];
        let db_types = [DbType::Sqlite, DbType::Mongodb];
        let db_libs = [
            DbLib::Sqlx,
            DbLib::SeaOrm,
            DbLib::Diesel,
            DbLib::Rbatis,
            DbLib::Mongodb,
        ];

        // Generate all combinations
        let combinations = db_types
            .iter()
            .cartesian_product(db_libs.iter())
            .collect::<Vec<_>>();

        // Test each combination
        for (db_type, db_lib) in combinations {
            if (db_lib == &DbLib::Mongodb && db_type != &DbType::Mongodb)
                || (db_lib != &DbLib::Mongodb && db_type == &DbType::Mongodb)
            {
                continue;
            }

            let user_selected = Selected {
                db_type: *db_type,
                db_lib: *db_lib,
            };
            let path_str = render_project(
                format!("test_{:?}_{:?}", db_type, db_lib),
                "zh",
                user_selected,
            );

            let output = std::process::Command::new("cargo")
                .arg("check")
                .current_dir(&path_str)
                .output()
                .expect("failed to execute process");
            if !output.status.success() {
                eprintln!(
                    "Failed on combination: db_type={:?}, db_lib={:?}",
                    db_type, db_lib
                );
                eprintln!("Output: {:?}", output);
                panic!();
            }

            cleanup(&path_str);
        }
    }

    #[test]
    fn test_seaorm_templates_render_project_and_migration_docs_correctly() {
        let cases = [
            (DbType::Sqlite, "sqlx-sqlite"),
            (DbType::Mysql, "sqlx-mysql"),
            (DbType::Postgres, "sqlx-postgres"),
        ];

        for (db_type, driver_feature) in cases {
            let path_str = render_project(
                format!("test_{:?}_SeaOrm_docs", db_type),
                "en",
                Selected {
                    db_type,
                    db_lib: DbLib::SeaOrm,
                },
            );

            let root_readme = fs::read_to_string(format!("{path_str}/README.md"))
                .expect("root README should exist");
            assert!(
                !root_readme.contains("# Running Migrator CLI"),
                "SeaORM root README should use the base project instructions"
            );
            assert!(
                root_readme.contains("## seaorm_cli"),
                "SeaORM root README should include the base SeaORM setup guide"
            );

            let migration_readme = fs::read_to_string(format!("{path_str}/migration/README.md"))
                .expect("migration README should exist");
            assert!(
                migration_readme.contains("Run these commands from the `migration/` directory."),
                "SeaORM migration README should clarify where to run migration commands"
            );

            let migration_cargo = fs::read_to_string(format!("{path_str}/migration/Cargo.toml"))
                .expect("migration Cargo.toml should exist");
            assert!(
                migration_cargo.contains(driver_feature),
                "SeaORM migration crate should enable the correct driver feature for {db_type:?}"
            );

            cleanup(&path_str);
        }
    }
}
