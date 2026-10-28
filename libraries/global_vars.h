#ifndef GLOBAL_VARS
#define GLOBAL_VARS
    //class for json
    using json = nlohmann::json;

    //response structure
    struct response {
        char *memory;
        size_t size;
    };

    //for soome reason httplib wants the variables to either
    //lie inside the srv.Get or to be global. since i don't want
    //them to lie in the get i declared them as global

    //url and credentials for Trentino Trasporti API request
    const std::string URL = "https://app-tpl.tndigit.it/gtlservice";
    const std::string USR = "mittmobile";
    const std::string PSW = "ecGsp.RHB3";

    //server variables
    const std::string HOST = "192.168.1.11";
    const int PORT = 1234;

    //vars for the api request for the stops and the routes
    char count = -1;
    json all_stops = json::array();
    json all_routes = json::array();

    //limits for the stops and the trips
    const int MAX_CLOSEST_STOPS = 5;
    int MAX_TRIPS_PER_STOP = 5;

    //constants for the distance
    const double PI = 3.14159265358;
    const int R_EARTH = 6371000;
#endif

