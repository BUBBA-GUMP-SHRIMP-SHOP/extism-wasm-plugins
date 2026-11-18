use extism_pdk::{FnResult, Json, plugin_fn};
use serde::Deserialize;

mod gain;

#[derive(Deserialize)]
struct GainInput {
    former: f64,
    later: f64,
}

#[plugin_fn]
pub fn gain(Json(input): Json<GainInput>) -> FnResult<Json<Option<f64>>> {
    Ok(Json(gain::gain(input.former, input.later)))
}

#[derive(Deserialize)]
struct GainInvInput {
    former: f64,
    gain: f64,
}

#[plugin_fn]
pub fn gain_inv(Json(input): Json<GainInvInput>) -> FnResult<Json<Option<f64>>> {
    Ok(Json(gain::gain_inv(input.former, input.gain)))
}
