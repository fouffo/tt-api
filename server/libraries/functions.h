#ifndef MAIN_LOOPS
#define MAIN_LOOPS
    void server_job(const httplib::Request &req, httplib::Response &res, json &next_trips_list);
    int update_stops_routes();
#endif

#ifndef GENERIC_FUNCTIONS
#define GENERIC_FUNCTIONS
    std::string get_current_time();
    void free_chunk(response &chunk);
    static size_t from_curl_to_string(void *contents, size_t size, size_t nmemb, void *userp);
    void from_string_to_json(json *j);
    void get_stops_list(CURL *curl, response &chunk, CURLcode &result);
    void get_routes_list(CURL *curl, response &chunk, CURLcode &result);
#endif

#ifndef CLOSEST_STOP_FUNCTIOND
#define CLOSEST_STOP_FUNCTIOND
    void get_closest_stops(float &lat_num, float &lon_num, std::string &type, json &next_trips_list);
    double distance(float &lat1, float &lon1, float &lat2, float &lon2);
    void order_closest_stops(json &arr);
    void shift_stops(json &arr, int idx);
#endif

#ifndef NEXT_RUNS
#define NEXT_RUNS
    void get_next_trips(CURL *curl, CURLcode &result, response &chunk, std::string &type, std::string &current_time, json &next_trips_list);
    void get_trips_list(CURL *curl, CURLcode &resutl, int code_closest, std::string &type, std::string &current_time);
#endif
