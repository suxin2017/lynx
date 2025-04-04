//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4
use async_trait::async_trait;
use schemars::JsonSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize,TS)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "rule_handle")]
#[ts(export, export_to = "HandlerModel.ts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub r#type: HandlerType,
    pub switch: bool,
    pub data: Json,
    pub rule_id: i32,
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema,TS
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "camelCase"
)]
#[ts(export)]
pub enum HandlerType {
    ConnectPassProxy,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::rule::Entity",
        from = "Column::RuleId",
        to = "super::rule::Column::Id"
    )]
    Rule,
}

impl Related<super::rule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rule.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}
