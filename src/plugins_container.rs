use clack_host::prelude::HostInfo;

use crate::plugin_host::PluginHost;

pub struct PluginsContainer {
    host_info: HostInfo,
    plugins: Vec<PluginHost>,
}

impl PluginsContainer {
    pub fn init() -> Self {
        Self {
            host_info: HostInfo::new(
                "Plugins loader",
                "no company",
                "https://github.com/gentoid/clap-host-rs",
                "0.1.0",
            )
            .unwrap(),
            plugins: vec![],
        }
    }

    pub fn load(&mut self, path: &str) {
        let plugin_host = PluginHost::new(&self.host_info, path);
        self.plugins.push(plugin_host);
    }

    pub fn unload(&mut self, index: usize) {
        if index >= self.plugins.len() {
            return;
        }

        self.plugins.remove(index);
    }

    pub fn plugins(&self) -> &Vec<PluginHost> {
        &self.plugins
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}
