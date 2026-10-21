import Toybox.Lang;
import Toybox.WatchUi;

class traquantopassa_mcDelegate extends WatchUi.BehaviorDelegate {

    function initialize() {
        BehaviorDelegate.initialize();
    }

    function onMenu() as Boolean {
        WatchUi.pushView(new Rez.Menus.MainMenu(), new traquantopassa_mcMenuDelegate(), WatchUi.SLIDE_UP);
        return true;
    }

}