using Toybox.Lang;

class Json {
    var json_data = "";
    var str_data = "";
    function initialize(str) {
        str_data = str;
    }
    function to_json() as Lang.Dictionary {
        return {"str" => str_data};
    }
}