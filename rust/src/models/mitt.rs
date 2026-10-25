use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Trip {
    pub delay: Option<String>,
    #[serde(rename = "directionId")]
    pub direction_id: u8,
    #[serde(rename = "indiceCorsaInLista")]
    pub indice_corsa_in_lista: u8,
    #[serde(rename = "lastEventReceivedAt")]
    pub last_event_received_at: Option<String>,
    #[serde(rename = "oraArrivoEffetttivaAFermataSelezionata")]
    pub ora_arrivo_effettiva: Option<String>,
    #[serde(rename = "oraArrivoProgrammataAFermataSelezionata")]
    pub ora_arrivo_programmata: Option<String>,
    #[serde(rename = "routeId")]
    pub route_id: u16,
    #[serde(rename = "stopLast")]
    pub stop_last: u16,
    #[serde(rename = "stopNext")]
    pub stop_next: u16,
    #[serde(rename = "totaleCorseInLista")]
    pub totale_corse_in_lista: u8,
    #[serde(rename = "tripFlag")]
    pub flag: String,
    #[serde(rename = "tripHeadsign")]
    pub head_sign: String,
    #[serde(rename = "tripId")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_var: String,
    #[serde(rename = "wheelchairAccessible")]
    pub accessible: u8, // might become a bool?
    #[serde(rename = "stopTimes")]
    pub stop_times: Vec<StopTime>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StopTime {
    #[serde(rename = "arrivalTime")]
    pub arrival_time: String,
    #[serde(rename = "departureTime")]
    pub departure_time: String,
    #[serde(rename = "stopId")]
    pub stop_id: u16,
    #[serde(rename = "stopSequence")]
    pub stop_sequence: u8, // This MIGHT not be needed?
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stop {
    pub distance: Option<String>, // TODO: check if this should actually not be a string
    #[serde(rename = "stopCode")]
    pub code: String,
    #[serde(rename = "stopDesc")]
    pub desc: String,
    #[serde(rename = "stopId")]
    pub id: u16,
    #[serde(rename = "stopLat")]
    pub lat: f32,
    #[serde(rename = "stopLon")]
    pub lon: f32,
    #[serde(rename = "stopLevel")]
    pub level: i16,
    #[serde(rename = "stopName")]
    pub name: String,
    pub street: Option<String>,
    pub town: Option<String>,
    #[serde(rename = "type")]
    pub type_var: String,
    #[serde(rename = "wheelchairBoarding")]
    pub wheelchair_boarding: i8, // Could be useful to distinguish big buses from small ones
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Route {
    #[serde(rename = "areaId")]
    pub area_id: i32, // This MAY be replaced with a smaller integer type. For now this is not a priority
    pub news: Option<Vec<News>>,
    #[serde(rename = "routeColor")]
    pub color: Option<String>,
    #[serde(rename = "routeId")]
    pub id: u16,
    #[serde(rename = "routeLongName")]
    pub long_name: String,
    #[serde(rename = "routeShortName")]
    pub short_name: String,
    #[serde(rename = "routeType")]
    pub route_type: u8, // WHY IS THIS A NUMBER!?
    #[serde(rename = "type")]
    pub type_var: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct News {
    #[serde(rename = "agencyId")]
    pub agency_id: String,
    pub details: String,
    #[serde(rename = "endDate")]
    pub end_date: String, // Keep in mind this is formatted in a peculiar "/Date(1673654400)/"
    pub header: String,
    #[serde(rename = "idFeed")]
    pub id_feed: Option<u32>,
    #[serde(rename = "routeIDs")]
    pub route_ids: Option<Vec<i16>>,
    #[serde(rename = "serviceType")]
    pub service_type: String,
    #[serde(rename = "startDate")]
    pub start_date: String, // Same special format as the end date
    #[serde(rename = "stopId")]
    pub stop_id: String,
    pub url: String,
}
