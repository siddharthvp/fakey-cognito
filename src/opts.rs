use std::path::PathBuf;
use structopt::StructOpt;
use tokio::sync::OnceCell;

pub static OPT: OnceCell<Opt> = OnceCell::const_new();

#[derive(StructOpt, Debug)]
#[structopt(name = "fakey-cognito", about = "Fake cognito api server.")]
pub struct Opt {
    /// Read specific config path
    #[structopt(long, name = "path", parse(from_os_str))]
    pub config: Option<PathBuf>,

    /// Read specific templates directory path
    #[structopt(long, name = "path")]
    pub templates: Option<String>,
}

pub async fn init_opt() {
    OPT.get_or_init(|| async { Opt::from_args() }).await;
}

pub fn get_opt_config() -> Option<&'static PathBuf> {
    OPT.get().unwrap().config.as_ref()
}

pub fn get_opt_templates() -> Option<&'static String> {
    OPT.get().unwrap().templates.as_ref()
}
