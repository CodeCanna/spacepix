# Spacepix

## Welcome!
Hello and welcome to the Spacepix repository.  Spacepix is educational software that allows the user to explore the NASA API and it's data from a desktop application written in Rust and built on the [egui](https://github.com/emilk/egui) crate.

## Building and Running
Because Spacepix is written in Rust, it should run roughly the same on Linux, Mac, and Windows.

### To Build Spacepix
Run `gh repo clone CodeCanna/spacepix` or `git clone https://github.com/CodeCanna/spacepix.git`

Spacepix uses `Make` and `cargo` to build and run:
* `make all` Builds optimized for both Linux and Windows x64 targets (MAC to come) uses `--release`
* `make linux` Builds for both Linux and Windows optmized uses `--release`
* `make linuxdev` Builds for Linux x64 unoptmized
* `make windows` Builds for Windows x64 optimized uses `--release`
* `make windowsdev` Builds for Windows x64 unoptimized

### To Run Spacepix

#### Running with Cargo
If you just want to run Spacepix you can use `cargo run` or `cargo run --release` (optimizations)

#### Running with Make
You can run Spacepix with Make using `make run` or `make rrun` for optimizations using `--release`

# "Activating" Spacepix
### Using NASA's demo API Key
By default Spacepix uses the `DEMO_KEY` to retrieve data from the NASA API.  This key should be sufficient if you are the only person on your network using Spacepix.  However Spacepix is supposed to be educational software to be used by many, if your organization wants to use Spacepix on a broader scale it would be a good idea to get an [API key](https://api.nasa.gov/) from the NASA website.

### DEMO_KEY Rate Limits

In documentation examples, the special DEMO_KEY api key is used. This API key can be used for initially exploring APIs prior to signing up, but it has much lower rate limits, so you’re encouraged to signup for your own API key if you plan to use the API (signup is quick and easy). The rate limits for the DEMO_KEY are:

* Hourly Limit: 30 requests per IP address per hour
* Daily Limit: 50 requests per IP address per day

### Obtaining an API key
Spacepix uses NASA's API to get it's data.  NASA requires that you [register here](https://api.nasa.gov/) to get a free API key.  This will allow you to use Spacepix with no limitations.

Once you get your API key, you can enter it in Spacepix by going to Settings->Set API Key.  For now you might have to restart Spacepix if it still doesn't work but it should work after that.  There's an issue already open for this problem.


