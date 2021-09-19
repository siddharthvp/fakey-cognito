use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_RESET_USER_PASSWORD_NAME: &str = "AdminResetUserPassword";
pub const ADMIN_RESET_USER_PASSWORD_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminResetUserPassword";

/// AdminResetUserPassword response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminResetUserPassword.html#API_AdminResetUserPassword_Errors
#[derive(Display, EnumString)]
pub enum AdminResetUserPasswordError {
    InternalErrorException,
    InvalidEmailRoleAccessPolicyException,
    InvalidLambdaResponseException,
    InvalidParameterException,
    InvalidSmsRoleAccessPolicyException,
    InvalidSmsRoleTrustRelationshipException,
    LimitExceededException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UnexpectedLambdaException,
    UserLambdaValidationException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminResetUserPasswordError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminResetUserPasswordError::InvalidEmailRoleAccessPolicyException
            | AdminResetUserPasswordError::InvalidParameterException
            | AdminResetUserPasswordError::InvalidLambdaResponseException
            | AdminResetUserPasswordError::InvalidSmsRoleAccessPolicyException
            | AdminResetUserPasswordError::InvalidSmsRoleTrustRelationshipException
            | AdminResetUserPasswordError::NotAuthorizedException
            | AdminResetUserPasswordError::LimitExceededException
            | AdminResetUserPasswordError::ResourceNotFoundException
            | AdminResetUserPasswordError::TooManyRequestsException
            | AdminResetUserPasswordError::UnexpectedLambdaException
            | AdminResetUserPasswordError::UserLambdaValidationException
            | AdminResetUserPasswordError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminResetUserPasswordRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminResetUserPasswordRequest {
    fn to_action_name() -> &'static str {
        ADMIN_RESET_USER_PASSWORD_NAME
    }
}

impl super::ToResponse for AdminResetUserPasswordRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminResetUserPasswordRequest, AdminResetUserPasswordError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminResetUserPasswordError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::empty_body())
            .unwrap()
    }
}

/// Validates request.
fn valid_request(request: &AdminResetUserPasswordRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminResetUserPasswordRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminResetUserPasswordRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminResetUserPasswordError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminResetUserPasswordError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}