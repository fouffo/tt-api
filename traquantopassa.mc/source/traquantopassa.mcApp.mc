/*

Problem: if you download the whole folder you can't compile because it uses absolute paths to my compiler
(yes, you have full access to my file system structure)


*/


using Toybox.Application;
using Toybox.Communications;
using Toybox.Graphics;
using Toybox.Lang;
using Toybox.System;
using Toybox.WatchUi;
using Toybox.PersistedContent;

class Json_parser {
    var json_data = "";
    var str_data = "";
    function initialize(str) {
        str_data = str;
    }
    function to_json() as Lang.Dictionary {
        return {"str" => str_data};
    }
}

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
        calling_instance.check(); //see Update_trips_list_view.make_request() for an explanation
    }

    function get_result_as_string() as Lang.String {
        return result;
    }

    function get_result_as_json() as Lang.Dictionary {
        var result_json = new Json_parser(result);
        return result_json.to_json();
    }
}

class Update_trips_list_view extends WatchUi.View {

    //it is my local tailscale address...
    const URL = "https://pc-andrea-win11.tail49644b.ts.net/api";
    var req;

    var next_trips_list_str = "";

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
        time as Lang.String
    ) {
        View.initialize();
        //init args
        args.put("lat", lat);
        args.put("lon", lon);
        args.put("type", type);
        args.put("time", time);
    }


    function make_request() {
        req = new Requests(URL, args);
        //since for the developers it is single threaded but the default library is multithreaded
        //i have to pass the object making the request to the request function so that once it
        //starts the request it can call the method inside that class that handles the response
        //the point of passing me is that i want to keep executing the code only whey i have an
        //usable answare, so once the on_response is called it calls back this instance and keeps
        //going with it. for simplicity:
        //user presses play -> created Update... -> called make_request -> created Requests ->
        //called its make_request function -> makes the request -> calls on_receive to handle the
        //the response -> when it arrives, on_receive calls check from the Update... instance that
        //created Request
        req.make_request(me); //me means exactly what you imagine

    }

    function check() {
        //var str_data = req.get_result_as_string();
        var json_data = req.get_result_as_json();
        System.println(json_data);

        WatchUi.requestUpdate();
    }

    function onLayout(dc) {
    }

    

    function onUpdate(dc) {
        dc.setColor(
            Graphics.COLOR_WHITE,
            Graphics.COLOR_BLACK
        );
        dc.clear();
    }
}

class Buttons_handler extends WatchUi.BehaviorDelegate {

    var view;

    function initialize(get_next_trips_list_instance) {
        BehaviorDelegate.initialize();
        view = get_next_trips_list_instance;
    }

    function onSelect() {
        view.make_request(); //doesn't show anything
        return true;
    }
}

class Traquantopassa extends Application.AppBase {

    function initialize() {
        AppBase.initialize();

    }

    function getInitialView() {
        var view = new Update_trips_list_view(
            46.0675,
            11.1504,
            "U",
            "2026-10-30T16:32:00Z"
        );
        return [
            view,
            new Buttons_handler(view)
        ];
    }
}
