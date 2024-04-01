use service_locator::ServiceLocator;
use services::service::Service;

use super::vati_locale_service::VATI_LOCALE;

pub trait TranslationsService: Service {
    fn set_locale(&self, locale: &str);
}

pub struct TranslationsServiceProvider;

impl TranslationsServiceProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Service for TranslationsServiceProvider {
    fn init(&mut self) {
        self.set_locale(VATI_LOCALE.service().unwrap().get_current_locale().as_str())
    }
}

impl TranslationsService for TranslationsServiceProvider {
    fn set_locale(&self, locale: &str) {
        rust_i18n::set_locale(locale);
    }
}

// Provide the service.
pub static TRANSLATIONS: ServiceLocator<dyn TranslationsService + Send + Sync> =
    ServiceLocator::new();
