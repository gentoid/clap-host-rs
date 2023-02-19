use std::mem::MaybeUninit;

use clack_extensions::{
    log::{HostLog, HostLogImpl},
    params::{
        info::{ParamInfo, ParamInfoData, ParamInfoFlags},
        PluginParams,
    },
};
use clack_host::{
    prelude::{Host, HostExtensions, HostInfo, HostShared, PluginBundle, PluginInstance},
    utils::Cookie,
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
    plugin_instance: PluginInstance<PluginHost>,
    name: String,
    params: Vec<MyParamInfoData>,
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

        let plugin_params = plugin_instance
            .shared_plugin_data()
            .get_extension::<PluginParams>()
            .unwrap();

        let main_handle = plugin_instance.main_thread_plugin_data();
        let count = plugin_params.count(&main_handle);

        let mut param_index = 0;
        let mut params = vec![];
        while param_index < count {
            let mut pass_info = MaybeUninit::<ParamInfo>::uninit();
            let info: ParamInfoData<'_> = plugin_params
                .get_info(&main_handle, param_index, &mut pass_info)
                .unwrap()
                .try_into()
                .unwrap();

            params.push(info.into());

            param_index += 1;
        }

        Self {
            plugin_instance,
            name: plugin_descriptor
                .name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            params,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn params(&self) -> &Vec<MyParamInfoData> {
        &self.params
    }
}

pub struct MyParamInfoData {
    pub id: u32,
    pub flags: ParamInfoFlags,
    pub cookie: Cookie,
    pub name: String,
    pub module: String,
    pub min_value: f64,
    pub max_value: f64,
    pub default_value: f64,
}

impl From<ParamInfoData<'_>> for MyParamInfoData {
    fn from(info: ParamInfoData<'_>) -> Self {
        MyParamInfoData {
            id: info.id,
            flags: info.flags.clone(),
            cookie: info.cookie.clone(),
            name: info.name.to_owned(),
            module: info.module.to_owned(),
            min_value: info.min_value,
            max_value: info.max_value,
            default_value: info.default_value,
        }
    }
}
