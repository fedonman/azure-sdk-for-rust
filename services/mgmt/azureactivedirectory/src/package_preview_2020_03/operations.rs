#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
pub mod private_link_for_azure_ad {
    use crate::models::*;
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
    ) -> std::result::Result<PrivateLinkPolicy, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| get::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| get::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| get::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| get::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicy = serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(get::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
        private_link_policy: &PrivateLinkPolicy,
    ) -> std::result::Result<create::Response, create::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| create::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PUT);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| create::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = azure_core::to_json(private_link_policy).map_err(|source| create::Error::SerializeError { source })?;
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| create::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| create::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicy = serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            http::StatusCode::CREATED => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicy = serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(create::Response::Created201(rsp_value))
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| create::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(create::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200(PrivateLinkPolicy),
            Created201(PrivateLinkPolicy),
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
        private_link_policy: Option<&PrivateLinkPolicyUpdateParameter>,
    ) -> std::result::Result<PrivateLinkPolicy, update::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| update::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::PATCH);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| update::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = if let Some(private_link_policy) = private_link_policy {
            azure_core::to_json(private_link_policy).map_err(|source| update::Error::SerializeError { source })?
        } else {
            bytes::Bytes::from_static(azure_core::EMPTY_BODY)
        };
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| update::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| update::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicy = serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| update::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(update::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| delete::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::DELETE);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| delete::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| delete::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| delete::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => Ok(delete::Response::Ok200),
            http::StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| delete::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(delete::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_subscription(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<PrivateLinkPolicyListResult, list_by_subscription::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/providers/microsoft.aadiam/privateLinkForAzureAd",
            operation_config.base_path(),
            subscription_id
        );
        let mut url = url::Url::parse(url_str).map_err(|source| list_by_subscription::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list_by_subscription::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list_by_subscription::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list_by_subscription::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicyListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list_by_subscription::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse =
                    serde_json::from_slice(rsp_body).map_err(|source| list_by_subscription::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Err(list_by_subscription::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list_by_subscription {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<PrivateLinkPolicyListResult, list::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd",
            operation_config.base_path(),
            subscription_id,
            resource_group_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| list::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkPolicyListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                let rsp_value: ErrorResponse = serde_json::from_slice(rsp_body).map_err(|source| list::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Err(list::Error::DefaultResponse {
                    status_code,
                    value: rsp_value,
                })
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse {
                status_code: http::StatusCode,
                value: models::ErrorResponse,
            },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub mod private_link_resources {
    use crate::models::*;
    pub async fn list_by_private_link_policy(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
    ) -> std::result::Result<PrivateLinkResourceListResult, list_by_private_link_policy::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}/privateLinkResources",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| list_by_private_link_policy::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| list_by_private_link_policy::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| list_by_private_link_policy::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| list_by_private_link_policy::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkResourceListResult =
                    serde_json::from_slice(rsp_body).map_err(|source| list_by_private_link_policy::Error::DeserializeError {
                        source,
                        body: rsp_body.clone(),
                    })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(list_by_private_link_policy::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod list_by_private_link_policy {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        policy_name: &str,
        group_name: &str,
    ) -> std::result::Result<PrivateLinkResource, get::Error> {
        let http_client = operation_config.http_client();
        let url_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/microsoft.aadiam/privateLinkForAzureAd/{}/privateLinkResources/{}",
            operation_config.base_path(),
            subscription_id,
            resource_group_name,
            policy_name,
            group_name
        );
        let mut url = url::Url::parse(url_str).map_err(|source| get::Error::ParseUrlError { source })?;
        let mut req_builder = http::request::Builder::new();
        req_builder = req_builder.method(http::Method::GET);
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource())
                .await
                .map_err(|source| get::Error::GetTokenError { source })?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
        url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
        let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        req_builder = req_builder.uri(url.as_str());
        let req = req_builder
            .body(req_body)
            .map_err(|source| get::Error::BuildRequestError { source })?;
        let rsp = http_client
            .execute_request(req)
            .await
            .map_err(|source| get::Error::ExecuteRequestError { source })?;
        match rsp.status() {
            http::StatusCode::OK => {
                let rsp_body = rsp.body();
                let rsp_value: PrivateLinkResource = serde_json::from_slice(rsp_body).map_err(|source| get::Error::DeserializeError {
                    source,
                    body: rsp_body.clone(),
                })?;
                Ok(rsp_value)
            }
            status_code => {
                let rsp_body = rsp.body();
                Err(get::Error::UnexpectedResponse {
                    status_code,
                    body: rsp_body.clone(),
                })
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
            #[error("Failed to parse request URL: {}", source)]
            ParseUrlError { source: url::ParseError },
            #[error("Failed to build request: {}", source)]
            BuildRequestError { source: http::Error },
            #[error("Failed to execute request: {}", source)]
            ExecuteRequestError { source: azure_core::errors::HttpError },
            #[error("Failed to serialize request body: {}", source)]
            SerializeError { source: serde_json::Error },
            #[error("Failed to deserialize response body: {}", source)]
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            #[error("Failed to get access token: {}", source)]
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
