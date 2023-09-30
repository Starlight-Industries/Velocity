fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        title: "Velocity 0.1";
        icon: @image-url("src/assets/icons/Velocity_256x.png");
    }
}