use super::ToStatusCode;
use crate::http;
use hyper::StatusCode;
use std::fmt::Display;
use strum_macros::{Display, EnumString};

/// AWS Cognito Common Errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/CommonErrors.html
#[derive(Display, EnumString)]
pub enum CommonError {
    AccessDeniedException,
    IncompleteSignature,
    InternalFailure,
    InvalidAction,
    InvalidClientTokenId,
    InvalidParameterCombination,
    InvalidParameterValue,
    InvalidQueryParameter,
    MalformedQueryString,
    MissingAction,
    MissingAuthenticationToken,
    MissingParameter,
    NotAuthorized,
    OptInRequired,
    RequestExpired,
    ServiceUnavailable,
    ThrottlingException,
    ValidationError,
}

impl ToStatusCode for CommonError {
    fn to_status_code(&self) -> StatusCode {
        match self {
            CommonError::AccessDeniedException
            | CommonError::IncompleteSignature
            | CommonError::InvalidAction
            | CommonError::InvalidParameterCombination
            | CommonError::InvalidParameterValue
            | CommonError::InvalidQueryParameter
            | CommonError::MissingAction
            | CommonError::MissingParameter
            | CommonError::NotAuthorized
            | CommonError::RequestExpired
            | CommonError::ThrottlingException
            | CommonError::ValidationError => http::status_code(400),
            CommonError::InvalidClientTokenId
            | CommonError::MissingAuthenticationToken
            | CommonError::OptInRequired => http::status_code(403),
            CommonError::MalformedQueryString => http::status_code(404),
            CommonError::InternalFailure => http::status_code(500),
            CommonError::ServiceUnavailable => http::status_code(503),
        }
    }
}

/// Response errors for any actions.
pub enum ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    ActionError(T),
    CommonError(CommonError),
}

impl<T> ToStatusCode for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    fn to_status_code(&self) -> StatusCode {
        match self {
            ResponseError::<T>::CommonError(err) => err.to_status_code(),
            ResponseError::ActionError(err) => err.to_status_code(),
        }
    }
}

impl<T> Display for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResponseError::<T>::CommonError(err) => write!(f, "{}", err.to_string()),
            ResponseError::ActionError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl<T> std::str::FromStr for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, strum::ParseError> {
        T::from_str(s)
            .map(|e| ResponseError::ActionError(e))
            .or(CommonError::from_str(s).map(|e| ResponseError::CommonError(e)))
    }
}