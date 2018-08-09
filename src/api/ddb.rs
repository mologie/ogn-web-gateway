use std::collections::HashMap;

use actix_web::*;
use futures::future::Future;

use ::app::AppState;
use ::db;

#[derive(Serialize)]
struct DeviceInfo {
    pub model: Option<String>,
    pub category: i16,
    pub registration: Option<String>,
    pub callsign: Option<String>,
}

pub fn get((state, request): (State<AppState>, HttpRequest<AppState>)) -> impl Responder {
    state.db.send(db::ReadOGNDevices).from_err::<Error>()
        .and_then(move |devices| {
            let devices = match devices {
                None => HashMap::new(),
                Some(devices) => devices.into_iter()
                    .map(|d| (d.ogn_id, DeviceInfo {
                        model: d.model,
                        category: d.category,
                        registration: d.registration,
                        callsign: d.callsign,
                    }))
                    .collect(),
            };

            let response = request
                .build_response(http::StatusCode::OK)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
                .json(devices);

            Ok(response)
        })
        .responder()
}
