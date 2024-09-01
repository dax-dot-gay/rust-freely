pub mod api_handlers {

    use crate::{
        api_client::{ApiError, Client},
        api_models::{collections::Collection, posts::{Post, PostCreation, PostCreationBuilder}, users::User},
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
        client: Client
    }

    impl PostHandler {
        pub fn new(client: Client) -> Self {
            PostHandler { client: client.clone() }
        }

        pub async fn get(&self, id: &str) -> Result<Post, ApiError> {
            self.client.api().get::<Post>(format!("/posts/{id}").as_str()).await.and_then(|mut p| Ok(p.with_client(self.client.clone())))
        }

        pub fn create(&self, body: String) -> PostCreationBuilder {
            PostCreationBuilder::default().client(Some(self.client.clone())).body(body).clone()
        }

        pub async fn publish(&self, post: PostCreation) -> Result<Post, ApiError> {
            self.client.api().post::<Post, PostCreation>("/posts", Some(post)).await.and_then(|mut p| Ok(p.with_client(self.client.clone())))
        }
    }
}
