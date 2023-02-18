use std::mem::MaybeUninit;

use clack_extensions::{
    log::{HostLog, HostLogImpl},
    params::{
        info::{ParamInfo, ParamInfoData, ParamInfoFlags},
        PluginParams,
    },
};
use clack_host::prelude::{
    Host, HostExtensions, HostInfo, HostShared, PluginBundle, PluginInstance,
};

#[derive(Default)]
pub struct PluginHostShared;

impl<'a> HostShared<'a> for PluginHostShared {
    fn request_restart(&self) {
        todo!()
    }

    fn request_process(&self) {
        todo!()
    }

    fn request_callback(&self) {
        todo!()
    }
}

impl<'a> HostLogImpl for PluginHostShared {
    fn log(&self, severity: clack_extensions::log::LogSeverity, message: &str) {
        println!("[{severity}] [Plugin] {message}")
    }
}

pub struct PluginHost {
    instance: PluginInstance<PluginHost>,
    name: String,
}

impl<'a> Host<'a> for PluginHost {
    type Shared = PluginHostShared;

    type MainThread = ();

    type AudioProcessor = ();

    fn declare_extensions(builder: &mut HostExtensions<'_, Self>, _shared: &Self::Shared) {
        builder.register::<HostLog>();
    }
}

impl PluginHost {
    pub fn new(host_info: &HostInfo, path: &str) -> Self {
        let bundle = PluginBundle::load(path).unwrap();
        let plugin_factory = bundle.get_plugin_factory().unwrap();
        let plugin_descriptor = plugin_factory.plugin_descriptor(0).unwrap();
        let plugin_instance = PluginInstance::<PluginHost>::new(
            |_| PluginHostShared,
            |_| (),
            &bundle,
            plugin_descriptor.id().unwrap(),
            &host_info,
        )
        .unwrap();

        Self {
            instance: plugin_instance,
            name: plugin_descriptor
                .name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
