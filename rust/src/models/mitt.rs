use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Trip {
    delay: Option<String>,
    #[serde(rename = "directionId")]
    direction_id: u8,
    #[serde(rename = "indiceCorsaInLista")]
    indice_corsa_in_lista: u8,
    #[serde(rename = "lastEventReceivedAt")]
    last_event_received_at: Option<String>,
    #[serde(rename = "oraArrivoEffetttivaAFermataSelezionata")]
    ora_arrivo_effettiva: String,
    #[serde(rename = "oraArrivoProgrammataAFermataSelezionata")]
    ora_arrivo_programmata: String,
    #[serde(rename = "routeId")]
    route_id: u16,
    #[serde(rename = "stopLast")]
    stop_last: u32,
    #[serde(rename = "stopNext")]
    stop_next: u32,
    #[serde(rename = "totaleCorseInLista")]
    totale_corse_in_lista: u8,
    #[serde(rename = "tripFlag")]
    flag: String,
    #[serde(rename = "tripHeadsign")]
    head_sign: String,
    #[serde(rename = "tripId")]
    id: String,
    #[serde(rename = "type")]
    type_var: String,
    #[serde(rename = "wheelchairAccessible")]
    accessible: bool,
    #[serde(rename = "stopTimes")]
    stop_times: Vec<StopTime>,
}

#[derive(Deserialize, Serialize)]
pub struct StopTime {
    #[serde(rename = "arrivalTime")]
    arrival_time: String,
    #[serde(rename = "departureTime")]
    departure_time: String,
    #[serde(rename = "stopId")]
    stop_id: u16,
    #[serde(rename = "stopSequence")]
    stop_sequence: u8, // This MIGHT not be needed?
}
