trait TargetPlugin {
}

trait ProviderPlugin {
}

trait MiddlewarePlugin {
}

pub enum Plugin {
    TargetPlugin(TargetPlugin),
    ProviderPlugin(ProviderPlugin),
    MiddlewarePlugin(MiddlewarePlugin),
}

fn load_plugin(name: &str) -> Plugin
