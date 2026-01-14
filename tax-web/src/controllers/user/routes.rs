use actix_web::web;

use crate::controllers::user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::create::handler);
    cfg.service(user::delete::handler);
    cfg.service(user::detail::handler);
    cfg.service(user::list::handler);
    cfg.service(user::update::handler);
}
