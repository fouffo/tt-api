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

    function pos_not_found() {
        text.setText("waiting for position");
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
            :font=>Graphics.FONT_SMALL,
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
    var gps; //here there will be stored the gps class
    var time_var = new Time_utils();

    var selected_what = "nothing";


    function initialize(get_next_trips_list_instance) {
        BehaviorDelegate.initialize();
        view = get_next_trips_list_instance;
        gps = new Gps(me);

    }

    function update_data(data) {
        if (!data.isEmpty()) {
            
            view.new_data(data);
        }
    }

    function onSelect() {
        //depending on where is the current view there will be some ifs
        if (gps.lat != 0 && gps.lon != 0) { //if the location has been already found
            var req = new Requests(
                gps.lat,
                gps.lon,
                "U",
                time_var.time_utc(),
                me
            );
            System.println(gps.lat);
            System.println(gps.lon);
            req.make_request(); //doesn't show anything
            return true;
        }
        else { //once the location has been found the Gps.on_position calls onSelect as if the user
               //has pressed the select button so it refreshes the data
               //there must be added a control on what the user is actually pressing
            view.pos_not_found();
            return true;
        }
        
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
