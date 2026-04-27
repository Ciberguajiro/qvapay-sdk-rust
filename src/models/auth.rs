use serde::{Deserialize, Serialize};

pub struct MerchantSecret {
    pub app_id: String,
    pub app_secret: String,
}

/// Request for login operation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthLoginRequest {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
    pub two_factor_code: Option<String>,
}

/// Request for Request Pin.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestPinRequest {
    pub email: String,
    pub password: String,
}

/// Response for login when 2FA is required.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse2FA {
    pub info: String,
    pub notified: bool,
    pub has_otp: bool,
}

/// Successful login response.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub token_type: String,
    pub me: Profile,
}

/// User profile information.
#[derive(Deserialize, Debug, Clone)]
pub struct Profile {
    pub uuid: String,
    pub username: String,
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub two_factor_reset_code: String,
    pub bio: String,
    pub address: String,
    pub image: String,
    pub cover: Option<String>,
    pub balance: String,
    pub pending_balance: String,
    pub satoshis: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub phone: String,
    pub phone_verified: String,
    pub telegram: String,
    pub twitter: Option<String>,
    pub kyc: bool,
    pub kyc_status: String,
    pub vip: bool,
    pub golden_check: bool,
    pub referral_source: Option<String>,
    pub registration_platform: Option<String>,
    pub pin: i32,
    pub last_seen: String,
    pub telegram_id: String,
    pub role: String,
    pub p2p_enabled: bool,
}

/// Request for registration operation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthRegisterRequest {
    pub name: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub invite: Option<String>,
    pub terms: String,
}

/// Successful registration response.
#[derive(Deserialize, Debug, Clone)]
pub struct AuthRegisterResponse {
    pub message: String,
    pub user: Profile,
}

/// Successful registration confirm response.
#[derive(Deserialize, Debug, Clone)]
pub struct GeneralMessageResponse {
    pub message: String,
}

/// Successful check login.
#[derive(Deserialize, Debug, Clone)]
pub struct CheckLoginResponse {
    pub success: String,
}

/// Confirm Registration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmRegistrationRequest {
    pub uuid: String,
    pub email: String,
    pub pin: String,
}
