use gtk::{Builder, Button, Spinner};

pub struct ButtonWithSpinner {
    pub button: Button,
    pub spinner: Spinner,
}

impl ButtonWithSpinner {
    fn new(
        builder: &Builder,
        button_glade_str: &str,
        spinner_glade_str: &str,
    ) -> ButtonWithSpinner {
        let button: Button = builder
            .get_object(button_glade_str)
            .expect("bad glade file");
        let spinner: Spinner = builder
            .get_object(spinner_glade_str)
            .expect("bad glade file");
        ButtonWithSpinner { button, spinner }
    }
}
