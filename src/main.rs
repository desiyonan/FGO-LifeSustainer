
// use lib::core::engine::ENGIN_LOCATOR;

fn main() {
    // The service is not provided yet.
    // assert!(AUDIO_SERVICE_LOCATOR.service().is_err());
    lib::core::p();
    println!("{}", lib::core::engine::ENGIN_LOCATOR.service().is_err());
}
