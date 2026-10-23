//headers and global variables
#include "./libraries/headers.h"

//MAIN LOOP

//runs the server and waits for lat and lon
int main() {
    //init server
    httplib::Server svr;

    //waits for the server to receive a lat and lon
    svr.Get("/api", [](const httplib::Request &req, httplib::Response &res) {
        //vars for the request

        //parameters from the watch
        std::string type;
        std::string current_time = "2026-10-21T17:50:00Z"; //REMEMBER TO SUBTRACT 2H BECAUSE IT IS IN GMT+0
        //IT MEANS YOU HAVE TO SCAN THE WOLE RESPONSE TO GET THE PROPER LINES AT THE PROPER TIME


        float lat = 46.0719f;
        float lon = 11.1196f;

        //curl structures 
        CURL *curl; //curl main structure
        CURLcode result; //response code from the server
        struct response chunk = { //contains the value curl stores in a buffer
            .memory = (char*)malloc(0),
            .size = 0};



        json next_runs_list;
        next_runs_list = {
            {"stopId", -1},
            {"stopName", ""},
            {"nextTrips", json::array()}
        };




        //initial curl setup
        result = curl_global_init(CURL_GLOBAL_ALL);
        curl = curl_easy_init();
        curl_easy_setopt(curl, CURLOPT_USERNAME, USR.c_str());
        curl_easy_setopt(curl, CURLOPT_PASSWORD, PSW.c_str());
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, from_curl_to_string);
        curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);




        //counter for the requests
        count++;
        if (count % 10 == 0) {
            count = 0;
            get_stops_list(curl, chunk, result);
            get_routes_list(curl, chunk, result);
        }





        //checks the latitude (default Stazione)
        if (req.has_param("lat")) {
            lat = stof(req.get_param_value("lat"));
        }

        //checks the longitude (default Stazione)
        if (req.has_param("lon")) {
            lon = stof(req.get_param_value("lon"));
        }

        //type of line (Urbana default)
        if (req.has_param("type")) {
            type = req.get_param_value("type");
        }
        else {
            type = "U";
        }

        //time of the request
        if (req.has_param("time")) {
            current_time = req.get_param_value("time");
        }
        else {
            current_time = "2026-10-19T07:50:00Z"; //to be changed with the time of the server
        }





        //gets the closest stop
        get_closest_stop(lat, lon, next_runs_list);

        //gets the next five runs from that stop
        get_next_runs(curl, result, chunk, type, current_time, next_runs_list);

        //shows the stop code
        res.set_content(to_string(next_runs_list), "text/plain");
    });
    svr.listen("localhost", 1234);
}









//BASIC FUNCTIONS

//gets the buffer from the curl requests and stores it into the memory chunk
//
void free_chunk(response &chunk) {
    free(chunk.memory);
    chunk.memory = nullptr;
    chunk.size = 0;
}

static size_t from_curl_to_string(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct response *mem = (struct response *)userp;

    char *ptr = (char *)realloc(mem->memory, mem->size + realsize + 1);
    if(!ptr) {
        /* Out of memory! */
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

//converts the string into a json object and frees the memory
void from_string_to_json(response &chunk, json &j) {
    j = json::parse(chunk.memory);
    free_chunk(chunk);
}




//gets all the stops
void get_stops_list(CURL *curl, response &chunk, CURLcode &result) {
    std::cout << "GET stop LISTS" << std::endl;
    curl_easy_setopt(curl, CURLOPT_URL, (URL + "/stops").c_str());
    result = curl_easy_perform(curl);

    json full_data;
    from_string_to_json(chunk, full_data);

    for (const auto &el : full_data) {
        all_stops.push_back({
            {"stopId", el["stopId"]},
            {"stopName", el["stopName"]},
            {"stopLat", el["stopLat"]},
            {"stopLon", el["stopLon"]}
        });
    }
}

//gets all the routes
void get_routes_list(CURL *curl, response &chunk, CURLcode &result) {
    std::cout << "GET LISTS" << std::endl;
    curl_easy_setopt(curl, CURLOPT_URL, (URL + "/routes").c_str());
    result = curl_easy_perform(curl);

    json full_data;
    from_string_to_json(chunk, full_data);

    for (const auto &el : full_data) {
        all_routes.push_back({
            {"routeId", el["routeId"]},
            {"routeShortName", el["routeShortName"]},
            {"routeLongName", el["routeLongName"]},
            {"routeColor", el["routeColor"]}
        });
    }
}








//FUNCTIONS TO GET THE CLOSEST STOP

//gets the closest stop
void get_closest_stop(float &lat_num, float &lon_num, json &next_runs_list) {
    float stop_lat = 0.0f;
    float stop_lon = 0.0f;
    double dist_min = 1000000000000.0; //initialized with a number too big to be true (in meters)
    double t = 0.0;


    lat_num *= PI / 180;
    lon_num *= PI /180;
    for (const auto &el : all_stops) {
        stop_lat = el["stopLat"].get<float>();
        stop_lon = el["stopLon"].get<float>();
        t = distance(stop_lat, stop_lon, lat_num, lon_num);
        if (t < dist_min) {
            dist_min = t;
            next_runs_list["stopId"] = el["stopId"].get<int>();
            next_runs_list["stopName"] = el["stopName"];
        }
    }
}

//calculates the distance between the position coords annd the stop coords
double distance(float &lat1, float &lon1, float &lat2, float &lon2) {
    lat1 *= PI / 180;
    lon1 *= PI / 180;

    float delta_lat = lat1 - lat2;
    float delta_lon = lon1 - lon2;
    return 2 * R_EARTH * asin(sqrt(
                pow(
                    sin(delta_lat / 2),
                    2
                )
                +
                cos(lat1) * cos(lat2) * pow(
                    sin(delta_lon / 2),
                    2
                    )
                ));
}








//FUNCTIONS TO GET THE NEXT RUNS FROM THE CLOSEST STOP

void get_next_runs(CURL *curl, CURLcode &result, response &chunk, std::string &type, std::string &current_time, json &next_runs_list) {
    get_runs_list(curl, result, next_runs_list["stopId"], type, current_time);

    json full_runs;
    from_string_to_json(chunk, full_runs);


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

void get_runs_list(CURL *curl, CURLcode &result, int code_closest, std::string &type, std::string &current_time) {
    //replaces ":" with "%3A" (url formatting)
    std::string::size_type pos = 0;
    while ((pos = current_time.find(':', pos)) != std::string::npos) {
        current_time.replace(pos, 1, "%3A");
        pos += 3;
    }
    std::string new_url = URL +
                          "/trips_new?stopId=" + std::to_string(code_closest) +
                          "&type=" + type +
                          "&limit=" + std::to_string(limit) +
                          "&refDateTime=" + current_time;
    std::cout << new_url << std::endl;
    curl_easy_setopt(curl, CURLOPT_URL, new_url.c_str());
    result = curl_easy_perform(curl);
}
