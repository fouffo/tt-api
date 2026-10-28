using Toybox.Application;
using Toybox.Communications;
using Toybox.Graphics;
using Toybox.Lang;
using Toybox.System;
using Toybox.WatchUi;
using Toybox.PersistedContent;


class Get_next_trips_list extends WatchUi.View {

    // Sostituisci con l'indirizzo del tuo endpoint
    const URL = "https://jsonplaceholder.typicode.com/posts/1";//"http://192.168.1.11:1234/api"; // HTTPS NEEDED

    var next_trips_list = "Premi START per richiedere il JSON";

    var lat;
    var lon;
    var type;
    var time;

    function initialize(
        lat_arg as Lang.Float,
        lon_arg as Lang.Float,
        type_arg as Lang.String,
        time_arg as Lang.String
    ) {
        View.initialize();
        //init args
        lat = lat_arg;
        lon = lon_arg;
        type = type_arg;
        time = time_arg;

        System.println(lat);
        makeRequest();
    }

    function onLayout(dc) {
    }

    function makeRequest() {
        next_trips_list = "Richiesta in corso...";
        WatchUi.requestUpdate();

        var options = {
            :method => Communications.HTTP_REQUEST_METHOD_GET,
            :responseType =>
                Communications.HTTP_RESPONSE_CONTENT_TYPE_JSON
        };

        Communications.makeWebRequest(
            URL,
            {},
            /*{
                "lat" => lat,
                "lon" => lon,
                "type" => type,
                "time" => time
            },*/
            options,
            method(:onReceive) //what happens when the rervers sends some kind of response
        );
    }

    function onReceive(
        responseCode as Lang.Number,
        data 
            as Lang.String
            or Lang.Dictionary
            or PersistedContent.Iterator
            or Null
    ) as Void  {
        if (responseCode == 200 && data != null) {
            // La risposta viene mantenuta come testo grezzo
            next_trips_list = data.toString();
        } else {
            next_trips_list = "Errore HTTP: "
                + responseCode.toString();

            if (data != null) {
                next_trips_list += "\n" + data.toString();
            }
        }
        System.println(next_trips_list);
        WatchUi.requestUpdate();
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

    function initialize(next_trips_list) {
        BehaviorDelegate.initialize();
        view = next_trips_list;
    }

    function onSelect() {
        view.makeRequest(); //doesn't show anything
        return true;
    }
}

class Traquantopassa extends Application.AppBase {

    function initialize() {
        AppBase.initialize();

    }

    function getInitialView() {
        var view = new Get_next_trips_list(
            46.0675,
            11.1504,
            "U",
            "2026-10-28T13:32:00Z"
        );
        return [
            view,
            new Buttons_handler(view)
        ];
    }
}
