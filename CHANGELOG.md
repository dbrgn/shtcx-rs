# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## 0.10.0 - 2020-10-23

- [add] Provide non-blocking API for measurements (#16)
- [add] Type aliases for the sensors (#20)
- [add] Expose `MeasurementDuration` trait (#23)
- [add] Add shortcut function `max_measurement_duration` (#23)
- [fix] Remove `MeasurementDuration` trait bound from some impls (#24)

Since a new async API has been added, we have decided to wait with
stabilization until we've gathered some more practical experience with this
API.


## 0.9.0 - 2020-02-11

- [add] Support for SHTC1 and SHTW2
- [add] Hardcoded I²C addresses 
- [fix] Fix underflow in temperature conversion (#9)
- [change] Remove WriteRead trait bound (#13)

This release should be feature-complete. Unless some bugs or API deficiencies
are found over the next weeks, a 1.0 release should follow later this year.


## 0.1.0 - 2020-01-25

This is the initial release to crates.io of the feature-complete driver (with
support for SHTC3). There may be some API changes in the future, in case I
decide that something can be further improved. Furthermore, support for SHTC1
and maybe other devices of this series will be added. All changes will be
documented in this CHANGELOG.
