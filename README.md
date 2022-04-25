# PGConfig API

[![Coverage Status](https://coveralls.io/repos/github/pgconfig/api/badge.svg?branch=main)](https://coveralls.io/github/pgconfig/api?branch=main) [![Go Report Card](https://goreportcard.com/badge/github.com/pgconfig/api)](https://goreportcard.com/report/github.com/pgconfig/api) ![GoReleaser](https://github.com/pgconfig/api/workflows/goreleaser/badge.svg)


> This project is a VERY BIG WORK IN PROGRESS.

PGConfig.org API v2.

## Objectives of this repo

1. host the v2 api of pgconfig.org:

    * [x] Create new API using fiber
        * [x] Route compare
        * [x] other routes
    * [x] Update Release with and build the docker images
    * [x] Add JSON format option
    * [x] Remove "blocked" params in the SGPostgresConfig format


1. build and release the pgconfigctl:

    * [x] Migrate/Review all Categories:
        * [x] Memory Configuration
        * [x] Checkpoint Related Configuration
        * [x] Network Related Configuration
        * [x] Storage Configuration
        * [x] Worker Processes Configuration
    * [x] Implement compute filters:
        * [x] Arch
        * [x] OS
        * [x] Storage
        * [x] CPU Count
        * [x] Total RAM
        * [x] Profile
        * [x] Version
    * [x] Review all metrics
    * [x] move all inputs to the `pkg/input` pkg
        * [x] update pgconfigctl to use the new inputs
        * [x] update api to use the new inputs

1. review metrics
    * [ ] based in stackgres list: https://gitlab.com/ongresinc/stackgres/-/issues/486#note_360442486
    * [ ] based in the old api: 
         * [ ] https://github.com/sebastianwebber/pgconfig-api/blob/master/advisors/tuning.py
         * [ ] https://github.com/sebastianwebber/pgconfig-api/blob/master/advisors/config.py



## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpgconfig%2Fapi.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpgconfig%2Fapi?ref=badge_large)
