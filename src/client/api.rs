pub mod api_wrapper {
    use std::fmt::Debug;

    use reqwest::{header, Client as ReqwestClient, Error, Method, RequestBuilder, Response, Url};
    use serde::{de::DeserializeOwned, Serialize};

    use crate::{
        api_client::{ApiError, Client, RequestError},
        api_models::responses::ResponseModel,
    };

    #[derive(Clone, Debug)]
    pub struct Api {
        client: Client,
    }

    impl Api {
        pub fn new(client: Client) -> Self {
            Api { client }
        }

        pub fn base(&self) -> String {
            self.client.url()
        }

        pub fn token(&self) -> Option<String> {
            self.client.token()
        }

        pub fn is_authenticated(&self) -> bool {
            self.client.is_authenticated()
        }

        pub fn url(&self, endpoint: &str) -> Result<Url, ApiError> {
            if let Ok(result) = Url::parse(self.base().as_str()) {
                if let Ok(result) = result.join(endpoint) {
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

        pub fn request(&self, endpoint: &str, method: Method) -> Result<RequestBuilder, ApiError> {
            if let Ok(http) = self.http() {
                if let Ok(url) = self.url(endpoint) {
                    let mut request = http.request(method, url);
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

        pub async fn delete<T: DeserializeOwned + Debug>(
            &self,
            endpoint: &str,
        ) -> Result<T, ApiError> {
            if let Ok(response) = self.request(endpoint, Method::DELETE)?.send().await {
                self.extract_response::<T>(response).await
            } else {
                Err(ApiError::ConnectionError {})
            }
        }

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
