#!/bin/bash

# https://wiki.openstreetmap.org/wiki/Key:highway

curl 'https://overpass-api.de/api/interpreter' -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0' -H 'Accept: */*' -H 'Accept-Language: en-US,en;q=0.5' --compressed -H 'Content-type: application/x-www-form-urlencoded; charset=UTF-8' --data-raw 'data=[timeout:900][maxsize:1073741824][bbox:53.916238,-1.16,54.011974,-1.0][out:json];(way[highway~"^(primary|secondary|tertiary|unclassified|residential|primary_link|secondary_link|tertiary_link|living_street|service|pedestrian|road)$"]; node(w););out skel;'
