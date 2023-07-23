use fluent_bundle::{FluentBundle, FluentValue, FluentResource, FluentArgs};
// Used to provide a locale for the bundle.
use unic_langid::langid;

pub fn init_fluent() {
    
    fluent_messages! {
        typeck => "../locales/en-US/typeck.ftl",
    }

}