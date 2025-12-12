#!/bin/sh
# Turn page forward on Kindle by injecting touch event

printf '%s' 'HrBpZ2m9DgABAEoBAQAAAB6waWdpvQ4AAwA5AAAAAAAesGlnab0OAAMANQABBAAAHrBpZ2m9DgAD
ADYAFwMAAB6waWdpvQ4AAwAwACkAAAAesGlnab0OAAAAAAAAAAAAH7BpZ6zIAAABAEoBAAAAAB+w
aWesyAAAAwA5AP////8fsGlnrMgAAAAAAAAAAAAA' | base64 -d > /dev/input/event1
