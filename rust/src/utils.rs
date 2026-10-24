use crate::models;
use reqwest::{Response};
use base64::prelude::*;
use std::collections::HashMap;

const USR: &str = "mittmobile";
const PSW: &str = "ecGsp.RHB3";
const URL: &str = "https://app-tpl.tndigit.it/gtlservice";

pub fn get_closest_stop(lat_num: f32, lon_num: f32, stops: Vec<models::internal::Stop>) -> models::internal::NextRunsList {
    let mut stop_lat: f32;
    let mut stop_lon: f32;
    let mut dist_min = f32::INFINITY;
    let mut dist: f32;
    let mut closest_stop = models::internal::NextRunsList {
        stop_id: u32::MAX,
        stop_name: String::new(),
        next_trips: vec![],
    };

    for stop in stops {
        stop_lat = stop.lat;
        stop_lon = stop.lon;
        dist = distance(stop_lat, stop_lon, lat_num, lon_num);
        if dist < dist_min {
            dist_min = dist;
            closest_stop.stop_id = stop.id;
            closest_stop.stop_name = stop.name;
        }
    }
    closest_stop
}

//calculates the distance between the position coords annd the stop coords
fn distance(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let lat1 = lat1 * std::f32::consts::PI / 180.0;
    let lon1 = lon1 * std::f32::consts::PI / 180.0;
    let lat2 = lat2 * std::f32::consts::PI / 180.0;
    let lon2 = lon2 * std::f32::consts::PI / 180.0;

    let delta_lat = lat1 - lat2;
    let delta_lon = lon1 - lon2;

    12742000.0 // This magic number is the diameter of the earth in meters.
        * (
                (delta_lat / 2.0).sin().powi(2) +
                lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2)
            )
            .sqrt()
            .asin()
}

pub async fn request(path: &str, params: HashMap<&str, &str>) -> Option<Response> {
    let mut params_str = String::from("?");
    for pair in params {
        params_str += &*format!("{}={}&", pair.0, pair.1);
    }
    params_str.pop(); // removes that last &
    params_str = params_str.replace(":", "%3A");
    let url = String::from(URL) + path + &params_str;
    println!("{}", url);
    let token = format!("Basic {}", BASE64_STANDARD.encode(format!("{}:{}", USR, PSW)));
    let client = reqwest::Client::new();
    let res = client.get(url)
        .header("Authorization", token)
        .send()
        .await
        .ok();
    Option::from(res)
}

pub async fn get_stop_list() -> Vec<models::mitt::Stop> {
    let res = request("/stops", HashMap::new()).await.unwrap().text().await.unwrap();
    serde_json::from_str(&res).expect("Serde couldn't deserialize the stop list.")
}

pub async fn get_route_list() -> Vec<models::mitt::Route> {
    let res = request("/routes", HashMap::new()).await.unwrap().text().await.unwrap();
    serde_json::from_str(&res).expect("Serde couldn't deserialize the route list.")
}

pub async fn get_run_list(closest_id: u32, type_str: &str, limit: u8, time: &str) -> Vec<models::mitt::Trip> {
    let res = request("/trips_new", HashMap::from([
        ("stopId", &*closest_id.to_string()),
        ("type", type_str),
        ("limit", &*limit.to_string()),
        ("refDateTime", time)])).await.unwrap().text().await.unwrap();
    serde_json::from_str(&res).expect("Serde couldn't deserialize the run list.")
}

pub async fn get_next_runs(type_str: &str, current_time: &str, stop_id: String){

}

/*

void get_next_runs(CURL *curl, CURLcode &result, response &chunk, std::string &type, std::string &current_time, json &next_runs_list) {
    get_runs_list(curl, result, next_runs_list["stopId"], type, current_time);
    //gets only the necessary data from the next runs
    int route_id = -1;
    std::string arrival_time = "";

    //cycles all the runs from the stop
    for (const auto &run : full_runs) {

        route_id = run["routeId"].get<int>();
        //cycles all the routes to chech which one has the same id
        for (const auto &route : all_routes) {

            //if the id coincide it gets the data from the routes
            if (route_id == route["routeId"].get<int>()) {

                //cycles for all the stops a trip does
                for (const auto &single_stop_data : run["stopTimes"]) {

                    //if the stop id is the same as the one in the schedule trip it gets the scheduled arrival time
                    if (next_runs_list["stopId"] == single_stop_data["stopId"]) {
                        arrival_time = single_stop_data["arrivalTime"];
                        break;
                    }
                }

                //builds the array
                next_runs_list["nextTrips"].push_back({
                    {"routeColor", route["routeColor"]},
                    {"routeShortName", route["routeShortName"]},
                    {"routeLongName", route["routeLongName"]},
                    {"delay", run["delay"]},
                    {"arrivalTime", arrival_time}
                });
            }
        }
    }
}
*/
