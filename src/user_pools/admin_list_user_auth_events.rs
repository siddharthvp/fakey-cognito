use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_LIST_USER_AUTH_EVENTS_NAME: &str = "AdminListUserAuthEvents";
pub const ADMIN_LIST_USER_AUTH_EVENTS_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminListUserAuthEvents";

/// AdminListUserAuthEvents response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminListUserAuthEvents.html#API_AdminListUserAuthEvents_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminListUserAuthEventsError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
    UserPoolAddOnNotEnabledException,
}

impl super::ToStatusCode for AdminListUserAuthEventsError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminListUserAuthEventsError::InvalidParameterException
            | AdminListUserAuthEventsError::NotAuthorizedException
            | AdminListUserAuthEventsError::ResourceNotFoundException
            | AdminListUserAuthEventsError::TooManyRequestsException
            | AdminListUserAuthEventsError::UserNotFoundException
            | AdminListUserAuthEventsError::UserPoolAddOnNotEnabledException => {
                http::status_code(400)
            }
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListUserAuthEventsRequest {
    pub max_results: Option<u8>,
    pub next_token: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminListUserAuthEventsRequest {
    fn to_action_name() -> &'static str {
        ADMIN_LIST_USER_AUTH_EVENTS_NAME
    }
}

impl super::ToResponse for AdminListUserAuthEventsRequest {
    type E = AdminListUserAuthEventsError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_LIST_USER_AUTH_EVENTS_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminListUserAuthEventsRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminListUserAuthEventsRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminListUserAuthEventsRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminListUserAuthEventsError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminListUserAuthEventsError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
