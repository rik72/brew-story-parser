use vati::core::{
    translations_service::{TranslationsServiceProvider, TRANSLATIONS},
    vati_locale_service::{VatiLocaleServiceProvider, VATI_LOCALE},
};

use crate::parsing::parsers::parser_service::{ParserServiceProvider, PARSER};

pub struct ServiceRegistry;

impl ServiceRegistry {
    pub fn init_services() {
        // set providers
        VATI_LOCALE.provide(Box::new(VatiLocaleServiceProvider::new()));
        TRANSLATIONS.provide(Box::new(TranslationsServiceProvider::new()));
        PARSER.provide(Box::new(ParserServiceProvider::new()));

        // init services
        VATI_LOCALE.service_mut().unwrap().init();
        TRANSLATIONS.service_mut().unwrap().init();
        PARSER.service_mut().unwrap().init();
    }
}
