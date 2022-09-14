use std::str::FromStr;
use std::string::ToString;

use bencher_json::{JsonNewOrganization, JsonOrganization, ResourceId};
use bencher_rbac::Organization;
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, SqliteConnection};
use uuid::Uuid;

use super::user::InsertUser;
use crate::{
    error::api_error,
    schema::{self, organization as organization_table},
    util::{resource_id::fn_resource_id, slug::unwrap_slug},
    ApiError,
};

#[derive(Insertable)]
#[diesel(table_name = organization_table)]
pub struct InsertOrganization {
    pub uuid: String,
    pub name: String,
    pub slug: String,
}

impl InsertOrganization {
    pub fn from_json(conn: &mut SqliteConnection, organization: JsonNewOrganization) -> Self {
        let JsonNewOrganization { name, slug } = organization;
        let slug = unwrap_slug!(conn, &name, slug, organization, QueryOrganization);
        Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            slug,
        }
    }

    pub fn from_user(insert_user: &InsertUser) -> Self {
        Self {
            uuid: Uuid::new_v4().to_string(),
            name: insert_user.name.clone(),
            slug: insert_user.slug.clone(),
        }
    }
}

fn_resource_id!(organization);

#[derive(Debug, Clone, Queryable)]
pub struct QueryOrganization {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub slug: String,
}

impl QueryOrganization {
    pub fn get_id(conn: &mut SqliteConnection, uuid: impl ToString) -> Result<i32, ApiError> {
        schema::organization::table
            .filter(schema::organization::uuid.eq(uuid.to_string()))
            .select(schema::organization::id)
            .first(conn)
            .map_err(api_error!())
    }

    pub fn get_uuid(conn: &mut SqliteConnection, id: i32) -> Result<Uuid, ApiError> {
        let uuid: String = schema::organization::table
            .filter(schema::organization::id.eq(id))
            .select(schema::organization::uuid)
            .first(conn)
            .map_err(api_error!())?;
        Uuid::from_str(&uuid).map_err(api_error!())
    }

    pub fn from_resource_id(
        conn: &mut SqliteConnection,
        organization: &ResourceId,
    ) -> Result<Self, ApiError> {
        schema::organization::table
            .filter(resource_id(organization)?)
            .first::<QueryOrganization>(conn)
            .map_err(api_error!())
    }

    pub fn into_json(self) -> Result<JsonOrganization, ApiError> {
        let Self {
            id: _,
            uuid,
            name,
            slug,
        } = self;
        Ok(JsonOrganization {
            uuid: Uuid::from_str(&uuid).map_err(api_error!())?,
            name,
            slug,
        })
    }
}

impl From<&QueryOrganization> for Organization {
    fn from(organization: &QueryOrganization) -> Self {
        Organization {
            id: organization.id.to_string(),
        }
    }
}
