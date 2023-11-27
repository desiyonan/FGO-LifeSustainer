use service_locator::ServiceLocator;

pub trait Engine {
    fn set_task_config(&mut self);
}

pub static ENGIN_LOCATOR: ServiceLocator<dyn Engine + Send + Sync> = ServiceLocator::new();
