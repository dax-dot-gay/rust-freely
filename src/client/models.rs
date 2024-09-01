/// This module provides API model definitions & associated methods.
pub mod api_models {
    

    /// This module provides models related to [User]
    pub mod users {
        use chrono::{DateTime, Utc};
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        /// Base User model
        pub struct User {
            /// Username
            pub username: String,

            /// Email (may not be present based on instance settings & associated request)
            pub email: Option<String>,

            /// Creation D/T (may not be present based on instance settings & associated request)
            pub created: Option<DateTime<Utc>>,
        }
    }

    /// This module provides models related to [Post]
    pub mod posts {
        use chrono::{DateTime, Utc};
        use derive_builder::Builder;
        use reqwest::Method;
        use serde_derive::{Deserialize, Serialize};

        use crate::api_client::{ApiError, Client};

        use super::collections::{Collection, MovePost, MoveResult};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        /// Enum describing the appearance/font of a post
        pub enum PostAppearance {
            #[serde(rename = "sans")]
            ///
            SansSerif,

            #[serde(rename = "serif")]
            #[serde(alias = "norm")]
            #[serde(alias = "serif")]
            ///
            Serif,

            #[serde(rename = "wrap")]
            ///
            Wrap,

            #[serde(rename = "mono")]
            ///
            Mono,

            #[serde(rename = "code")]
            ///
            Code,
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        /// Struct describing a pending update to a [Post]
        pub struct PostUpdate {
            #[serde(skip_serializing)]
            /// [Client] instance
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            /// Post ID
            pub id: String,

            /// Post token, if not owned
            pub token: Option<String>,

            /// New post body
            pub body: String,

            /// New post title
            pub title: Option<String>,

            /// New post font
            pub font: Option<PostAppearance>,

            /// New post language
            pub lang: Option<String>,

            /// New post RTL
            pub rtl: bool,
        }

        impl PostUpdate {
            /// Dispatches an update request to the server.
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
        /// Main struct describing a single Post
        pub struct Post {
            ///
            pub client: Option<Client>,
            ///
            pub id: String,
            ///
            pub slug: Option<String>,
            ///
            pub appearance: Option<PostAppearance>,
            ///
            pub language: Option<String>,
            ///
            pub rtl: bool,
            ///
            pub created: Option<DateTime<Utc>>,
            ///
            pub title: Option<String>,
            ///
            pub body: String,
            ///
            pub tags: Vec<String>,
            ///
            pub views: Option<u64>,
            ///
            pub collection: Option<Collection>,
            ///
            pub token: Option<String>,
        }

        impl Post {
            #[doc(hidden)]
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }

            /// Returns a [PostUpdateBuilder] initialized with a [Client], the correct ID, and the specified body text
            pub fn build_update(&self, body: String) -> PostUpdateBuilder {
                PostUpdateBuilder::default()
                    .client(self.client.clone())
                    .id(self.id.clone())
                    .body(body)
                    .clone()
            }

            /// Dispatches an update with an existing [PostUpdate]
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

            /// Deletes this post
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

            /// Moves the post to a [Collection] by its alias
            pub async fn move_to(&self, collection: &str) -> Result<MoveResult, ApiError> {
                if let Some(client) = self.client.clone() {
                    match client.collections().get(collection).await {
                        Ok(coll) => {
                            match client.is_authenticated() {
                                true => coll.take_posts(&[MovePost::new(&self.id)]).await,
                                false => coll.take_posts(&[MovePost {id: self.id.clone(), token: self.token.clone()}]).await
                            }.and_then(|v| {
                                match v.get(0) {
                                    Some(item) => match item {
                                        Ok(result) => Ok(result.clone()),
                                        Err(result) => Ok(result.clone())
                                    },
                                    None => Err(ApiError::UnknownError {  })
                                }
                            })
                        },
                        Err(e) => Err(e) 
                    }
                } else {
                    Err(ApiError::UsageError {})
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        /// Post creation struct
        pub struct PostCreation {
            #[serde(skip_serializing)]
            /// [Client] instance
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            /// Collection to post to, if desired
            pub collection: Option<String>,

            /// Post body
            pub body: String,

            /// Post title
            pub title: Option<String>,

            /// Post font
            pub font: Option<PostAppearance>,

            /// Post language
            pub lang: Option<String>,

            /// Post RTL
            pub rtl: Option<bool>,

            /// Specific post creation DT
            pub created: Option<DateTime<Utc>>,
        }

        impl PostCreation {
            /// Publishes the described post to the server
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

    #[doc(hidden)]
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

    #[doc(hidden)]
    pub mod requests {
        use serde_derive::{Deserialize, Serialize};

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Login {
            pub alias: String,
            pub pass: String,
        }
    }

    /// This module provides models related to [Collection]
    pub mod collections {
        use derive_builder::Builder;
        use serde_derive::{Deserialize, Serialize};
        use serde_repr::{Deserialize_repr, Serialize_repr};

        use crate::api_client::{ApiError, Client};

        use super::posts::Post;

        #[derive(Clone, Debug, Serialize, Deserialize)]
        /// A struct describing a post to move into a collection
        pub struct MovePost {
            /// Post ID
            pub id: String,

            /// Post token, if post isn't owned
            pub token: Option<String>,
        }

        impl MovePost {
            /// Creates a new MovePost with just an ID
            pub fn new(id: &str) -> Self {
                MovePost {
                    id: id.to_string(),
                    token: None,
                }
            }

            /// Creates a new MovePost with an ID and token
            pub fn new_with_token(id: &str, token: &str) -> Self {
                MovePost {
                    id: id.to_string(),
                    token: Some(token.to_string()),
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        /// Describes the result of a single post move operation
        pub enum MoveResult {
            /// Successful operation
            Success { 
                /// Operation status code
                code: u32, 

                /// Affected [Post]
                post: Post 
            },

            /// Failed operation
            Error { 
                /// Operation status code
                code: u32, 

                /// Operation status text
                error_msg: String 
            },
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        /// A struct describing how to pin or unpin a post to a collection
        pub struct PinPost {
            /// Post ID
            pub id: String,

            #[serde(skip_serializing_if = "Option::is_none")]
            /// Pin position (should not be used with `unpin`)
            pub postion: Option<u64>
        }

        impl PinPost {
            /// Creates a new PinPost with an ID
            pub fn new(id: &str) -> Self {
                PinPost {
                    id: id.to_string(),
                    postion: None
                }
            }

            /// Creates a new PinPost with an ID and a position
            pub fn new_at_position(id: &str, position: u64) -> Self {
                PinPost {
                    id: id.to_string(),
                    postion: Some(position),
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        /// Describes the result of a single pin/unpin operation
        pub enum PinResult {
            /// Successful operation
            Success { 
                /// Operation status code
                code: u32, 
                /// Post ID
                id: String 
            },

            /// Failed operation
            Error { 
                /// Operation status code
                code: u32, 
                /// Operation status text
                error_msg: String 
            },
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        /// A struct describing a single Collection entity
        pub struct Collection {
            ///
            pub client: Option<Client>,
            ///
            pub alias: String,
            ///
            pub title: String,
            ///
            pub description: Option<String>,
            ///
            pub style_sheet: Option<String>,
            ///
            pub public: bool,
            ///
            pub views: Option<u64>,
            ///
            pub verification_link: Option<String>,
            ///
            pub total_posts: Option<u64>,
        }

        impl Collection {
            #[doc(hidden)]
            pub fn with_client(&mut self, client: Client) -> Self {
                self.client = Some(client);
                self.clone()
            }

            /// Creates a [CollectionUpdateBuilder] with defaults set
            pub fn build_update(&self) -> CollectionUpdateBuilder {
                CollectionUpdateBuilder::default()
                    .alias(Some(self.alias.clone()))
                    .client(self.client.clone())
                    .clone()
            }
            
            /// Updates a collection from an existing [CollectionUpdate]
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

            /// Deletes this [Collection]
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

            /// Returns all [Post]s belonging to this collection
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

            /// Returns a single [Post] belonging to this collection
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

            /// Moves a set of [Post]s into this collection
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

            /// Pins a set of [Post]s in this collection
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

            /// Unpins a set of [Post]s from this collection
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
        /// Enum describing a collection's visibility
        pub enum CollectionVisibility {
            ///
            Unlisted = 0,
            ///
            Public = 1,
            ///
            Private = 2,
            ///
            Password = 4,
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Builder)]
        /// Struct describing a collection update
        pub struct CollectionUpdate {
            #[serde(skip_serializing)]
            /// [Client] instance
            pub client: Option<Client>,

            #[serde(skip_serializing)]
            /// Collection alias to update
            pub alias: Option<String>,

            /// New title
            pub title: Option<String>,

            /// New description
            pub description: Option<String>,

            /// New style sheet
            pub style_sheet: Option<String>,

            /// New script (Write.as only)
            pub script: Option<String>,

            /// New visibility level
            pub visibility: Option<CollectionVisibility>,

            /// New password (only [CollectionVisibility::Password])
            pub pass: Option<String>,

            /// Whether to enable Mathjax support
            pub mathjax: bool,
        }

        impl CollectionUpdate {
            /// Publish the update request to the server
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
