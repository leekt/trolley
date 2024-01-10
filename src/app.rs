use async_trait::async_trait;
use loco_rs::{
    Result,
    prelude::Routes, controller::AppRoutes, boot::{StartMode, BootResult, create_app}, worker::Processor, app::AppContext, task::Tasks
};
use loco_rs::app::Hooks;
use crate::{controllers::eth::*, config::get_supported_networks};


pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &str) -> Result<BootResult> {
        create_app::<Self>(mode, environment).await
    }

    fn routes() -> AppRoutes {
        AppRoutes::empty().add_routes({
            let mut routes : Vec<Routes> = vec![];
            get_supported_networks().iter().for_each(|scanner| {
                match get_api_key(scanner.to_owned())
                {
                    Ok(_) => {},
                    Err(_) => {
                        print!("{} api key not found\n", scanner.chain_name);
                        return
                    }, // skip if api key not found
                }
                routes.push(scanner_routes(scanner.clone()));
            });
            routes
        })
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {}

    fn register_tasks(_tasks: &mut Tasks) {}
}

