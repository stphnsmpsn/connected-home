use self::super::super::schema;
use crate::user::User;
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use grpc::user::user_service_server::UserService;
use grpc::user::{
    LoginRequest, LoginResponse, Profile, ProfileRequest, ProfileResponse, RegisterRequest,
    RegisterResponse,
};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use schema::users::dsl::*;
use sha2::Sha256;
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use tonic::{Code, Request, Response, Status};
use tracing::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use types::jwt::Jwt;

// defining a struct for our service
pub struct MyUserService {
    db: Arc<Mutex<PgConnection>>,
}

impl Debug for MyUserService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is a test")
    }
}

impl MyUserService {
    pub fn new(db: Arc<Mutex<PgConnection>>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    #[instrument]
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        tracing::info!("Got Register Request");

        let x = request.metadata();
        let y = x.clone().into_headers();

        let mut fields: HashMap<_, _> = HashMap::new();
        for (k, v) in y {
            fields.insert(k.unwrap().to_string(), v.to_str().unwrap().to_string());
        }

        let propagator = TraceContextPropagator::new();
        let context = propagator.extract(&fields);
        let span = tracing::Span::current();
        span.set_parent(context);

        let new_user = request
            .into_inner()
            .credentials
            //.ok_or_else(|| Err(Status::new(Code::InvalidArgument, "")))?;
            .unwrap();

        tracing::info!(
            "Registering user: {} with password: {}",
            new_user.username,
            new_user.password
        );

        // todo: remove unwrap
        let db = self.db.lock().unwrap();

        let results = users
            .filter(username.eq(new_user.username.clone()))
            .limit(1)
            .load::<User>(db.deref())
            .expect("Error querying user");

        if !results.is_empty() {
            return Err(Status::new(
                Code::InvalidArgument,
                "A user with that name already exists.",
            ));
        }

        let hashed_user = User::new(new_user.username, new_user.password);

        // TODO: handle error saving new user
        diesel::insert_into(users)
            .values(&hashed_user)
            .execute(db.deref())
            .unwrap();

        Ok(Response::new(RegisterResponse {
            jwt: create_jwt(hashed_user.username()).to_string(),
        }))
    }

    #[instrument]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        tracing::info!("Got Login Request");

        // todo: remove unwrap
        let credentials = request.into_inner().credentials.unwrap();

        // todo: remove unwrap
        let db = self.db.lock().unwrap();

        let results = users
            .filter(username.eq(credentials.username.clone()))
            .limit(1)
            .load::<User>(db.deref())
            .expect("Error querying user");

        if let Some(user) = results.get(0) {
            return if user.verify_password(credentials.password) {
                Ok(Response::new(LoginResponse {
                    jwt: create_jwt(user.username()).to_string(),
                }))
            } else {
                Err(Status::new(
                    Code::Unauthenticated,
                    "Failed to authenticate with provided credentials",
                ))
            };
        }

        Err(Status::new(
            Code::Unauthenticated,
            "Failed to authenticate with provided credentials",
        ))
    }

    #[instrument]
    async fn profile(
        &self,
        request: Request<ProfileRequest>,
    ) -> Result<Response<ProfileResponse>, Status> {
        tracing::info!("Got Profile Request");

        match request.metadata().get("authorization") {
            Some(token) => {
                let jwt: Jwt = serde_json::from_str(token.to_str().unwrap()).unwrap();
                // todo: manage secret
                let claims = jwt.verify("SUPERSECRETKEY");
                match claims {
                    Ok(_claims) => Ok(Response::new(ProfileResponse {
                        profile: Some(Profile {
                            first_name: "John".to_string(),
                            last_name: "Doe".to_string(),
                            street_number: 123,
                            street: "My Street".to_string(),
                            city: "Any Town".to_string(),
                            postal_code: "90210".to_string(),
                        }),
                    })),
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        Err(Status::unauthenticated("No valid auth token"))
                    }
                }
            }
            _ => Err(Status::unauthenticated("No valid auth token")),
        }
    }
}

#[instrument]
pub fn create_jwt(user: &str) -> Jwt {
    // todo: manage secrets
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"SUPERSECRETKEY").unwrap();

    let mut claims = BTreeMap::<String, String>::new();
    claims.insert(String::from("username"), user.to_string());
    // todo: choose appropriate expiry and implement a token refresh
    let expiry = Utc::now() + Duration::minutes(10);
    claims.insert(
        String::from("expiry"),
        format!("{}", expiry.format("%Y-%m-%d %H:%M:%S %z")),
    );

    let token = claims
        .sign_with_key(&key)
        .expect("If this fails, we have an algorithm mismatch between token header and key");

    Jwt { token }
}
