//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4
use anyhow::anyhow;
use async_trait::async_trait;
use schemars::{schema_for, JsonSchema};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "rule_content")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub content: Json,
    pub rule_id: i32,
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

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Capture {
    /// use glob pattern to match the uri
    /// syntax: https://crates.io/crates/glob-match
    pub uri: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RuleContent {
    pub capture: Capture,
    pub handler: Handler,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Handler {
    /// proxy pass to the target
    /// example: http://localhost:8080
    pub proxy_pass: String,
}

pub fn parse_rule_content(instance: serde_json::Value) -> anyhow::Result<RuleContent> {
    let schema = serde_json::to_value(schema_for!(RuleContent)).unwrap();
    jsonschema::validate(&schema, &instance).map_err(|e| anyhow!(format!("{}", e)))?;
    let rule_content: RuleContent = serde_json::from_value(instance)?;
    Ok(rule_content)
}

