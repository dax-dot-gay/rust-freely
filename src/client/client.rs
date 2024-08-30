pub mod api_client {
    use serde_derive::{Deserialize, Serialize};

    use crate::{api_models, api_wrapper::Api};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum Auth {
        Token(String),
        Login{username: String, password: String}
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct RequestError {
        pub code: u16,
        pub reason: Option<String>
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum ApiError {
        Request{error: RequestError},
        AuthenticationError{},
        UnknownError{},
        UrlError{},
        ParseError{text: String},
        ConnectionError{}
    }


    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Client {
        _base_url: String,
        _token: Option<String>,
    }

    impl Client {
        pub fn new(base: String) -> Self {
            Client { _base_url: base, _token: None }
        }

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

        pub fn url(&self) -> String {
            self._base_url.clone()
        }

        pub fn token(&self) -> Option<String> {
            self._token.clone()
        }

        pub fn is_authenticated(&self) -> bool {
            self._token.is_some()
        }

        pub fn api(&self) -> Api {
            Api::new(self.clone())
        }
    }
}

#[cfg(test)]
mod tests {
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

}