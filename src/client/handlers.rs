pub mod api_handlers {
    use crate::{
        api_client::{ApiError, Client},
        api_models::{collections::Collection, posts::Post, users::User},
    };

    #[derive(Clone, Debug)]
    pub struct UserHandler {
        client: Client,
        current: Option<User>,
    }

    impl UserHandler {
        pub async fn create(client: Client) -> Self {
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
}
