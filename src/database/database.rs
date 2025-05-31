use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

pub async fn connect_to_database() -> Result<(), DbErr> {
    let database_url =
        std::env::var("DATABASE_URL").expect("Environment: Missing DATABASE_URL variable");
    let database_url = database_url.as_str();
    let db_name = "rwirrer";

    let db = Database::connect(database_url).await?;
    // Migrator::up(&connection, None).await?;

    let _ = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            Database::connect(database_url).await?
        }
        DbBackend::Postgres => {
            // db.execute(Statement::from_string(
            //     db.get_database_backend(),
            //     format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            // ))
            // .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            Database::connect(database_url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(())
}
