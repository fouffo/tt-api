using Toybox.Lang;
using Toybox.PersistedContent;
using Toybox.Communications;
using Toybox.Time;

class Requests {
    //it is my local tailscale address...
    const URL = "https://pc-andrea-win11.tail49644b.ts.net/api";
    var req;
    var res;

    var next_trips_list_str = "";

    var text = "";

    var buttons_instance;

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
        buttons
    ) {
        //init args
        args.put("lat", lat);
        args.put("lon", lon);
        args.put("type", type);
        args.put("time", time);
        buttons_instance = buttons;
    }

    function make_request() {

        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            :responseType =>
                Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON
        };
        System.println(("here"));

        Communications.makeWebRequest(
            URL,
            args,
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

            res = data;
        }
        //if something went wrong
        else {
            res = ["Error: " + responseCode.toString()];
        }
        buttons_instance.update_data(res);
    }
}