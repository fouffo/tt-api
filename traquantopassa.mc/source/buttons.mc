using Toybox.WatchUi;

class Buttons extends WatchUi.BehaviorDelegate {

    var view;
    var gps; //here there will be stored the gps class
    var time_var = new Time_utils();

    var type = null;


    function initialize(get_next_trips_list_instance) {
        BehaviorDelegate.initialize();
        view = get_next_trips_list_instance;
        gps = new Gps(me);

    }

    function update_data(data) {
        if (data.size() > 0) {
            view.new_data(data);
        }
    }

    function onSelect() {
        //depending on where is the current view there will be some ifs
        if (gps.lat != 0 && gps.lon != 0 && type != null) { //if the location has been already found
            var req = new Requests(
                gps.lat,
                gps.lon,
                type,
                time_var.time_utc(),
                me
            );
            System.println(gps.lat);
            System.println(gps.lon);
            WatchUi.requestUpdate();
            req.make_request(); //doesn't show anything
        }
        else { //once the location has been found the Gps.on_position calls onSelect as if the user
               //has pressed the select button so it refreshes the data
               //there must be added a control on what the user is actually pressing
            view.onShow();
        }



        return true;
    }
}