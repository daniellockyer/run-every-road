#!/bin/bash

curl 'https://overpass-api.de/api/interpreter' -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:84.0) Gecko/20100101 Firefox/84.0' -H 'Accept: */*' -H 'Accept-Language: en-US,en;q=0.5' --compressed -H 'Content-type: application/x-www-form-urlencoded; charset=UTF-8' --data-raw 'data=%5Btimeout%3A900%5D%5Bmaxsize%3A1073741824%5D%5Bbbox%3A53.916238%2C-1.16%2C54.011974%2C-1.0%5D%5Bout%3Ajson%5D%3B%0A(way%5Bhighway%5D%3B%20node(w)%3B)%3B%0Aout%20skel%3B'
