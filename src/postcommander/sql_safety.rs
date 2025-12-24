#[derive(Debug, Clone, PartialEq)]
pub enum SqlDangerLevel {
    Safe,
    Warning(String),
    Dangerous(String),
}

pub fn analyze_sql(sql: &str) -> SqlDangerLevel {
    let sql_upper = sql.to_uppercase();
    let sql_trimmed = sql_upper.trim();

    if sql_trimmed.starts_with("DROP ") {
        return SqlDangerLevel::Dangerous("DROP statement will permanently delete database objects".to_string());
    }

    if sql_trimmed.starts_with("TRUNCATE ") {
        return SqlDangerLevel::Dangerous("TRUNCATE will permanently delete all rows from the table".to_string());
    }

    if sql_trimmed.starts_with("ALTER ") && sql_trimmed.contains("DROP ") {
        return SqlDangerLevel::Dangerous("ALTER...DROP will permanently remove columns or constraints".to_string());
    }

    if sql_trimmed.starts_with("DELETE ") {
        if !sql_upper.contains(" WHERE ") {
            return SqlDangerLevel::Warning("DELETE without WHERE clause will delete ALL rows".to_string());
        }
        if sql_upper.contains(" WHERE 1=1") || sql_upper.contains(" WHERE TRUE") {
            return SqlDangerLevel::Warning("DELETE with always-true condition will delete ALL rows".to_string());
        }
    }

    if sql_trimmed.starts_with("UPDATE ") {
        if !sql_upper.contains(" WHERE ") {
            return SqlDangerLevel::Warning("UPDATE without WHERE clause will update ALL rows".to_string());
        }
        if sql_upper.contains(" WHERE 1=1") || sql_upper.contains(" WHERE TRUE") {
            return SqlDangerLevel::Warning("UPDATE with always-true condition will update ALL rows".to_string());
        }
    }

    if sql_trimmed.starts_with("CREATE TABLE ") && sql_upper.contains("LIKE") && sql_upper.contains("INCLUDING ALL") {
        return SqlDangerLevel::Warning("CREATE TABLE...LIKE INCLUDING ALL will copy table structure".to_string());
    }

    SqlDangerLevel::Safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_select() {
        assert_eq!(analyze_sql("SELECT * FROM users"), SqlDangerLevel::Safe);
    }

    #[test]
    fn test_drop_dangerous() {
        match analyze_sql("DROP TABLE users") {
            SqlDangerLevel::Dangerous(_) => {}
            _ => panic!("DROP should be dangerous"),
        }
    }

    #[test]
    fn test_truncate_dangerous() {
        match analyze_sql("TRUNCATE TABLE users") {
            SqlDangerLevel::Dangerous(_) => {}
            _ => panic!("TRUNCATE should be dangerous"),
        }
    }

    #[test]
    fn test_delete_without_where_warning() {
        match analyze_sql("DELETE FROM users") {
            SqlDangerLevel::Warning(_) => {}
            _ => panic!("DELETE without WHERE should be warning"),
        }
    }

    #[test]
    fn test_delete_with_where_safe() {
        assert_eq!(
            analyze_sql("DELETE FROM users WHERE id = 1"),
            SqlDangerLevel::Safe
        );
    }

    #[test]
    fn test_update_without_where_warning() {
        match analyze_sql("UPDATE users SET name = 'test'") {
            SqlDangerLevel::Warning(_) => {}
            _ => panic!("UPDATE without WHERE should be warning"),
        }
    }

    #[test]
    fn test_update_with_where_safe() {
        assert_eq!(
            analyze_sql("UPDATE users SET name = 'test' WHERE id = 1"),
            SqlDangerLevel::Safe
        );
    }
}
