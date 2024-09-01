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
    pub struct UserHandler {
        client: Client,
        current: Option<User>,
    }

    impl UserHandler {
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

        pub fn info(&self) -> Option<User> {
            self.current.clone()
        }

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
    pub struct PostHandler {
        client: Client,
    }

    impl PostHandler {
        pub fn new(client: Client) -> Self {
            PostHandler {
                client: client.clone(),
            }
        }

        pub async fn get(&self, id: &str) -> Result<Post, ApiError> {
            self.client
                .api()
                .get::<Post>(format!("/posts/{id}").as_str())
                .await
                .and_then(|mut p| Ok(p.with_client(self.client.clone())))
        }

        pub fn create(&self, body: String) -> PostCreationBuilder {
            PostCreationBuilder::default()
                .client(Some(self.client.clone()))
                .body(body)
                .clone()
        }

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
    pub struct CollectionHandler {
        client: Client,
    }

    impl CollectionHandler {
        pub fn new(client: Client) -> Self {
            CollectionHandler {
                client: client.clone(),
            }
        }

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

        pub async fn get(&self, alias: &str) -> Result<Collection, ApiError> {
            self.client
                .api()
                .get::<Collection>(format!("/collections/{alias}").as_str())
                .await
                .and_then(|mut v| Ok(v.with_client(self.client.clone())))
        }
    }
}
