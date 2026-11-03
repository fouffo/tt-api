using Toybox.System;
using Toybox.Time;

class Time_utils {
    var time;
    function initialize() {}
    function time_utc(){
        time = Time.Gregorian.utcInfo(Time.now(), Time.FORMAT_SHORT);
        var t = Lang.format(
            "$1$-$2$-$3$T$4$:$5$:$6$Z",
            [
                time.year.format("%04d"),
                time.month.format("%02d"),
                time.day.format("%02d"),
                time.hour.format("%02d"),
                time.min.format("%02d"),
                time.sec.format("%02d")
            ]
        );
        System.println(t);
        return t;
    }

}