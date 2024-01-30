use actix_web::{HttpRequest, Responder, web};
use actix_web::Result as WebResult;
use actix_web::web::Json;
use validator::Validate;
use crate::AppState;
use crate::common::response::{BaseResponse, ErrorResponse};
use crate::models::event::SendEventRequest;

pub(crate) async fn register_event(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
    let header = req
        .headers()
        .get("context");
    if header.is_none() {
        return state.sse_emitter.reject_client().await;
    }
    let id =
        header.unwrap().to_str().unwrap();

    return state.sse_emitter.new_client(id).await;
}

pub(crate) async fn send(
    state: web::Data<AppState>,
    body: web::Json<SendEventRequest>,
) -> WebResult<impl Responder, ErrorResponse> {
    let validate = body.validate();
    if validate.is_err() {
        return Err(ErrorResponse::bad_request(
            400,
            validate.unwrap_err().to_string(),
        ));
    }

    let sent = state.sse_emitter
        .send_to(
            body.to.as_str(),
            body.event_name.as_str(),
            body.message.as_str(),
        ).await;

    if sent.is_none(){
        return Err(ErrorResponse::bad_request(
            400,
            format!("User {} is not active",body.to),
        ));
    }

    return Ok(Json(BaseResponse::success(
        200,
        Some(body),
        "".to_string(),
    )));
}

pub(crate) async fn broadcast(
    state: web::Data<AppState>,
    body: Json<SendEventRequest>,
) -> WebResult<impl Responder, ErrorResponse> {
    let generate = "1234";
    let validate = body.validate();

    if validate.is_err() {
        return Err(ErrorResponse::bad_request(
            400,
            validate.unwrap_err().to_string(),
        ));
    }

    let sent = state
        .sse_emitter
        .broadcast(body.event_name.as_str(), body.message.as_str())
        .await;

    if sent.is_none(){
        return Err(ErrorResponse::bad_request(
            400,
            format!("No one active at current time {}",generate),
        ));
    }

    return Ok(Json(BaseResponse::success(
        200,
        Some(body),
        "".to_string(),
    )));
}