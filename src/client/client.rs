/// This module contains the main [Client] struct, which provides access to all of the other types & methods.
pub mod api_client {
    use serde_derive::{Deserialize, Serialize};

    use crate::{api_handlers::{CollectionHandler, PostHandler, UserHandler}, api_models, api_wrapper::Api};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    /// The desired authentication method
    pub enum Auth {
        /// Authenticate with an API token
        Token(String),

        /// Authenticate with a username and password
        Login{
            /// Login username
            username: String, 

            /// Login password
            password: String
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    /// Represents a request error (see [ApiError])
    pub struct RequestError {
        /// Error code (HTTP status)
        pub code: u16,

        /// Optional result information
        pub reason: Option<String>
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    /// The main Error enum for this library
    pub enum ApiError {
        /// Raised if the API returns a non-success status code
        Request{
            /// RequestError instance
            error: RequestError
        },

        /// Raised if authentication fails
        AuthenticationError{},

        /// Raised on an unexpected error. Should never appear in normal operation
        UnknownError{},

        /// Raised if URL creation fails
        UrlError{},

        /// Raised if data parsing fails
        ParseError{
            /// Text that serde failed to parse
            text: String
        },

        /// Raised if connecting to the API server fails
        ConnectionError{},

        /// Raised if an action cannot be performed when logged out
        LoggedOut{},

        /// Raised if invalid data was passed from the user, or if no [Client] instance is defined on the referenced struct
        UsageError{}
    }


    #[derive(Clone, Serialize, Deserialize, Debug)]
    /// Main Client struct
    pub struct Client {
        _base_url: String,
        _token: Option<String>,
    }

    impl Client {
        /// Creates a new client with a base URL
        pub fn new(base: String) -> Self {
            Client { _base_url: base, _token: None }
        }

        /// Authenticates with an [Auth] enum value
        pub async fn authenticate(&mut self, auth: Auth) -> Result<Self, ApiError> {
            match auth {
                Auth::Token(token) => {
                    self._token = Some(token);
                    Ok(self.clone())
                },
                Auth::Login { username, password } => {
                    match self.api().post::<api_models::responses::Login, _>("/auth/login", Some(api_models::requests::Login {alias: username, pass: password})).await {
                        Ok(data) => {
                            self._token = Some(data.access_token);
                            Ok(self.clone())
                        },
                        Err(e) => Err(e)
                    }
                }
            }
        }

        /// Deauthenticates from the server
        pub async fn logout(&mut self) -> Result<Self, ApiError> {
            if self.is_authenticated() {
                match self.api().delete("/auth/me").await {
                    Ok(_) => {
                        self._token = None;
                        Ok(self.clone())
                    },
                    Err(e) => Err(e)
                }
            } else {
                Err(ApiError::LoggedOut {  })
            }
        }

        /// Retrieves the base URL
        pub fn url(&self) -> String {
            self._base_url.clone()
        }

        /// Retrieves the access token
        pub fn token(&self) -> Option<String> {
            self._token.clone()
        }

        /// Checks if the instance is authenticated
        pub fn is_authenticated(&self) -> bool {
            self._token.is_some()
        }

        /// Returns a new [Api] instance. In general, a new instance should be created for each separate operation to prevent cloned [Client] desync.
        pub fn api(&self) -> Api {
            Api::new(self.clone())
        }

        /// Returns a wrapper around User methods
        pub async fn user(&self) -> Result<UserHandler, ApiError> {
            if self.is_authenticated() {
                Ok(UserHandler::new(self.clone()).await)
            } else {
                Err(ApiError::LoggedOut {  })
            }
        }

        /// Returns a wrapper around Post methods
        pub fn posts(&self) -> PostHandler {
            PostHandler::new(self.clone())
        }

        /// Returns a wrapper around Collection methods
        pub fn collections(&self) -> CollectionHandler {
            CollectionHandler::new(self.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    use api_client::{Auth, Client};
    use tokio_test;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    fn anon() -> Client {
        Client::new("http://0.0.0.0:8080".to_string())
    }

    async fn auth() -> Client {
        Client::new("http://0.0.0.0:8080".to_string()).authenticate(Auth::Login { username: "username".to_string(), password: "password".to_string() }).await.unwrap()
    }

    #[test]
    fn eq_url() {
        assert_eq!(anon().url(), "http://0.0.0.0:8080".to_string());
    }

    #[test]
    fn anon_no_token() {
        assert!(!anon().is_authenticated());
    }

    #[test]
    fn auth_has_token() {
        assert!(aw!(auth()).is_authenticated())
    }

    #[test]
    fn auth_bad_login() {
        assert!(aw!(anon().authenticate(Auth::Login { username: "usernameee".to_string(), password: "passwordeee".to_string() })).is_err())
    }

    #[test]
    fn auth_logout() {
        let mut authed = aw!(auth());
        println!("{:?}", authed.clone().token());
        sleep(Duration::from_secs(2));
        let logged_out = aw!(authed.logout());
        
        assert!(!logged_out.unwrap().is_authenticated());
    }

}