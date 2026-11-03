/*

TODO:
- check why i have to push two times select to enter
  the closest stops if the position is available
  from the beginning
- add comments
- rewirte the code in a more readable way
- check the flow of the calls
- check memory usage

*/

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
