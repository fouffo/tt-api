using Toybox.WatchUi;

class Menus extends WatchUi.Menu2InputDelegate {

    var calling_instance;
    var menu_type;

    function initialize(instance, type) {
        calling_instance = instance;
        menu_type = type;
        Menu2InputDelegate.initialize();
    }
    
  	function onSelect(item) {
        var id = item.getId();
        WatchUi.popView(WatchUi.SLIDE_LEFT);


        if (menu_type.equals("type")) {
            if(id.equals("U") || id.equals("E")) {
                System.println(id);
                calling_instance.type = id;
            }
            else {
                System.println("something went wrong getting the id");
            }
        }
        else if (menu_type.equals("stops")) {
            System.println(id);
            calling_instance.new_trips(id);
        }
        else if (menu_type.equals("trips")) {
            calling_instance.trips_menu[:selected] = id;
            calling_instance.onShow();
        }
	}
}	