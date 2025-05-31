use sea_orm_migration::{
    prelude::{extension::postgres::TypeDropStatement, *},
    schema::*,
    sea_orm::{ActiveEnum, DbBackend, DeriveActiveEnum, EnumIter, Schema},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager
            .create_type(schema.create_enum_from_active_enum::<AccountStatus>())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Name))
                    .col(string(User::Email))
                    .col(enumeration(
                        User::Status,
                        "account_status",
                        AccountStatus::iden_values(),
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Text))
                    .col(integer(Post::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-user_id")
                            .from(Post::Table, Post::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Followers::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .col(Followers::FollowedId)
                            .col(Followers::FollowerId),
                    )
                    .col(integer(Followers::FollowedId))
                    .col(integer(Followers::FollowerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-followed_id-user_id")
                            .from(Followers::Table, Followers::FollowedId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-follower_id-user_id")
                            .from(Followers::Table, Followers::FollowerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(
                TypeDropStatement::new()
                    .name(AccountStatus::name())
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Followers::Table).to_owned())
            .await?;
        Ok(())
    }
}

///
/// Tables
///

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Status,
}

#[derive(DeriveIden)]
pub enum Post {
    Table,
    Id,
    Text,
    UserId,
}

#[derive(DeriveIden)]
pub enum Followers {
    Table,
    FollowedId,
    FollowerId,
}

///
/// Types
///

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "account_status")]
pub enum AccountStatus {
    #[sea_orm(string_value = "Created")]
    Created,
    #[sea_orm(string_value = "Pending")]
    Pending,
    #[sea_orm(string_value = "Approved")]
    Approved,
    #[sea_orm(string_value = "Rejected")]
    Rejected,
}
