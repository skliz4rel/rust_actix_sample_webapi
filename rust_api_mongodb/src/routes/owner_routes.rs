use actix_web::web;

//super can not be used here to access the handlers module because it is not a submodule of the current module. Instead, you can use the full path to access the create_owner function from the owner_handler module.
//The right import statement would be crate::handlers::owner_handler::create_owner;
//Because crate is the root of the current crate, and handlers is a module within that crate. Then, owner_handler is a submodule of handlers, and create_owner is a function within owner_handler.
use crate::handlers::owner_handler::create_owner;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1").service(create_owner));
}
