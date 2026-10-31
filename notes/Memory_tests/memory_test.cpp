#include "./httplib.h"
#include <iostream>
#include "./json.hpp"

    
int main() {
    //init server
    httplib::Server svr;
    using json = nlohmann::json;
    svr.Get("/api", [](const httplib::Request &req,
                       httplib::Response &res) {

        {
            std::string testo(10 * 1024 * 1024, 'A');
            json risposta = json::array();

            for (int i = 0; i < 10000; i++) {
                risposta.push_back({
                    {"id", i},
                    {"value", "test"}
                });
            }
        }

        res.set_content("OK", "text/plain");
    });
    svr.listen("127.0.0.1", 1265);
}