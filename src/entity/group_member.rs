//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use super::sea_orm_active_enums::GroupRole;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "group_member")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub role: Option<GroupRole>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Group,
    #[sea_orm(
        belongs_to = "super::user_credential::Entity",
        from = "Column::UserId",
        to = "super::user_credential::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UserCredential,
}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Group.def()
    }
}

impl Related<super::user_credential::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserCredential.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
