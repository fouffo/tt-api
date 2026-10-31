/*
Problem: if you download the whole folder you can't compile because it uses absolute paths to my compiler
(yes, you have full access to my file system structure)
*/
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



using Toybox.Application;
using Toybox.Communications;
using Toybox.Graphics;
using Toybox.Lang;
using Toybox.System;
using Toybox.WatchUi;
using Toybox.PersistedContent;

class Update_view extends WatchUi.View {

    var text;
    var full_data = {};

    function initialize() {
        View.initialize();
    }
    
    function new_data(dict) {   
        full_data = dict;
        reset_full_data();
        WatchUi.requestUpdate();
    }

    function reset_full_data() {
        text.setText("new data");
    }

    function onLayout(dc) {
    }

    function onShow() as Void {
        text = new Text({
            :text=>"Hello World!",
            :color=>Graphics.COLOR_WHITE,
            :font=>Graphics.FONT_LARGE,
            :locX =>WatchUi.LAYOUT_HALIGN_CENTER,
            :locY=>WatchUi.LAYOUT_VALIGN_CENTER
        });
    }

    function onUpdate(dc) {
        dc.setColor(
            Graphics.COLOR_WHITE,
            Graphics.COLOR_BLACK
        );
        dc.clear();
        if (text != null) {
            text.draw(dc);
        }
    }
}

class Buttons_handler extends WatchUi.BehaviorDelegate {

    var view;
    var gps = new Gps(); //here there will be stored the gps class
    var time = new Time();

    function initialize(get_next_trips_list_instance) {
        BehaviorDelegate.initialize();
        view = get_next_trips_list_instance;
    }

    function onSelect() {
        //depending on where is the current view there will be some ifs
        var req = new Request_handler(
            gps.get_lat(),
            gps.get_lon(),
            "U",
            time.time_TT(),
            view
        );
        req.make_request(); //doesn't show anything
        return true;
    }
}

class Traquantopassa extends Application.AppBase {

    function initialize() {
        AppBase.initialize();

    }

    function getInitialView() {
        var view = new Update_view();
        return [
            view,
            new Buttons_handler(view)
        ];
    }
}
