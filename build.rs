fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res\\windows\\space-acres.ico");
        res.compile().unwrap();
    }

    glib_build_tools::compile_resources::<&str>(
        &["res"],
        "res/resources.gresource.xml",
        "prophesy.gresource",
    );
}
