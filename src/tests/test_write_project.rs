#[cfg(test)]
mod tests {
    use std::path::Path;

    use itertools::Itertools;

    use crate::templates::classic;
    use crate::templates::classic::selection::{DbLib, DbType, Selected};
    use crate::Project;

    #[test]
    fn test_write_project_all_combinations() {
        let db_types = [
            DbType::Sqlite,
            DbType::Mysql,
            DbType::Postgres,
            DbType::Mongodb,
        ];
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
            .filter(|(db_type, db_lib)| match db_lib {
                DbLib::Sqlx => *db_type == DbType::Sqlite,
                DbLib::Mongodb => *db_type == DbType::Mongodb,
                DbLib::SeaOrm | DbLib::Diesel | DbLib::Rbatis => {
                    *db_type == DbType::Sqlite
                        || *db_type == DbType::Mysql
                        || *db_type == DbType::Postgres
                }
                _ => false,
            })
            .collect::<Vec<_>>();

        // Test each combination
        for (db_type, db_lib) in combinations {
            let proj = Project {
                name: format!("test_{:?}_{:?}", db_type, db_lib),
                lang: "zh".to_string(),
            };
            println!("Testing combination: {:?}", proj.name);
            let path_str = format!("target/{}", proj.name);
            std::fs::remove_dir_all(&path_str).unwrap_or(());
            let path = Path::new(&path_str);

            let user_selected = Selected {
                db_type: *db_type,
                db_lib: *db_lib,
            };
            match classic::create_files(path, user_selected, &proj) {
                Ok(()) => {
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
                }
                Err(e) => {
                    eprintln!(
                        "Failed to write project file on combination: db_type={:?}, db_lib={:?}",
                        db_type, db_lib
                    );
                    eprintln!("Error: {:?}", e);
                    panic!();
                }
            }
            std::fs::remove_dir_all(&path_str).unwrap_or(());
        }
    }
}
