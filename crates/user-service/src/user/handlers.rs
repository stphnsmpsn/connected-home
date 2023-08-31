use crate::{context::Context, repo::Repo};
use argon2::Config;
use chrono::{Duration, Utc};
use common::{auth::jwt::Jwt, error::ConnectedHomeError};
use grpc::user::{
    user_service_server::UserService, LoginRequest, LoginResponse, Profile, ProfileRequest, ProfileResponse,
    RegisterRequest, RegisterResponse,
};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use rand::Rng;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    sync::Arc,
};
use tonic::{Code, Request, Response, Status};
use tracing::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub struct MyUserService {
    context: Arc<Context>,
}

impl Debug for MyUserService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is a test")
    }
}

impl MyUserService {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    #[instrument]
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        if let Some(request_context) = request.extensions().get::<grpc::RequestContext>() {
            let span = tracing::Span::current();
            span.set_parent(request_context.to_owned().tracing_context);
        }

        let user_credentials = request.into_inner().credentials;

        let Some(new_user) = user_credentials else {
            return Err(Status::new(Code::InvalidArgument, ""));
        };

        // let hashed_user = User::new(new_user.username, new_user.password);

        let result = self
            .context
            .get_db_conn()
            .await
            .unwrap()
            .store_user(new_user.username, hash(new_user.password.as_bytes()))
            .await;

        match result {
            Ok(user) => Ok(Response::new(RegisterResponse {
                jwt: create_jwt(user.username.as_str()).to_string(),
            })),
            Err(ConnectedHomeError::UserAlreadyExists(_)) => Err(Status::new(
                Code::AlreadyExists,
                "A user with that name already exists.",
            )),
            _ => Err(Status::new(
                Code::Internal,
                "Failed to register user. Please try again later.",
            )),
        }
    }

    #[instrument]
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        if let Some(request_context) = request.extensions().get::<grpc::RequestContext>() {
            let span = tracing::Span::current();
            span.set_parent(request_context.to_owned().tracing_context);
        }

        let Some(credentials) = request.into_inner().credentials else {
            return Err(Status::new(Code::InvalidArgument, ""));
        };

        let user_dto = self
            .context
            .get_db_conn()
            .await
            .unwrap()
            .load_user(credentials.username)
            .await
            .unwrap()
            .unwrap();

        if user_dto.verify_password(credentials.password.as_str()) {
            return Ok(Response::new(LoginResponse {
                jwt: create_jwt(user_dto.username.as_str()).to_string(),
            }));
        }

        Err(Status::new(
            Code::Unauthenticated,
            "Failed to authenticate with provided credentials",
        ))
    }

    #[instrument]
    async fn profile(&self, request: Request<ProfileRequest>) -> Result<Response<ProfileResponse>, Status> {
        if let Some(request_context) = request.extensions().get::<grpc::RequestContext>() {
            let span = tracing::Span::current();
            span.set_parent(request_context.to_owned().tracing_context);
        }

        match request.metadata().get("authorization") {
            Some(token) => {
                tracing::info!("got token: {}", token.to_str().unwrap());
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

#[tracing::instrument]
fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
