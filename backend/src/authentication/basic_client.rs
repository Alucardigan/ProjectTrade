use std::env;

use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, EndpointNotSet, EndpointSet, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RevocationErrorResponseType, Scope, StandardErrorResponse, StandardRevocableToken,
    StandardTokenIntrospectionResponse, StandardTokenResponse, TokenUrl,
};
use url::Url;

use crate::models::authentication::Auth0UserInfo;
use crate::models::errors::user_error::UserError;
//got this type from the compiler itself
pub struct AuthorizationClient {
    client: Client<
        StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>,
        EndpointSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointNotSet,
        EndpointSet,
    >,
    http_client: reqwest::Client,
}

pub struct Auth0LoginResponse {
    pub auth_url: Url,
    pub pkce_code_verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
}

impl AuthorizationClient {
    pub fn new() -> AuthorizationClient {
        let client = BasicClient::new(ClientId::new(env::var("AUTH_CLIENT_ID").unwrap()))
            .set_client_secret(ClientSecret::new(env::var("AUTH_CLIENT_SECRET").unwrap()))
            .set_auth_uri(AuthUrl::new(env::var("AUTH_DOMAIN").unwrap() + "/authorize").unwrap())
            .set_token_uri(
                TokenUrl::new(env::var("AUTH_DOMAIN").unwrap() + "/oauth/token").unwrap(),
            )
            .set_redirect_uri(
                RedirectUrl::new(env::var("FRONTEND_URL").unwrap() + "/auth_callback").unwrap(),
            );
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");
        AuthorizationClient {
            client,
            http_client,
        }
    }

    pub async fn auth0_login(&self) -> Auth0LoginResponse {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scopes([
                Scope::new("openid".to_string()),
                Scope::new("profile".to_string()),
                Scope::new("email".to_string()),
            ])
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        return Auth0LoginResponse {
            auth_url,
            pkce_code_verifier,
            csrf_token,
        };
    }
    pub async fn auth0_callback(
        &self,
        code: String,
        state_token: String,
        pkce_code_verifier: PkceCodeVerifier,
        csrf_token: CsrfToken,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, UserError> {
        if state_token != csrf_token.secret().to_string() {
            return Err(UserError::CSRFMismatch);
        }
        let token_response = match self
            .client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(&self.http_client)
            .await
        {
            Ok(res) => res,
            Err(e) => {
                return Err(UserError::TokenExchange(e));
            }
        };
        Ok(token_response)
    }
    pub async fn get_user_info(
        &self,
        access_token: &AccessToken,
    ) -> Result<Auth0UserInfo, UserError> {
        let response = self
            .http_client
            .get(env::var("AUTH_DOMAIN").expect("AUTH_DOMAIN not set") + "/userinfo")
            .bearer_auth(access_token.secret())
            .send()
            .await?;
        let user_info = response.json::<Auth0UserInfo>().await?;
        Ok(user_info)
    }
}
