#[cfg(test)]
mod tests {
    use crate::{
        utils::{
            create_project::write_project_file,
            get_selection::{DbConnectionType, DbType, TemplateType, UserSelected},
        },
        Project,
    };
    use itertools::Itertools;
    use std::path::Path;

    #[test]
    fn test_write_project_all_combinations() {
        let template_types = [TemplateType::SalvoWebSite, TemplateType::SalvoWebApi];
        //let db_types = [DbType::Sqlite, DbType::Mysql, DbType::Postgres, DbType::Mssql];
        let db_types = [DbType::Sqlite];
        let db_conn_types = [
            DbConnectionType::Sqlx,
            DbConnectionType::SeaOrm,
            DbConnectionType::Diesel,
            DbConnectionType::Rbatis,
            DbConnectionType::Mongodb,
            DbConnectionType::Nothing,
        ];

        // Generate all combinations
        let combinations = template_types
            .iter()
            .cartesian_product(db_types.iter())
            .cartesian_product(db_conn_types.iter())
            .map(|((template_type, db_type), db_conn_type)| (template_type, db_type, db_conn_type))
            .collect::<Vec<_>>();

        // Test each combination
        for (template_type, db_type, db_conn_type) in combinations {
            // Generate a unique project name for each combination
            let project_name = format!("test_{:?}_{:?}_{:?}", template_type, db_type, db_conn_type);
            println!("Testing combination: {:?}", project_name);
            let path_str = format!("target/{}", project_name);
            std::fs::remove_dir_all(&path_str).unwrap_or(());
            let path = Path::new(&path_str);

            let user_selected = UserSelected {
                template_type: *template_type,
                db_type: *db_type,
                db_conn_type: *db_conn_type,
            };
            let project = Project {
                project_name: project_name.clone(),
                lang: Some("zh".to_string()),
            };
            match write_project_file(path, user_selected, project) {
                Ok(()) => {
                    let output = std::process::Command::new("cargo")
                        .arg("check")
                        .current_dir(&path_str)
                        .output()
                        .expect("failed to execute process");
                    if !output.status.success() {
                        eprintln!(
                            "Failed on combination: template_type={:?}, db_type={:?}, db_conn_type={:?}",
                            template_type, db_type, db_conn_type
                        );
                        eprintln!("Output: {:?}", output);
                        panic!();
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to write project file on combination: template_type={:?}, db_type={:?}, db_conn_type={:?}",
                        template_type, db_type, db_conn_type
                    );
                    eprintln!("Error: {:?}", e);
                    panic!();
                }
            }
            std::fs::remove_dir_all(&path_str).unwrap_or(());
        }
    }
}
