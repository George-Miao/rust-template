# Rust Template

A simple template for Rust services.

## Features

- Load config from env and other files into struct with [`figment`] and [`serde`]
- Tokio runtime
- Dynamic error handling with [`color_eyre`]
- Graceful shutdown with [`tokio_graceful_shutdown`]

[`figment`]: https://crates.io/crates/figment
[`serde`]: https://crates.io/crates/serde
[`color_eyre`]: https://crates.io/crates/color_eyre
[`tokio_graceful_shutdown`]: https://crates.io/crates/tokio_graceful_shutdown

## Usage

### Derive from the template

Use degit:

```
degit https://github.com/George-Miao/rust-template my_app
cd my_app
```

### Start editing code

If you want, change config prefix. This is used when loading environmental variables. For example, `some_path` in `Config` will be loaded from `APP_SOME_PATH`.

```rust
/// Change this to your own prefix.
const CONFIG_PREFIX: &str = "APP_";
```

Add config to `src/config.rs`. Notice that default value can be added via `#[serde(default = "...")]` where `...` is a module path to default function. In this case, a specific mod `default` is being used to place default functions.

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // Add your config fields here
    #[serde(default = "default::some_path")]
    pub some_path: PathBuf,
}
```

Add test:

```rust
#[test]
fn test_config() {
    figment::Jail::expect_with(|jail| {
        jail.set_env(format!("{}SOME_PATH", CONFIG_PREFIX), "/some/path");
        let conf = Config::from_env().unwrap();
        assert_eq!(conf.some_path, PathBuf::from("/some/path"));
        Ok(())
    })
}
```

Now add services. Services are simple functions that takes a [`SubsystemHandle`] which is used for graceful shutdown, or a struct with all dependencies needed with a startup function that takes a [`SubsystemHandle`]. An example is provided:

```rust
async fn app(sys: SubsystemHandle) -> Result<()> {
    // Load the config from cache, which is cheap
    let _config = Config::get();

    let mut count = 3;

    while count > 0 {
        // Do some job
        sleep(Duration::from_secs(1)).await;
        info!("Tick!");
        count -= 1;
    }

    info!("Gonna go!");

    sys.request_shutdown();

    Ok(())
}
```

[`SubsystemHandle`]: https://docs.rs/tokio-graceful-shutdown/0.9.0/tokio_graceful_shutdown/struct.SubsystemHandle.html

### Dotenv

Since dotenv is enabled, if you have local debugging variables, create an `.env` file. This will be loaded during startup. However, these dotenv files with critical information should not be committed to source control.

```Shell
// .env
APP_SOME_PATH=/tmp/path
```

### Run

```bash
➜ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/rust-template`
2022-06-02T02:09:50.694254Z  INFO rust_template: Config loaded! config=Config { some_path: "/tmp/path" }
2022-06-02T02:09:51.697428Z  INFO rust_template: Tick!
2022-06-02T02:09:52.700287Z  INFO rust_template: Tick!
2022-06-02T02:09:53.702866Z  INFO rust_template: Tick!
2022-06-02T02:09:53.703097Z  INFO rust_template: Gonna go!
2022-06-02T02:09:53.703214Z  INFO tokio_graceful_shutdown::shutdown_token: Initiating shutdown ...
```
