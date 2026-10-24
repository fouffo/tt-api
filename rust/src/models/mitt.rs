use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Trip {

}

/*
{
    "delay": null,
    "directionId": 1,
    "indiceCorsaInLista": 0,
    "lastEventRecivedAt": null,
    "oraArrivoEffettivaAFermataSelezionata": "2023-01-13T11:17:00Z",
    "oraArrivoProgrammataAFermataSelezionata": "2023-01-13T11:17:00Z",
    "routeId": 536,
    "stopLast": 0,
    "stopNext": 0,
    "stopTimes": [
        {
            "arrivalTime": "12:16:00",
            "departureTime": "12:16:00",
            "stopId": 235,
            "stopSequence": 1,
            "tripId": "0003804452022091220230609",
            "type": "U"
        },
        {
            "arrivalTime": "12:17:00",
            "departureTime": "12:17:00",
            "stopId": 231,
            "stopSequence": 2,
            "tripId": "0003804452022091220230609",
            "type": "U"
        },
        // ...omitted...
        {
            "arrivalTime": "12:44:00",
            "departureTime": "12:44:00",
            "stopId": 284,
            "stopSequence": 19,
            "tripId": "0003804452022091220230609",
            "type": "U"
        }
    ],
    "totaleCorseInLista": 0,
    "tripFlag": "TRIP_FLAG__START",
    "tripHeadsign": "Bolghera \"S.Antonio\"",
    "tripId": "0003804452022091220230609",
    "type": "U",
    "wheelchairAccessible": 1
}
*/
