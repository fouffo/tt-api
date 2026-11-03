using Toybox.Lang;

class Build_menu {
    var json_data = "";
    var str_data = "";
    function initialize(str) {
        str_data = str;
    }
    function to_json() as Lang.Dictionary {
        return {"str" => str_data};
    }
}