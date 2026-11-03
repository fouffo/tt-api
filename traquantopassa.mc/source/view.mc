using Toybox.WatchUi;

class View_handler extends WatchUi.View {

    var text = null;

    var routes_type_menu = {
        :show => true,
        :what => "type",
        :menu => new WatchUi.Menu2({:title => "Route type"})
            
    };
    var stops_menu = {
        :show => false,
        :what => "stops",
        :selected => -1,
        :menu => new WatchUi.Menu2({:title => "Select a stop"})
    };
    var trips_menu = {
        :show => false,
        :what => "trips",
        :selected => -1,
        :menu => new WatchUi.Menu2({:title => "Next trips"})
    };

    
    var full_data = {};

    var buttons_instance;
    var opening;

    function initialize() {
        View.initialize();
        opening = true;
    }
    function get_buttons(buttons) {
        buttons_instance = buttons;
    }

    function new_data(arr) {   
        full_data = arr;
        me.build_stops_menu();
        stops_menu[:show] = true;
        me.onShow();
    }

    function new_trips(id) {
        stops_menu[:selected] = id;
        me.build_trips_menu();
        trips_menu[:show] = true;
        me.onShow();
    }

    function build_stops_menu() {
        for (var item_num = 0; item_num < full_data.size(); item_num++) {
            stops_menu[:menu].addItem(new WatchUi.MenuItem(
                full_data[item_num]["stopName"].toString(),
                "Distance: " + full_data[item_num]["distance"].toString() + "m",
                item_num,
                {}
            ));
            System.println(item_num);
        }
    }

    function build_trips_menu() {
        var next_trips = full_data[stops_menu[:selected]]["nextTrips"];
        System.println(next_trips.toString());
        for(var trip_num = 0; trip_num < next_trips.size(); trip_num++) {
            trips_menu[:menu].addItem(new WatchUi.MenuItem(
                next_trips[trip_num]["routeShortName"],
                next_trips[trip_num]["routeShortName"],
                trip_num,
                {}
            ));
        }
    }

    function onLayout(dc) {
        routes_type_menu[:menu].addItem(new WatchUi.MenuItem("Urbano", "", "U", {}));
        routes_type_menu[:menu].addItem(new WatchUi.MenuItem("Extraurbano", "", "E", {}));
    }

    function onShow() as Void {
        var bottom = false;

        if (trips_menu[:selected] >= 0) { //there is something to show
            bottom = true;
            text = new Text({
                :text => full_data[stops_menu[:selected]]["nextTrips"][trips_menu[:selected]]["routeLongName"],
                :color => Graphics.COLOR_WHITE,
                :font => Graphics.FONT_SMALL,
                :locX => WatchUi.LAYOUT_HALIGN_CENTER,
                :locY => WatchUi.LAYOUT_VALIGN_CENTER
            });
        }
        else {
            if (trips_menu[:selected] == -1) {
                bottom = false;
                //don't do anythong, it means it is the initialization
            }
            else if (trips_menu[:selected] == -2) {
                bottom = true;
                text = new Text({
                    :text=>"No more trips for today",
                    :color=>Graphics.COLOR_WHITE,
                    :font=>Graphics.FONT_SMALL,
                    :locX =>WatchUi.LAYOUT_HALIGN_CENTER,
                    :locY=>WatchUi.LAYOUT_VALIGN_CENTER
                });
            }
        }

        if (routes_type_menu[:show] == true) {
            routes_type_menu[:show] = false;
            WatchUi.pushView(
                routes_type_menu[:menu],
                new Menus(buttons_instance, routes_type_menu[:what]),
                WatchUi.SLIDE_IMMEDIATE
            );
        }
        else {
            if (stops_menu[:show] == true) {
                stops_menu[:show] = false;
                WatchUi.pushView(
                    stops_menu[:menu],
                    new Menus(me, stops_menu[:what]),
                    WatchUi.SLIDE_IMMEDIATE
                );
            }
            else {
                if (trips_menu[:show] == true) {
                    trips_menu[:show] = false;
                    WatchUi.pushView(
                        trips_menu[:menu],
                        new Menus(me, trips_menu[:what]),
                        WatchUi.SLIDE_IMMEDIATE
                    );
                }
                else {
                    if (bottom == false) {
                        text = new Text({
                            :text=>"Waiting for position",
                            :color=>Graphics.COLOR_WHITE,
                            :font=>Graphics.FONT_SMALL,
                            :locX =>WatchUi.LAYOUT_HALIGN_CENTER,
                            :locY=>WatchUi.LAYOUT_VALIGN_CENTER
                        });
                    }
                    
                }
               
            }
        }
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

