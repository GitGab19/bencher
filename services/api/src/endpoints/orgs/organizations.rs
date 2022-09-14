use std::sync::Arc;

use bencher_json::{JsonNewOrganization, JsonOrganization, ResourceId};
use bencher_rbac::organization::{Permission, Role};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use dropshot::{endpoint, HttpError, Path, RequestContext, TypedBody};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    endpoints::{
        endpoint::{response_accepted, response_ok, ResponseAccepted, ResponseOk},
        Endpoint, Method,
    },
    error::api_error,
    model::{
        organization::{InsertOrganization, QueryOrganization},
        user::{auth::AuthUser, organization::InsertOrganizationRole},
    },
    schema,
    util::{
        cors::{get_cors, CorsResponse},
        Context,
    },
    ApiError,
};

use super::Resource;

const ORGANIZATION_RESOURCE: Resource = Resource::Organization;

#[endpoint {
    method = OPTIONS,
    path =  "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn dir_options(_rqctx: Arc<RequestContext<Context>>) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path = "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn get_ls(
    rqctx: Arc<RequestContext<Context>>,
) -> Result<ResponseOk<Vec<JsonOrganization>>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::GetLs);

    let json = get_ls_inner(&auth_user, rqctx.context())
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_ls_inner(
    auth_user: &AuthUser,
    context: &Context,
) -> Result<Vec<JsonOrganization>, ApiError> {
    let context = &mut *context.lock().await;
    let conn = &mut context.db_conn;
    let organizations = auth_user.organizations(&context.rbac, Permission::View);

    let json: Vec<JsonOrganization> = schema::organization::table
        .filter(schema::organization::id.eq_any(organizations))
        .order(schema::organization::name)
        .load::<QueryOrganization>(conn)
        .map_err(api_error!())?
        .into_iter()
        .filter_map(|query| query.into_json().ok())
        .collect();

    Ok(json)
}

#[endpoint {
    method = POST,
    path = "/v0/organizations",
    tags = ["organizations"]
}]
pub async fn post(
    rqctx: Arc<RequestContext<Context>>,
    body: TypedBody<JsonNewOrganization>,
) -> Result<ResponseAccepted<JsonOrganization>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::Post);

    let json = post_inner(&auth_user, rqctx.context(), body.into_inner())
        .await
        .map_err(|e| endpoint.err(e))?;

    response_accepted!(endpoint, json)
}

async fn post_inner(
    auth_user: &AuthUser,
    context: &Context,
    json_organization: JsonNewOrganization,
) -> Result<JsonOrganization, ApiError> {
    let context = &mut *context.lock().await;
    let conn = &mut context.db_conn;

    // Create the organization
    let insert_organization = InsertOrganization::from_json(conn, json_organization);
    diesel::insert_into(schema::organization::table)
        .values(&insert_organization)
        .execute(conn)
        .map_err(api_error!())?;
    let query_organization = schema::organization::table
        .filter(schema::organization::uuid.eq(&insert_organization.uuid))
        .first::<QueryOrganization>(conn)
        .map_err(api_error!())?;

    // Connect the user to the organization as a `Maintainer`
    let insert_org_role = InsertOrganizationRole {
        user_id: auth_user.id,
        organization_id: query_organization.id,
        role: Role::Leader.to_string(),
    };
    diesel::insert_into(schema::organization_role::table)
        .values(&insert_org_role)
        .execute(conn)
        .map_err(api_error!())?;

    query_organization.into_json()
}

#[derive(Deserialize, JsonSchema)]
pub struct GetOneParams {
    pub organization: ResourceId,
}

#[endpoint {
    method = OPTIONS,
    path =  "/v0/organizations/{organization}",
    tags = ["organizations"]
}]
pub async fn one_options(
    _rqctx: Arc<RequestContext<Context>>,
    _path_params: Path<GetOneParams>,
) -> Result<CorsResponse, HttpError> {
    Ok(get_cors::<Context>())
}

#[endpoint {
    method = GET,
    path = "/v0/organizations/{organization}",
    tags = ["organizations"]
}]
pub async fn get_one(
    rqctx: Arc<RequestContext<Context>>,
    path_params: Path<GetOneParams>,
) -> Result<ResponseOk<JsonOrganization>, HttpError> {
    let auth_user = AuthUser::new(&rqctx).await?;
    let endpoint = Endpoint::new(ORGANIZATION_RESOURCE, Method::GetOne);

    let json = get_one_inner(&auth_user, rqctx.context(), path_params.into_inner())
        .await
        .map_err(|e| endpoint.err(e))?;

    response_ok!(endpoint, json)
}

async fn get_one_inner(
    auth_user: &AuthUser,
    context: &Context,
    path_params: GetOneParams,
) -> Result<JsonOrganization, ApiError> {
    let context = &mut *context.lock().await;
    let conn = &mut context.db_conn;

    let query = QueryOrganization::from_resource_id(conn, &path_params.organization)?;
    context
        .rbac
        .is_allowed_organization(auth_user, Permission::View, &query)?;

    query.into_json()
}
