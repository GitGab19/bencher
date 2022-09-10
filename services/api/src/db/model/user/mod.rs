use std::str::FromStr;

use bencher_json::{auth::JsonInvite, jwt::JsonWebToken, JsonSignup, JsonUser, ResourceId};
use diesel::{
    expression_methods::BoolExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl,
    SqliteConnection,
};
use dropshot::{HttpError, RequestContext};
use email_address_parser::EmailAddress;
use uuid::Uuid;

use crate::{
    diesel::ExpressionMethods,
    schema::{self, user as user_table},
    util::{http_error, Context},
};

pub mod organization;
pub mod project;
pub mod token;

const USER_ERROR: &str = "Failed to get user.";

#[derive(Insertable)]
#[diesel(table_name = user_table)]
pub struct InsertUser {
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub admin: bool,
    pub locked: bool,
}

impl InsertUser {
    pub fn from_json(conn: &mut SqliteConnection, signup: JsonSignup) -> Result<Self, HttpError> {
        let JsonSignup {
            name,
            slug,
            email,
            invite: _,
        } = signup;
        validate_email(&email)?;
        let slug = validate_slug(conn, &name, slug);
        Ok(Self {
            uuid: Uuid::new_v4().to_string(),
            name,
            slug,
            email,
            admin: false,
            locked: false,
        })
    }
}

fn validate_slug(conn: &mut SqliteConnection, name: &str, slug: Option<String>) -> String {
    let mut slug = slug
        .map(|s| {
            if s == slug::slugify(&s) {
                s
            } else {
                slug::slugify(name)
            }
        })
        .unwrap_or_else(|| slug::slugify(name));

    if schema::user::table
        .filter(schema::user::slug.eq(&slug))
        .first::<QueryUser>(conn)
        .is_ok()
    {
        let rand_suffix = rand::random::<u32>().to_string();
        slug.push_str(&rand_suffix);
        slug
    } else {
        slug
    }
}

fn validate_email(email: &str) -> Result<EmailAddress, HttpError> {
    EmailAddress::parse(email, None).ok_or(http_error!(USER_ERROR))
}

#[derive(Queryable)]
pub struct QueryUser {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub slug: String,
    pub email: String,
    pub admin: bool,
    pub locked: bool,
}

impl QueryUser {
    pub fn get_id(conn: &mut SqliteConnection, uuid: impl ToString) -> Result<i32, HttpError> {
        schema::user::table
            .filter(schema::user::uuid.eq(uuid.to_string()))
            .select(schema::user::id)
            .first(conn)
            .map_err(|_| http_error!(USER_ERROR))
    }

    pub fn get_uuid(conn: &mut SqliteConnection, id: i32) -> Result<Uuid, HttpError> {
        let uuid: String = schema::user::table
            .filter(schema::user::id.eq(id))
            .select(schema::user::uuid)
            .first(conn)
            .map_err(|_| http_error!(USER_ERROR))?;
        Uuid::from_str(&uuid).map_err(|_| http_error!(USER_ERROR))
    }

    pub fn get_id_from_email(conn: &mut SqliteConnection, email: &str) -> Result<i32, HttpError> {
        schema::user::table
            .filter(schema::user::email.eq(email))
            .select(schema::user::id)
            .first(conn)
            .map_err(|_| http_error!(USER_ERROR))
    }

    pub fn get_email_from_id(conn: &mut SqliteConnection, id: i32) -> Result<String, HttpError> {
        schema::user::table
            .filter(schema::user::id.eq(id))
            .select(schema::user::email)
            .first(conn)
            .map_err(|_| http_error!(USER_ERROR))
    }

    pub fn from_resource_id(
        conn: &mut SqliteConnection,
        user: &ResourceId,
    ) -> Result<Self, HttpError> {
        let user = &user.0;
        schema::user::table
            .filter(schema::user::slug.eq(user).or(schema::user::uuid.eq(user)))
            .first(conn)
            .map_err(|_| http_error!(USER_ERROR))
    }

    pub fn to_json(self) -> Result<JsonUser, HttpError> {
        let Self {
            id: _,
            uuid,
            name,
            slug,
            email,
            admin,
            locked,
        } = self;
        Ok(JsonUser {
            uuid: Uuid::from_str(&uuid).map_err(|_| http_error!("Failed to get user."))?,
            name,
            slug,
            email,
            admin,
            locked,
        })
    }

    pub async fn auth(rqctx: &RequestContext<Context>) -> Result<i32, HttpError> {
        let request = rqctx.request.lock().await;

        let headers = request
            .headers()
            .get("Authorization")
            .ok_or(http_error!("Missing \"Authorization\" header."))?
            .to_str()
            .map_err(|_| http_error!("Invalid \"Authorization\" header."))?;
        let (_, token) = headers
            .split_once("Bearer ")
            .ok_or(http_error!("Missing \"Authorization\" Bearer."))?;
        let jwt: JsonWebToken = token.to_string().into();

        const INVALID_JWT: &str = "Invalid JWT (JSON Web Token).";

        let context = &mut *rqctx.context().lock().await;
        let token_data = jwt
            .validate_user(&context.key)
            .map_err(|_| http_error!(INVALID_JWT))?;

        let conn = &mut context.db;
        schema::user::table
            .filter(schema::user::email.eq(token_data.claims.email()))
            .select(schema::user::id)
            .first::<i32>(conn)
            .map_err(|_| http_error!(INVALID_JWT))
    }

    pub fn has_access(
        conn: &mut SqliteConnection,
        user_id: i32,
        project_id: i32,
    ) -> Result<(), HttpError> {
        // TODO check with `bencher_rbac`
        // schema::project::table
        //     .filter(
        //         schema::project::id
        //             .eq(project_id)
        //             .and(schema::project::owner_id.eq(user_id)),
        //     )
        //     .select(schema::project::id)
        //     .first::<i32>(conn)
        //     .map_err(|_| http_error!("Access denied."))?;

        Ok(())
    }
}
