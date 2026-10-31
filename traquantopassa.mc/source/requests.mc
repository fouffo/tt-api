using Toybox.Lang;
using Toybox.PersistedContent;
using Toybox.Communications;



class Requests {
    var url = "";
    var params = {};
    var result = "";
    var calling_instance;
    function initialize(
        request_url as Lang.String,
        request_params as Lang.Dictionary
    ) {
        url = request_url;
        params = request_params;
    }

    function make_request(calling_obj) {
        calling_instance = calling_obj;

        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            :responseType =>
                Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON
        };

        Communications.makeWebRequest(
            url,
            {
                "lat" => params.get("lat"),
                "lon" => params.get("lon"),
                "type" => params.get("type"),
                "time" => params.get("time")
            },
            options,
            method(:on_receive) //what happens when the rervers sends some kind of response
        );
    }

    function on_receive(
        responseCode as Lang.Number,
        data 
            as Lang.String
            or Lang.Dictionary
            or PersistedContent.Iterator
            or Null
    ) as Void  {
        //if request is ok
        if (responseCode == 200 && data != null) {

            result = data.toString();
        }
        //if something went wrong
        else {
            result = "Error: "
                + responseCode.toString();

            if (data != null) {
                result += "\n" + data.toString();
            }
        }
        calling_instance.response_arrived(); //see Update_trips_list_view.make_request() for an explanation
    }

    function get_result_as_string() as Lang.String {
        return result;
    }

    function get_result_as_json() as Lang.Dictionary {
        var result_json = new Json(result);
        return result_json.to_json();
    }
}

class Request_handler {
    //it is my local tailscale address...
    const URL = "https://pc-andrea-win11.tail49644b.ts.net/api";
    var req;

    var next_trips_list_str = "";

    var text = "";

    var view;

    var args = {
        "lat" => "",
        "lon" => "",
        "type" => "",
        "time" => "",
    };


    function initialize(
        lat as Lang.Float,
        lon as Lang.Float,
        type as Lang.String,
        time as Lang.String,
        view_to_update
    ) {
        //init args
        args.put("lat", lat);
        args.put("lon", lon);
        args.put("type", type);
        args.put("time", time);
        view = view_to_update;
    }


    function make_request() {
        req = new Requests(URL, args);
        req.make_request(me);

    }

    function response_arrived() {
        var json_data = req.get_result_as_json();
        System.println(json_data);
        view.new_data(json_data);
    }
}