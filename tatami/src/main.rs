use sentry::ClientInitGuard;

fn main() {
    // populate environment variables from `.env` file without overriding
    dotenvy::dotenv().ok();

    let environment = std::env::var("ENVIRONMENT").unwrap_or_default();
    if environment.is_empty() {
        panic!("ENVIRONMENT must be set; e.g. `development` or `production`");
    }

    let mut _guard: Option<ClientInitGuard> = None;
    let sentry_dsn = std::env::var("SENTRY_DSN").unwrap_or_default();
    if !sentry_dsn.is_empty() {
        // how frequently to record transactions for performance monitoring;
        // * 1.0 if working on performance issues in development
        // * 0.0 otherwise in development to avoid using quota
        // * 0.1 or something low in production
        let t_sample_rate = std::env::var("SENTRY_T_SAMPLE_RATE")
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or_default();

        let options = sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(environment.into()),
            traces_sample_rate: t_sample_rate,
            ..Default::default()
        };

        _guard = Some(sentry::init((sentry_dsn, options)));
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(tatami::run_server());
}
