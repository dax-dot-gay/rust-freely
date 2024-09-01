/// This module provides wrappers for top-level (ie, not referencing a specific entity) API methods
pub mod api_handlers {

    use serde_derive::{Deserialize, Serialize};

    use crate::{
        api_client::{ApiError, Client},
        api_models::{
            collections::Collection,
            posts::{Post, PostCreation, PostCreationBuilder},
            users::User,
        },
    };

    #[derive(Clone, Debug)]
    /// Handler for [User] methods
    pub struct UserHandler {
        client: Client,
        current: Option<User>,
    }

    impl UserHandler {
        /// Creates a new [UserHandler] instance, and preloads the authenticated user info if available.
        pub async fn new(client: Client) -> Self {
            if client.is_authenticated() {
                UserHandler {
                    client: client.clone(),
                    current: match client.api().get::<User>("/me").await {
                        Ok(user) => Some(user),
                        Err(_) => None,
                    },
                }
            } else {
                UserHandler {
                    client: client.clone(),
                    current: None,
                }
            }
        }

        /// Returns the current [User] if available
        pub fn info(&self) -> Option<User> {
            self.current.clone()
        }

        /// Returns all [Post]s associated with the authenticated [User]
        pub async fn posts(&self) -> Result<Vec<Post>, ApiError> {
            if self.client.is_authenticated() {
                self.client
                    .api()
                    .get::<Vec<Post>>("/me/posts")
                    .await
                    .and_then(|mut v| {
                        Ok(v.iter_mut()
                            .map(|x| x.with_client(self.client.clone()))
                            .collect())
                    })
            } else {
                Err(ApiError::LoggedOut {})
            }
        }

        /// Returns the specified [Post]
        pub async fn post(&self, id: &str) -> Result<Post, ApiError> {
            if self.client.is_authenticated() {
                self.client
                    .api()
                    .get::<Post>(format!("/posts/{id}").as_str())
                    .await
                    .and_then(|mut v| Ok(v.with_client(self.client.clone())))
            } else {
                Err(ApiError::LoggedOut {})
            }
        }

        /// Returns all [Collection]s associated with the authenticated [User]
        pub async fn collections(&self) -> Result<Vec<Collection>, ApiError> {
            if self.client.is_authenticated() {
                self.client
                    .api()
                    .get::<Vec<Collection>>("/me/collections")
                    .await
                    .and_then(|mut v| {
                        Ok(v.iter_mut()
                            .map(|x| x.with_client(self.client.clone()))
                            .collect())
                    })
            } else {
                Err(ApiError::LoggedOut {})
            }
        }

        /// Returns the specified [Collection]
        pub async fn collection(&self, alias: &str) -> Result<Collection, ApiError> {
            if self.client.is_authenticated() {
                self.client
                    .api()
                    .get::<Collection>(format!("/collections/{alias}").as_str())
                    .await
                    .and_then(|mut v| Ok(v.with_client(self.client.clone())))
            } else {
                Err(ApiError::LoggedOut {})
            }
        }
    }

    #[derive(Clone, Debug)]
    /// Handler for [Post] methods
    pub struct PostHandler {
        client: Client,
    }

    impl PostHandler {
        /// Creates a new [PostHandler] with a [Client] instance
        pub fn new(client: Client) -> Self {
            PostHandler {
                client: client.clone(),
            }
        }

        /// Gets a specific [Post] by ID
        pub async fn get(&self, id: &str) -> Result<Post, ApiError> {
            self.client
                .api()
                .get::<Post>(format!("/posts/{id}").as_str())
                .await
                .and_then(|mut p| Ok(p.with_client(self.client.clone())))
        }

        /// Creates a [PostCreationBuilder] with the desired body.
        pub fn create(&self, body: String) -> PostCreationBuilder {
            PostCreationBuilder::default()
                .client(Some(self.client.clone()))
                .body(body)
                .clone()
        }

        /// Publishes a previously-made [PostCreation] instance
        pub async fn publish(&self, post: PostCreation) -> Result<Post, ApiError> {
            if let Some(collection) = post.collection.clone() {
                self.client
                    .api()
                    .post::<Post, PostCreation>(format!("/collections/{collection}/post").as_str(), Some(post))
                    .await
                    .and_then(|mut p| Ok(p.with_client(self.client.clone())))
            } else {
                self.client
                    .api()
                    .post::<Post, PostCreation>("/posts", Some(post))
                    .await
                    .and_then(|mut p| Ok(p.with_client(self.client.clone())))
            }
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    struct CollectionParameters {
        pub alias: Option<String>,
        pub title: Option<String>,
    }

    #[derive(Clone, Debug)]
    /// Handler for [Collection] methods
    pub struct CollectionHandler {
        client: Client,
    }

    impl CollectionHandler {
        /// Creates a new [CollectionHandler] with a [Client] instance
        pub fn new(client: Client) -> Self {
            CollectionHandler {
                client: client.clone(),
            }
        }

        /// Creates a new [Collection]. At least one of `alias` and `title` must be specified.
        pub async fn create(
            &self,
            alias: Option<String>,
            title: Option<String>,
        ) -> Result<Collection, ApiError> {
            if alias.is_none() && title.is_none() {
                return Err(ApiError::UsageError {});
            }

            if !self.client.is_authenticated() {
                return Err(ApiError::LoggedOut {});
            }

            let params = CollectionParameters { alias, title };
            self.client
                .api()
                .post::<Collection, CollectionParameters>("/collections", Some(params))
                .await
                .and_then(|mut v| Ok(v.with_client(self.client.clone())))
        }

        /// Retrieves a [Collection] by its alias.
        pub async fn get(&self, alias: &str) -> Result<Collection, ApiError> {
            self.client
                .api()
                .get::<Collection>(format!("/collections/{alias}").as_str())
                .await
                .and_then(|mut v| Ok(v.with_client(self.client.clone())))
        }
    }
}
