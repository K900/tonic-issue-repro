**To run**: `docker build . -t repro && docker run -it --rm repro`

Expected result: first request fails, second and third requests succeed.

Observed result: first and second requests fail; third request succeeds.

Server code lightly adapted from [`tonic` examples](https://github.com/hyperium/tonic/blob/c7fd9d4586d7e645ff88be3a76769a435385516c/examples/src/helloworld/server.rs).

Proxy code heavily adapted from an internal project.
