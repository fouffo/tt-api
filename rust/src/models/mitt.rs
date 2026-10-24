use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Stop {
    distance: Option<String>, // TODO: check if this should actually not be a string
    #[serde(rename = "stopCode")]
    code: String,
    #[serde(rename = "stopDesc")]
    desc: String,
    #[serde(rename = "stopId")]
    id: u16,
    #[serde(rename = "stopLat")]
    lat: f32,
    #[serde(rename = "stopLon")]
    lon: f32,
    #[serde(rename = "stopLevel")]
    level: i16,
    #[serde(rename = "stopName")]
    name: String,
    street: Option<String>,
    town: Option<String>,
    #[serde(rename = "type")]
    type_var: String,
    #[serde(rename = "wheelchairBoarding")]
    wheelchair_boarding: i8, // Could be useful to distinguish big buses from small ones
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Route {
    #[serde(rename = "areaId")]
    area_id: i32, // This MAY be replaced with a smaller integer type. For now this is not a priority
    news: Option<Vec<News>>,
    #[serde(rename = "routeColor")]
    color: Option<String>,
    #[serde(rename = "routeId")]
    id: u16,
    #[serde(rename = "routeLongName")]
    long_name: String,
    #[serde(rename = "routeShortName")]
    short_name: String,
    #[serde(rename = "routeType")]
    route_type: u8, // WHY IS THIS A NUMBER!?
    #[serde(rename = "type")]
    type_var: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct News {
    #[serde(rename = "agencyId")]
    agency_id: String,
    details: String,
    #[serde(rename = "endDate")]
    end_date: String, // Keep in mind this is formatted in a peculiar "/Date(1673654400)/"
    header: String,
    #[serde(rename = "idFeed")]
    id_feed: Option<u32>,
    #[serde(rename = "routeIDs")]
    route_ids: Option<Vec<i16>>,
    #[serde(rename = "serviceType")]
    service_type: String,
    #[serde(rename = "startDate")]
    start_date: String, // Same special format as the end date
    #[serde(rename = "stopId")]
    stop_id: String,
    url: String,
}
