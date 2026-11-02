using Toybox.Position;

class Gps {
    var buttons_instance; //needed to call on select after it has found the position
    var current_location;
    var lat = 0;
    var lon = 0;
    function initialize(buttons) {
        buttons_instance = buttons;
        Position.enableLocationEvents(Position.LOCATION_ONE_SHOT, method(:on_position));
        System.println(("First location request"));
    }
    function update_location() { //if you want another position different from the cached one
        Position.enableLocationEvents(Position.LOCATION_ONE_SHOT, method(:on_position));
        System.println(("Update location request"));
    }
    function on_position(pos_info as Position.Info) as Void {
        System.println(("Position received"));
        current_location = pos_info.position.toDegrees();
        lat = current_location[0];
        lon = current_location[1];
        buttons_instance.onSelect(); //once it founds the location it refreshes all the date
    }
}