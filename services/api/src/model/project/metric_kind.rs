use std::str::FromStr;

use bencher_json::{
    project::metric_kind::{
        JsonUpdateMetricKind, ESTIMATED_CYCLES_NAME_STR, ESTIMATED_CYCLES_SLUG_STR,
        INSTRUCTIONS_NAME_STR, INSTRUCTIONS_SLUG_STR, L1_ACCESSES_NAME_STR, L1_ACCESSES_SLUG_STR,
        L2_ACCESSES_NAME_STR, L2_ACCESSES_SLUG_STR, LATENCY_NAME_STR, LATENCY_SLUG_STR,
        RAM_ACCESSES_NAME_STR, RAM_ACCESSES_SLUG_STR, THROUGHPUT_NAME_STR, THROUGHPUT_SLUG_STR,
    },
    JsonMetricKind, JsonNewMetricKind, NonEmpty, ResourceId, Slug,
};
use chrono::Utc;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl};
use uuid::Uuid;

use crate::{
    context::DbConnection,
    error::api_error,
    model::project::QueryProject,
    schema,
    schema::metric_kind as metric_kind_table,
    util::{
        query::{fn_get, fn_get_id},
        resource_id::fn_resource_id,
        slug::unwrap_child_slug,
        to_date_time,
    },
    ApiError,
};

fn_resource_id!(metric_kind);

#[derive(Queryable)]
pub struct QueryMetricKind {
    pub id: i32,
    pub uuid: String,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub units: String,
    pub created: i64,
    pub modified: i64,
}

impl QueryMetricKind {
    fn_get!(metric_kind);
    fn_get_id!(metric_kind);

    pub fn get_uuid(conn: &mut DbConnection, id: i32) -> Result<Uuid, ApiError> {
        let uuid: String = schema::metric_kind::table
            .filter(schema::metric_kind::id.eq(id))
            .select(schema::metric_kind::uuid)
            .first(conn)
            .map_err(api_error!())?;
        Uuid::from_str(&uuid).map_err(api_error!())
    }

    pub fn from_resource_id(
        conn: &mut DbConnection,
        project_id: i32,
        metric_kind: &ResourceId,
    ) -> Result<Self, ApiError> {
        schema::metric_kind::table
            .filter(schema::metric_kind::project_id.eq(project_id))
            .filter(resource_id(metric_kind)?)
            .first::<QueryMetricKind>(conn)
            .map_err(api_error!())
    }

    pub fn into_json(self, conn: &mut DbConnection) -> Result<JsonMetricKind, ApiError> {
        let Self {
            uuid,
            project_id,
            name,
            slug,
            units,
            created,
            modified,
            ..
        } = self;
        Ok(JsonMetricKind {
            uuid: Uuid::from_str(&uuid).map_err(api_error!())?,
            project: QueryProject::get_uuid(conn, project_id)?,
            name: NonEmpty::from_str(&name).map_err(api_error!())?,
            slug: Slug::from_str(&slug).map_err(api_error!())?,
            units: NonEmpty::from_str(&units).map_err(api_error!())?,
            created: to_date_time(created).map_err(api_error!())?,
            modified: to_date_time(modified).map_err(api_error!())?,
        })
    }

    pub fn get_or_create(
        conn: &mut DbConnection,
        project_id: i32,
        metric_kind: &ResourceId,
    ) -> Result<i32, ApiError> {
        let query_metric_kind = Self::from_resource_id(conn, project_id, metric_kind);

        let api_error = match query_metric_kind {
            Ok(query) => return Ok(query.id),
            Err(e) => e,
        };

        // Dynamically create adapter specific metric kinds
        // Or recreate default metric kinds if they were previously deleted
        let insert_metric_kind = match metric_kind.as_ref() {
            // Recreate
            LATENCY_SLUG_STR => InsertMetricKind::latency(conn, project_id),
            THROUGHPUT_SLUG_STR => InsertMetricKind::throughput(conn, project_id),
            // Adapter specific
            INSTRUCTIONS_SLUG_STR => InsertMetricKind::instructions(conn, project_id),
            L1_ACCESSES_SLUG_STR => InsertMetricKind::l1_accesses(conn, project_id),
            L2_ACCESSES_SLUG_STR => InsertMetricKind::l2_accesses(conn, project_id),
            RAM_ACCESSES_SLUG_STR => InsertMetricKind::ram_accesses(conn, project_id),
            ESTIMATED_CYCLES_SLUG_STR => InsertMetricKind::estimated_cycles(conn, project_id),
            _ => return Err(api_error),
        };
        diesel::insert_into(schema::metric_kind::table)
            .values(&insert_metric_kind)
            .execute(conn)
            .map_err(api_error!())?;

        Self::get_id(conn, &insert_metric_kind.uuid)
    }

    pub fn is_system(&self) -> bool {
        is_system(self.name.as_ref(), self.slug.as_ref())
    }
}

#[derive(Insertable)]
#[diesel(table_name = metric_kind_table)]
pub struct InsertMetricKind {
    pub uuid: String,
    pub project_id: i32,
    pub name: String,
    pub slug: String,
    pub units: String,
    pub created: i64,
    pub modified: i64,
}

impl InsertMetricKind {
    pub fn from_json(
        conn: &mut DbConnection,
        project: &ResourceId,
        metric_kind: JsonNewMetricKind,
    ) -> Result<Self, ApiError> {
        let project_id = QueryProject::from_resource_id(conn, project)?.id;
        Ok(Self::from_json_inner(conn, project_id, metric_kind))
    }

    pub fn latency(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::latency())
    }

    pub fn throughput(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::throughput())
    }

    pub fn instructions(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::instructions())
    }

    pub fn l1_accesses(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::l1_accesses())
    }

    pub fn l2_accesses(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::l2_accesses())
    }

    pub fn ram_accesses(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::ram_accesses())
    }

    pub fn estimated_cycles(conn: &mut DbConnection, project_id: i32) -> Self {
        Self::from_json_inner(conn, project_id, JsonNewMetricKind::estimated_cycles())
    }

    fn from_json_inner(
        conn: &mut DbConnection,
        project_id: i32,
        metric_kind: JsonNewMetricKind,
    ) -> Self {
        let JsonNewMetricKind { name, slug, units } = metric_kind;
        let slug = unwrap_child_slug!(
            conn,
            project_id,
            name.as_ref(),
            slug,
            metric_kind,
            QueryMetricKind
        );
        let timestamp = Utc::now().timestamp();
        Self {
            uuid: Uuid::new_v4().to_string(),
            project_id,
            name: name.into(),
            slug,
            units: units.into(),
            created: timestamp,
            modified: timestamp,
        }
    }

    pub fn is_system(&self) -> bool {
        is_system(self.name.as_ref(), self.slug.as_ref())
    }
}

fn is_system(name: &str, slug: &str) -> bool {
    matches!(
        name,
        LATENCY_NAME_STR
            | THROUGHPUT_NAME_STR
            | INSTRUCTIONS_NAME_STR
            | L1_ACCESSES_NAME_STR
            | L2_ACCESSES_NAME_STR
            | RAM_ACCESSES_NAME_STR
            | ESTIMATED_CYCLES_NAME_STR
    ) || matches!(
        slug,
        LATENCY_SLUG_STR
            | THROUGHPUT_SLUG_STR
            | INSTRUCTIONS_SLUG_STR
            | L1_ACCESSES_SLUG_STR
            | L2_ACCESSES_SLUG_STR
            | RAM_ACCESSES_SLUG_STR
            | ESTIMATED_CYCLES_SLUG_STR
    )
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = metric_kind_table)]
pub struct UpdateMetricKind {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub units: Option<String>,
    pub modified: i64,
}

impl From<JsonUpdateMetricKind> for UpdateMetricKind {
    fn from(update: JsonUpdateMetricKind) -> Self {
        let JsonUpdateMetricKind { name, slug, units } = update;
        Self {
            name: name.map(Into::into),
            slug: slug.map(Into::into),
            units: units.map(Into::into),
            modified: Utc::now().timestamp(),
        }
    }
}
