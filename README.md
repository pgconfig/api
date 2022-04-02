# PGConfig API

[![Coverage Status](https://coveralls.io/repos/github/pgconfig/api/badge.svg?branch=master)](https://coveralls.io/github/pgconfig/api?branch=master) [![Go Report Card](https://goreportcard.com/badge/github.com/pgconfig/api)](https://goreportcard.com/report/github.com/pgconfig/api) ![GoReleaser](https://github.com/pgconfig/api/workflows/goreleaser/badge.svg)


> This project is a VERY BIG WORK IN PROGRESS.

PGConfig.org API v2.

## Objectives of this repo

1. host the v2 api of pgconfig.org (not started):

    * [x] Create new API using fiber
        * [x] Route compare
        * [x] other routes
    * [x] Update Release with and build the docker images
    * [ ] Add JSON format option
    * [ ] Remove "blocked" params in the SGPostgresConfig format


1. build and release the pgconfigctl (wip):

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
    * [ ] Review all metrics



## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpgconfig%2Fapi.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpgconfig%2Fapi?ref=badge_large)