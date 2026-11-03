using Toybox.Application;

class Traquantopassa extends Application.AppBase {

    function initialize() {
        AppBase.initialize();

    }

    function getInitialView() {
        var view = new View_handler();
        var buttons = new Buttons(view);
        view.get_buttons(buttons);
        return [
            view,
            buttons
        ];
    }
}
