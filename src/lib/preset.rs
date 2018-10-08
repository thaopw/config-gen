use actix_web::http::Method;
use actix_web::App;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use config::ProgramConfig;
use options::ProxyOpts;
use presets::m2::preset_m2::ModuleData;
use presets::m2::requirejs_config::RequireJsClientConfig;
use rewrites::RewriteContext;
use std::sync::Arc;
use std::sync::Mutex;

pub trait Preset<T> {
    fn enhance(&self, app: App<T>) -> App<T>;
    fn rewrites(&self) -> RewriteFns {
        vec![]
    }
    fn add_before_middleware(&self, app: App<T>) -> App<T> {
        app
    }
    fn add_after_middleware(&self, app: App<T>) -> App<T> {
        app
    }
}

///
/// The following are just aliases
///
pub type RewriteFns = Vec<fn(&str, &RewriteContext) -> String>;
pub type ResourceDef<'a> = (&'a str, Method, fn(&HttpRequest<AppState>) -> HttpResponse);

pub struct AppState {
    pub program_config: ProgramConfig,
    pub opts: ProxyOpts,
    pub rewrites: RewriteFns,
    pub module_items: Mutex<Vec<ModuleData>>,
    pub require_client_config: Arc<Mutex<RequireJsClientConfig>>,
}
