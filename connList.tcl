#!/usr/bin/env tclsh

set output {[
        {
            "site" : "verity",
            "process" : "verity",
            "thread" : "testdb1",
            "ip" : "localhost",
            "port" : "8080",
            "insave" : "1",
            "infile" : "testdb1",
            "outsave" : "0",
            "outfile" : ""
        },
        {
            "site"    : "verity",
            "process" : "verity",
            "thread"  : "no_smat_configured",
            "ip"    : "",
            "port"    : "",
            "insave"  : "0",
            "infile"  : "",
            "outsave" : "0",
            "outfile" : ""
        },
        {
       "site"    : "verity",
        "process" : "verity",
        "thread"  : "both_smat_configured",
        "ip"    : "",
        "port"    : "",
        "insave"  : "1",
        "infile"  : "testdb1",
        "outsave" : "1",
        "outfile" : "testdb1"
}]}

puts -nonewline $output
flush stdout