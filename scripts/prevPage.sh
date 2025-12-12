#!/bin/sh
# Turn page backward on Kindle by injecting touch event

printf '%s' 'JLBpZ9XXBQABAEoBAQAAACSwaWfV1wUAAwA5AAAAAAAksGln1dcFAAMANQAoAAAAJLBpZ9XXBQAD
ADYAHwMAACSwaWfV1wUAAwAwADQAAAAksGln1dcFAAAAAAAAAAAAJLBpZ5uJBwABAEoBAAAAACSw
aWebiQcAAwA5AP////8ksGlnm4kHAAAAAAAAAAAA' | base64 -d > /dev/input/event1
