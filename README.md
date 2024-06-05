# Async spawn and non-Send

Playing with Tokio spawn to:

* How the runtime distribute tasks across threads.
* Giving an example of non-Send future (due to non-Send data across await points) and how this is incompatible with Tokio spawn.
