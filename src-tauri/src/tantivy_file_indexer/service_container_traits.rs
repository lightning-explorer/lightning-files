use std::sync::Arc;

use super::service_container::AppServiceContainer;

pub trait FromAppService{
    fn new_from_service(service: &AppServiceContainer) -> Arc<Self>;
}