pub mod api_models {
    pub mod users {
        use serde_derive::{Deserialize, Serialize};
        use chrono::{DateTime, Utc};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct User {
            pub username: String,
            pub email: Option<String>,
            pub created: Option<DateTime<Utc>>
        }

        
    }

    pub mod posts {
        use derive_builder::Builder;
        use reqwest::Method;
        use serde_derive::{Deserialize, Serialize};
        use chrono::{DateTime, Utc};

        use crate::api_client::{ApiError, Client};

        use super::collections::Collection;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum PostAppearance {
            #[serde(rename = "sans")]
            SansSerif,

            #[serde(rename = "serif")]
            #[serde(alias = "norm")]
            #[serde(alias = "serif")]
            Serif,

            #[serde(rename = "wrap")]
            Wrap,

            #[serde(rename = "mono")]
            Mono,

            #[serde(rename = "code")]
            Code
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        pub struct PostUpdate {
            #[serde(skip_serializing)]
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            pub id: String,

            pub token: Option<String>,
            pub body: String,
            pub title: Option<String>,
            pub font: Option<PostAppearance>,
            pub lang: Option<String>,
            pub rtl: bool
        }

        impl PostUpdate {
            pub async fn update(&self) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client.api().post::<Post, PostUpdate>(format!("/posts/{}", self.id).as_str(), Some(self.clone())).await
                } else {
                    Err(ApiError::UsageError {  })
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Post {
            pub client: Option<Client>,
            pub id: String,
            pub slug: Option<String>,
            pub appearance: Option<PostAppearance>,
            pub language: Option<String>,
            pub rtl: bool,
            pub created: Option<DateTime<Utc>>,
            pub title: Option<String>,
            pub body: String,
            pub tags: Vec<String>,
            pub views: Option<u64>,
            pub collection: Option<Collection>,
            pub token: Option<String>
        }

        impl Post {
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }

            pub fn build_update(&self, body: String) -> PostUpdateBuilder {
                PostUpdateBuilder::default().client(self.client.clone()).id(self.id.clone()).body(body).clone()
            }

            pub async fn update(&self, update: PostUpdate) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client.api().post::<Post, PostUpdate>(format!("/posts/{}", self.id).as_str(), Some(update.clone())).await
                } else {
                    Err(ApiError::UsageError {  })
                }
            }

            pub async fn delete(&self) -> Result<(), ApiError> {
                if let Some(client) = self.client.clone() {
                    let mut request = client.api().request(format!("/posts/{}", self.id).as_str(), Method::DELETE).unwrap();
                    if !client.is_authenticated() && self.token.is_some() {
                        request = request.query(&[("token", self.token.clone().unwrap())]);
                    }
                    if let Ok(result) = request.send().await {
                        client.api().extract_response(result).await
                    } else {
                        Err(ApiError::ConnectionError {  })
                    }
                } else {
                    Err(ApiError::UsageError {  })
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        pub struct PostCreation {
            #[serde(skip_serializing)]
            pub client: Option<Client>,

            pub body: String,
            pub title: Option<String>,
            pub font: Option<PostAppearance>,
            pub lang: Option<String>,
            pub rtl: Option<bool>,
            pub created: Option<DateTime<Utc>>
        }

        impl PostCreation {
            pub async fn publish(&self) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client.api().post::<Post, PostCreation>("/posts", Some(self.clone())).await
                } else {
                    Err(ApiError::UsageError {  })
                }
            }
        }

        
    }

    pub mod responses {
        use std::fmt::Debug;

        use super::users::User;
        use serde_derive::{Deserialize, Serialize};
        use serde_json::Value;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub access_token: String,
            pub user: User
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct ResponseModel {
            pub code: u16,
            pub data: Value
        }
    }

    pub mod requests {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub alias: String,
            pub pass: String
        }
    }

    pub mod collections {
        use serde_derive::{Deserialize, Serialize};

        use crate::api_client::Client;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Collection {
            pub client: Option<Client>,
            pub alias: String,
            pub title: String,
            pub description: Option<String>,
            pub style_sheet: Option<String>,
            pub public: bool,
            pub views: Option<u64>,
            pub verification_link: Option<String>,
            pub total_posts: Option<u64>,
        }

        impl Collection {
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }
        }
    }
}