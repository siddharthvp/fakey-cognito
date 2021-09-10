use std::path::PathBuf;
use structopt::StructOpt;
use tokio::sync::OnceCell;

pub static OPT: OnceCell<Opt> = OnceCell::const_new();

#[derive(StructOpt, Debug)]
#[structopt(name = "fakey-cognito", about = "Fake cognito api server.")]
pub struct Opt {
    /// Read specific config path
    #[structopt(short, long, name = "path", parse(from_os_str))]
    pub config: Option<PathBuf>,
}

pub async fn init_opt() {
    OPT.get_or_init(|| async { Opt::from_args() }).await;
}

pub fn get_opt_config() -> Option<&'static PathBuf> {
    OPT.get().unwrap().config.as_ref()
}