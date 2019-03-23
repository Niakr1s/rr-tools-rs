use glib::signal::SignalHandlerId;
use gtk::{Builder, Button, ButtonExt, Spinner, SpinnerExt, WidgetExt};

#[derive(Clone)]
pub struct SpinnerButton {
    pub button: Button,
    pub spinner: Spinner,
}

impl SpinnerButton {
    pub fn new(
        builder: &Builder,
        button_glade_str: &str,
        spinner_glade_str: &str,
    ) -> SpinnerButton {
        let button: Button = builder
            .get_object(button_glade_str)
            .unwrap_or_else(|| panic!("no {} in glade file", button_glade_str));
        let spinner: Spinner = builder
            .get_object(spinner_glade_str)
            .unwrap_or_else(|| panic!("no {} in glade file", spinner_glade_str));
        SpinnerButton { button, spinner }
    }

    pub fn start(&self) {
        self.button.set_sensitive(false);
        self.spinner.start();
    }

    pub fn stop(&self) {
        self.button.set_sensitive(true);
        self.spinner.stop();
    }

    pub fn connect_clicked<F: Fn(&Button) + 'static>(&self, f: F) -> SignalHandlerId {
        self.button.connect_clicked(f)
    }
}
