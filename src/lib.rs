pub mod api; // everything releated to api server
pub mod datamodels; // project datamodels
pub mod service; // entire projects business logic
pub mod repository; // all database related things
pub mod configuration; //
pub mod queue; // rabbitmq
pub mod error; // definition of error that will be used in project
pub mod worker; // processing thing related to worker
pub mod template_engine;