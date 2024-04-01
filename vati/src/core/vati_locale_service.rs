use service_locator::ServiceLocator;
use services::service::Service;

use crate::parse_locale::parse_locale::ParseLocale;

use super::{translations_service::TRANSLATIONS, vati_locale::VatiLocale};

pub trait VatiLocaleService: Service {
    fn get_current(&self) -> &VatiLocale;
    fn get_current_locale(&self) -> String;
    fn get_current_parse_locale(&self) -> ParseLocale;
    fn set_current(&mut self, vati_locale: VatiLocale);
}

pub struct VatiLocaleServiceProvider {
    vati_locale: VatiLocale,
}

impl VatiLocaleServiceProvider {
    pub fn get_default() -> VatiLocale {
        VatiLocale::English
    }

    pub fn new() -> Self {
        Self {
            vati_locale: Self::get_default(),
        }
    }
}

impl Service for VatiLocaleServiceProvider {
    fn init(&mut self) {
        // NOP
    }
}

impl VatiLocaleService for VatiLocaleServiceProvider {
    fn get_current(&self) -> &VatiLocale {
        &self.vati_locale
    }

    fn get_current_locale(&self) -> String {
        self.vati_locale.locale()
    }

    fn get_current_parse_locale(&self) -> ParseLocale {
        self.vati_locale.parse_locale()
    }

    fn set_current(&mut self, vati_locale: VatiLocale) {
        self.vati_locale = vati_locale;
        TRANSLATIONS
            .service()
            .unwrap()
            .set_locale(self.vati_locale.locale().as_str());
    }
}

// Provide the service.
pub static VATI_LOCALE: ServiceLocator<dyn VatiLocaleService + Send + Sync> = ServiceLocator::new();
