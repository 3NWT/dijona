use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let dijona = Application::builder().application_id("dijona").build();
    dijona.connect_activate(buildui);
    dijona.run();
}

fn buildui(dijona: &Application){
    let pencere = ApplicationWindow::builder().title("dijona").application(dijona).build();
    pencere.show();
}
