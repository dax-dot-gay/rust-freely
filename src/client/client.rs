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
        ParseError{},
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