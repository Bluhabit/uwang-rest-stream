use validator::ValidationErrors;

use crate::common;
use crate::common::response::ErrorResponse;
use crate::entity::sea_orm_active_enums::{AuthProvider, UserStatus};
use crate::entity::user_credential;

pub fn get_readable_validation_message(
    err: Option<ValidationErrors>
) -> String {
    match err {
        None => String::from(""),
        Some(validation) => validation.field_errors().into_iter()
            .map(|(_field, b)| {
                let message: String = b.into_iter().map(|er| {
                    if er.code.eq("dob") {
                        return "Format tanggal lahir belum sesuai.".to_string();
                    }

                    if er.code.eq("gender") {
                        return "Gender tidak sesuai.".to_string();
                    }

                    let message = match er.clone().message {
                        Some(val) => val.to_string(),
                        None => er.code.to_string()
                    };
                    return format!("{} ", message);
                }).collect();
                return message;
            }).collect()
    }
}

pub fn check_account_user_status_active(
    credential: &user_credential::Model
) -> Result<user_credential::Model, ErrorResponse> {
    match credential.status {
        UserStatus::Active => Ok(credential.clone()),
        UserStatus::Locked => Err(ErrorResponse::unauthorized("Akun Anda dibatasi untuk sementara.".to_string())),
        UserStatus::Inactive => Err(ErrorResponse::unauthorized("Akun Anda tidak aktif.".to_string())),
        UserStatus::Suspended => Err(ErrorResponse::unauthorized("Akun Anda ditangguhkan, anda tidak dapat melanjutkan proses ini.".to_string())),
        UserStatus::WaitingConfirmation => {
            if credential.auth_provider == AuthProvider::Google {
                return Ok(credential.clone());
            }
            return Err(ErrorResponse::unauthorized("Akun Anda belum terkonfirmasi, silahkan cek email untuk mengkonfirmasi.".to_string()));
        }
    }
}


pub fn create_session_redis_from_user(
    user: user_credential::Model,
    token: String,
) -> Vec<(String, String)> {
    return vec![
        (common::constant::REDIS_KEY_USER_ID.to_string(), user.id),
        (common::constant::REDIS_KEY_EMAIL.to_string(), user.email),
        (common::constant::REDIS_KEY_FULL_NAME.to_string(), user.full_name),
        (common::constant::REDIS_KEY_TOKEN.to_string(), token),
    ];
}