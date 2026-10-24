#ifndef GENERIC_FUNCTIONS
#define GENERIC_FUNCTIONS
    void free_chunk(response &chunk);
    static size_t from_curl_to_string(void *contents, size_t size, size_t nmemb, void *userp);
    void from_string_to_json(json *j);
    void get_stops_list(CURL *curl, response &chunk, CURLcode &result);
    void get_routes_list(CURL *curl, response &chunk, CURLcode &result);
#endif

#ifndef CLOSEST_STOP_FUNCTIOND
#define CLOSEST_STOP_FUNCTIOND
    void get_closest_stops(float &lat_num, float &lon_num, json &next_runs_list, std::string &type);
    double distance(float &lat1, float &lon1, float &lat2, float &lon2);
    void order_closest_stops(json &arr);
    void shift_stops(json &arr, int idx);
#endif

#ifndef NEXT_RUNS
#define NEXT_RUNS
    void get_next_runs(CURL *curl, CURLcode &result, response &chunk, std::string &type, std::string &current_time, json &next_runs_list);
    void get_runs_list(CURL *curl, CURLcode &resutl, int code_closest, std::string &type, std::string &current_time);
#endif
