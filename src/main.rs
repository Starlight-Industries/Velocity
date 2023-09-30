slint::include_modules!();
fn main() {
    onboard::new().unwrap().run().unwrap();
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        width: 1280px;
        height: 720px;
        background: #161219;
        title: "Velocity 0.1";
        icon: @image-url("src/assets/icons/Velocity_256x.png");
    }
}