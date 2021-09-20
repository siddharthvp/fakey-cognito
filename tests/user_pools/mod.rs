use fakey_cognito::*;

mod admin_add_user_to_group_test;

pub async fn setup() {
    opts::init_opt().await;
    let templates_opt = opts::get_opt_templates();
    tokio::join!(
        user_pools::init_config(opts::get_opt_config()),
        templates::init_template(templates_opt.map(String::as_str)),
        templates::init_default_template()
    );
}