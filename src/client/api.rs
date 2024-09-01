/// Provides convenience functions for HTTP requests & serialization
pub mod api_wrapper {
    use std::fmt::Debug;

    use reqwest::{header, Client as ReqwestClient, Error, Method, RequestBuilder, Response, Url};
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{
        api_client::{ApiError, Client, RequestError},
        api_models::responses::ResponseModel,
    };

    #[derive(Clone, Debug)]
    /// Wrapper struct for API, implements all API methods. Generally not useful for clients.
    pub struct Api {
        client: Client,
    }

    impl Api {
        /// Creates a new Api instance with a passed [Client]
        pub fn new(client: Client) -> Self {
            Api { client }
        }

        /// Fetches the base URL from the [Client] instance
        pub fn base(&self) -> String {
            self.client.url()
        }

        /// Fetches the API token from the [Client] instance
        pub fn token(&self) -> Option<String> {
            self.client.token()
        }

        /// Determines if the current session is authenticated
        pub fn is_authenticated(&self) -> bool {
            self.client.is_authenticated()
        }

        /// Assembles an API url from the base url and an endpoint.
        pub fn url(&self, endpoint: &str) -> Result<Url, ApiError> {
            if let Ok(result) = Url::parse(self.base().as_str()) {
                if let Ok(result) = result.join(vec!["/api", endpoint].join("").as_str()) {
                    Ok(result)
                } else {
                    Err(ApiError::UrlError {})
                }
            } else {
                Err(ApiError::UrlError {})
            }
        }

        fn http(&self) -> Result<ReqwestClient, Error> {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Accept",
                header::HeaderValue::from_static("application/json"),
            );
            headers.insert(
                "Content-Type",
                header::HeaderValue::from_static("application/json"),
            );

            ReqwestClient::builder().default_headers(headers).build()
        }

        /// Assembles a request builder with default settings
        pub fn request(&self, endpoint: &str, method: Method) -> Result<RequestBuilder, ApiError> {
            if let Ok(http) = self.http() {
                if let Ok(url) = self.url(endpoint) {
                    let mut request = http.request(method, url.clone());
                    println!("{:?}", url);
                    if let Some(token) = self.token() {
                        request = request.header(header::AUTHORIZATION, format!("Token {token}"));
                    }
                    Ok(request)
                } else {
                    Err(ApiError::UrlError {})
                }
            } else {
                Err(ApiError::UnknownError {})
            }
        }

        /// Extracts a reponse with serde
        pub async fn extract_response<T: DeserializeOwned + Debug>(
            &self,
            response: Response,
        ) -> Result<T, ApiError> {
            match response.error_for_status() {
                Ok(resp) => {
                    let text = resp.text().await.unwrap();
                    serde_json::from_str::<ResponseModel>(text.clone().as_str())
                        .or(Err(ApiError::ParseError {
                            text: text.clone(),
                        }))
                        .and_then(|v| {
                            serde_json::from_value::<T>(v.data).or(Err(ApiError::ParseError {
                                text: text.clone(),
                            }))
                        })
                }
                Err(resp) => Err(ApiError::Request {
                    error: RequestError {
                        code: resp.status().map_or(0, |s| s.as_u16()),
                        reason: Some(resp.to_string()),
                    },
                }),
            }
        }

        /// Executes a GET request.
        pub async fn get<T: DeserializeOwned + Debug>(
            &self,
            endpoint: &str,
        ) -> Result<T, ApiError> {
            if let Ok(response) = self.request(endpoint, Method::GET)?.send().await {
                self.extract_response::<T>(response).await
            } else {
                Err(ApiError::ConnectionError {})
            }
        }

        /// Executes a DELETE request
        pub async fn delete(
            &self,
            endpoint: &str,
        ) -> Result<(), ApiError> {
            if let Ok(response) = self.request(endpoint, Method::DELETE)?.send().await {
                match response.error_for_status() {
                    Ok(_) => Ok(()),
                    Err(resp) => Err(ApiError::Request {
                        error: RequestError {
                            code: resp.status().map_or(0, |s| s.as_u16()),
                            reason: Some(resp.to_string()),
                        },
                    })
                }
                
            } else {
                Err(ApiError::ConnectionError {})
            }
        }

        /// Executes a POST request
        pub async fn post<T: DeserializeOwned + Debug, D: Serialize>(
            &self,
            endpoint: &str,
            data: Option<D>,
        ) -> Result<T, ApiError> {
            if let Ok(response) = self
                .request(endpoint, Method::POST)?
                .json(&data)
                .send()
                .await
            {
                self.extract_response::<T>(response).await
            } else {
                Err(ApiError::ConnectionError {})
            }
        }
    }
}
