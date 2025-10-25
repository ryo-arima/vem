use crate::ent::request::environment::ENVIRONMENT as RequestEnvironment;
use crate::ent::response::environment::ENVIRONMENT as ResponseEnvironment;

pub trait EnvironmentUsecase {
    fn create(&self, request: RequestEnvironment) -> ResponseEnvironment;
}