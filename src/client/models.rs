pub mod api_models {
    pub mod users {
        use chrono::{DateTime, Utc};
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct User {
            pub username: String,
            pub email: Option<String>,
            pub created: Option<DateTime<Utc>>,
        }
    }

    pub mod posts {
        use chrono::{DateTime, Utc};
        use derive_builder::Builder;
        use reqwest::Method;
        use serde_derive::{Deserialize, Serialize};

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
            Code,
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
            pub rtl: bool,
        }

        impl PostUpdate {
            pub async fn update(&self) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .post::<Post, PostUpdate>(
                            format!("/posts/{}", self.id).as_str(),
                            Some(self.clone()),
                        )
                        .await
                        .and_then(|mut p| Ok(p.with_client(client.clone())))
                } else {
                    Err(ApiError::UsageError {})
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
            pub token: Option<String>,
        }

        impl Post {
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }

            pub fn build_update(&self, body: String) -> PostUpdateBuilder {
                PostUpdateBuilder::default()
                    .client(self.client.clone())
                    .id(self.id.clone())
                    .body(body)
                    .clone()
            }

            pub async fn update(&self, update: PostUpdate) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .post::<Post, PostUpdate>(
                            format!("/posts/{}", self.id).as_str(),
                            Some(update.clone()),
                        )
                        .await
                        .and_then(|mut p| Ok(p.with_client(client.clone())))
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn delete(&self) -> Result<(), ApiError> {
                if let Some(client) = self.client.clone() {
                    let mut request = client
                        .api()
                        .request(format!("/posts/{}", self.id).as_str(), Method::DELETE)
                        .unwrap();
                    if !client.is_authenticated() && self.token.is_some() {
                        request = request.query(&[("token", self.token.clone().unwrap())]);
                    }
                    if let Ok(result) = request.send().await {
                        client.api().extract_response(result).await
                    } else {
                        Err(ApiError::ConnectionError {})
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        pub struct PostCreation {
            #[serde(skip_serializing)]
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            pub collection: Option<String>,

            pub body: String,
            pub title: Option<String>,
            pub font: Option<PostAppearance>,
            pub lang: Option<String>,
            pub rtl: Option<bool>,
            pub created: Option<DateTime<Utc>>,
        }

        impl PostCreation {
            pub async fn publish(&self) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    if let Some(collection) = self.collection.clone() {
                        client
                            .api()
                            .post::<Post, PostCreation>(
                                format!("/collections/{collection}/post").as_str(),
                                Some(self.clone()),
                            )
                            .await
                            .and_then(|mut v| Ok(v.with_client(client.clone())))
                    } else {
                        client
                            .api()
                            .post::<Post, PostCreation>("/posts", Some(self.clone()))
                            .await
                            .and_then(|mut v| Ok(v.with_client(client.clone())))
                    }
                } else {
                    Err(ApiError::UsageError {})
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
            pub user: User,
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct ResponseModel {
            pub code: u16,
            pub data: Value,
        }
    }

    pub mod requests {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub alias: String,
            pub pass: String,
        }
    }

    pub mod collections {
        use derive_builder::Builder;
        use serde_derive::{Deserialize, Serialize};
        use serde_repr::{Deserialize_repr, Serialize_repr};

        use crate::api_client::{ApiError, Client};

        use super::posts::Post;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct MovePost {
            pub id: String,
            pub token: Option<String>,
        }

        impl MovePost {
            pub fn new(id: &str) -> Self {
                MovePost {
                    id: id.to_string(),
                    token: None,
                }
            }

            pub fn new_with_token(id: &str, token: &str) -> Self {
                MovePost {
                    id: id.to_string(),
                    token: Some(token.to_string()),
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum MoveResult {
            Success { code: u32, post: Post },
            Error { code: u32, error_msg: String },
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct PinPost {
            pub id: String,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub postion: Option<u64>
        }

        impl PinPost {
            pub fn new(id: &str) -> Self {
                PinPost {
                    id: id.to_string(),
                    postion: None
                }
            }

            pub fn new_at_position(id: &str, position: u64) -> Self {
                PinPost {
                    id: id.to_string(),
                    postion: Some(position),
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum PinResult {
            Success { code: u32, id: String },
            Error { code: u32, error_msg: String },
        }

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

            pub fn build_update(&self) -> CollectionUpdateBuilder {
                CollectionUpdateBuilder::default()
                    .alias(Some(self.alias.clone()))
                    .client(self.client.clone())
                    .clone()
            }

            pub async fn update(&self, update: CollectionUpdate) -> Result<Collection, ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .post::<Collection, CollectionUpdate>(
                            format!("/collections/{}", self.alias).as_str(),
                            Some(update.clone()),
                        )
                        .await
                        .and_then(|mut p| Ok(p.with_client(client.clone())))
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn delete(&self) -> Result<(), ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .delete(format!("/collections/{}", self.alias).as_str())
                        .await
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn get_posts(&self) -> Result<Vec<Post>, ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .get::<Vec<Post>>(format!("/collections/{}/posts", self.alias).as_str())
                        .await
                        .and_then(|mut v| {
                            Ok(v.iter_mut()
                                .map(|x| x.with_client(client.clone()))
                                .collect())
                        })
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn get_post(&self, slug: String) -> Result<Post, ApiError> {
                if let Some(client) = self.client.clone() {
                    client
                        .api()
                        .get::<Post>(format!("/collections/{}/posts/{}", self.alias, slug).as_str())
                        .await
                        .and_then(|mut v| Ok(v.with_client(client.clone())))
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn take_posts(
                &self,
                posts: &[MovePost],
            ) -> Result<Vec<Result<MoveResult, MoveResult>>, ApiError> {
                if let Some(client) = self.client.clone() {
                    let result = client
                        .api()
                        .post::<Vec<MoveResult>, &[MovePost]>(
                            format!("/collections/{}/collect", self.alias).as_str(),
                            Some(posts),
                        )
                        .await;
                    match result {
                        Ok(results) => Ok(results
                            .iter()
                            .map::<Result<MoveResult, MoveResult>, _>(|r| match r {
                                MoveResult::Success { code, post } => Ok(MoveResult::Success {
                                    code: code.clone(),
                                    post: post.clone().with_client(client.clone()),
                                }),
                                MoveResult::Error { code, error_msg } => Err(MoveResult::Error {
                                    code: code.clone(),
                                    error_msg: error_msg.clone(),
                                }),
                            })
                            .collect()),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn pin_posts(
                &self,
                posts: &[PinPost],
            ) -> Result<Vec<Result<PinResult, PinResult>>, ApiError> {
                if let Some(client) = self.client.clone() {
                    let result = client
                        .api()
                        .post::<Vec<PinResult>, &[PinPost]>(
                            format!("/collections/{}/pin", self.alias).as_str(),
                            Some(posts),
                        )
                        .await;
                    match result {
                        Ok(results) => Ok(results
                            .iter()
                            .map::<Result<PinResult, PinResult>, _>(|r| match r {
                                PinResult::Success { code, id } => Ok(PinResult::Success {
                                    code: code.clone(),
                                    id: id.clone()
                                }),
                                PinResult::Error { code, error_msg } => Err(PinResult::Error {
                                    code: code.clone(),
                                    error_msg: error_msg.clone(),
                                }),
                            })
                            .collect()),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }

            pub async fn unpin_posts(&self, posts: &[String]) -> Result<Vec<Result<PinResult, PinResult>>, ApiError> {
                if let Some(client) = self.client.clone() {
                    let result = client
                        .api()
                        .post::<Vec<PinResult>, Vec<PinPost>>(
                            format!("/collections/{}/unpin", self.alias).as_str(),
                            Some(posts.iter().map(|v| PinPost::new(v.as_str())).collect()),
                        )
                        .await;
                    match result {
                        Ok(results) => Ok(results
                            .iter()
                            .map::<Result<PinResult, PinResult>, _>(|r| match r {
                                PinResult::Success { code, id } => Ok(PinResult::Success {
                                    code: code.clone(),
                                    id: id.clone()
                                }),
                                PinResult::Error { code, error_msg } => Err(PinResult::Error {
                                    code: code.clone(),
                                    error_msg: error_msg.clone(),
                                }),
                            })
                            .collect()),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }
        }

        #[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
        #[repr(u8)]
        pub enum CollectionVisibility {
            Unlisted = 0,
            Public = 1,
            Private = 2,
            Password = 4,
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        pub struct CollectionUpdate {
            #[serde(skip_serializing)]
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            pub alias: Option<String>,

            pub title: Option<String>,
            pub description: Option<String>,
            pub style_sheet: Option<String>,
            pub script: Option<String>,
            pub visibility: Option<CollectionVisibility>,
            pub pass: Option<String>,
            pub mathjax: bool,
        }

        impl CollectionUpdate {
            pub async fn update(&self) -> Result<Collection, ApiError> {
                if let Some(client) = self.client.clone() {
                    if let Some(alias) = self.alias.clone() {
                        client
                            .api()
                            .post::<Collection, CollectionUpdate>(
                                format!("/collections/{}", alias).as_str(),
                                Some(self.clone()),
                            )
                            .await
                            .and_then(|mut p| Ok(p.with_client(client.clone())))
                    } else {
                        Err(ApiError::UsageError {})
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }
        }
    }
}
