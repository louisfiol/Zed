use zed_extension_api as zed;

struct MyExtension;

impl zed::Extension for MyExtension {
    fn new() -> Self {
        MyExtension
    }
}

// Register the extension with the Zed runner
zed::register_extension!(MyExtension);
