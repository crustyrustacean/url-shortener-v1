# URL Shortener Project Plan

## MVP

- [x] overall app design
  - [URL Shortener](https://docs.shuttle.dev/templates/tutorials/url-shortener)
- [x] create basic project template from the Shuttle Axum starter
- [x] refactor into `bin` and `lib` crates
- [x] add health_check endpoint
- [x] deploy to Shuttle to confirm that the service runs and health_check endpoint responds in the correct fashion
- [x] add boilerplate for integration tests
- [x] add test for health_check endpoint
- [x] add Postgres database and migrations
- [x] re-factor to add database state to `Application::build()` function
- [x] add error handling boilerplate (ended up keeping it simple like the existing URL shortener example)
- [x] add the endpoints
  - [x] shorten endpoint
    - [x] associated integration test
  - [x] redirect endpoint
    - [x] associated integration test
- [x] instrumentation
- [x] final deployment
- [x] custom domain name and add https
- [ ] monitoring/status page with [BetterStack](https://www.shuttle.dev/blog/2024/12/20/set-up-status-page-betterstack)

## Future Plans

- [ ] tera templates for web ui
- [ ] user logins
  - [ ] authentication
