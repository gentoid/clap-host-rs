use clack_host::prelude::{HostInfo, PluginAudioConfiguration};

use crate::plugin_host::PluginHost;

pub struct PluginsContainer {
    host_info: HostInfo,
    pub plugins: Vec<PluginHost>,
    audio_configuration: PluginAudioConfiguration,
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
            audio_configuration: PluginAudioConfiguration {
                sample_rate: 48_000.0,
                frames_count_range: 32..=32,
            },
        }
    }

    pub fn load(&mut self, path: &str) {
        let mut plugin_host = PluginHost::new(&self.host_info, path);
        let audio_configuration = PluginAudioConfiguration {
            sample_rate: self.audio_configuration.sample_rate,
            frames_count_range: self.audio_configuration.frames_count_range.clone(),
        };
        plugin_host.activate(audio_configuration);
        self.plugins.push(plugin_host);
    }

    pub fn unload(&mut self, index: usize) {
        if index >= self.plugins.len() {
            return;
        }

        let mut plugin_host = self.plugins.remove(index);
        plugin_host.deactivate();
    }

    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}
