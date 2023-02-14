use async_trait::async_trait;
use protos::users::{
    users_server::Users, CreateUserRequest, CreateUserResponse, DeleteUserRequest,
    DeleteUserResponse, GetByFiltersRequest, ListByFiltersRequest, ListByFiltersResponse,
    UpdateUserRequest, UpdateUserResponse, User,
};
use tonic::Response;

pub struct UsersServices {}

impl UsersServices {
    pub fn new() -> UsersServices {
        UsersServices {}
    }
}

#[async_trait]
impl Users for UsersServices {
    async fn create(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> Result<tonic::Response<CreateUserResponse>, tonic::Status> {
        Ok(Response::new(CreateUserResponse::default()))
    }

    async fn get_by_filters(
        &self,
        request: tonic::Request<GetByFiltersRequest>,
    ) -> Result<tonic::Response<User>, tonic::Status> {
        Ok(Response::new(User::default()))
    }

    async fn list_by_filters(
        &self,
        request: tonic::Request<ListByFiltersRequest>,
    ) -> Result<tonic::Response<ListByFiltersResponse>, tonic::Status> {
        Ok(Response::new(ListByFiltersResponse::default()))
    }

    async fn update_user(
        &self,
        request: tonic::Request<UpdateUserRequest>,
    ) -> Result<tonic::Response<UpdateUserResponse>, tonic::Status> {
        Ok(Response::new(UpdateUserResponse::default()))
    }
    async fn delete_user(
        &self,
        request: tonic::Request<DeleteUserRequest>,
    ) -> Result<tonic::Response<DeleteUserResponse>, tonic::Status> {
        Ok(Response::new(DeleteUserResponse::default()))
    }
}
