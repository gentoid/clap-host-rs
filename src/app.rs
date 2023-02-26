use cpal::{
    traits::{DeviceTrait, HostTrait},
    SupportedStreamConfigRange,
};
use egui::Slider;

use crate::{audio_io::AudioIO, plugins_container::PluginsContainer};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    plugins_to_remove: Vec<usize>,
    #[serde(skip)]
    plugins_container: PluginsContainer,
    #[serde(skip)]
    selected_audio_device: String,
    #[serde(skip)]
    selected_input_config: String,
    #[serde(skip)]
    selected_output_config: String,
    #[serde(skip)]
    audio_io: AudioIO,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            plugins_to_remove: vec![],
            plugins_container: PluginsContainer::init(),
            selected_audio_device: String::new(),
            selected_input_config: String::new(),
            selected_output_config: String::new(),
            audio_io: AudioIO::init(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self { label, value } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::right("settings").show(ctx, |ui| {
            let host = cpal::default_host();

            ui.vertical(|ui| {
                egui::ComboBox::from_label("Devices")
                    .width(350.0)
                    .selected_text(&self.selected_audio_device)
                    .show_ui(ui, |ui| {
                        for device in host.devices().unwrap() {
                            let device_name = device.name().unwrap();
                            ui.selectable_value(
                                &mut self.selected_audio_device,
                                device_name.clone(),
                                device_name,
                            );
                        }
                    });

                egui::ComboBox::from_label("Inputs")
                    .width(350.0)
                    .selected_text(&self.selected_input_config)
                    .show_ui(ui, |ui| {
                        if let Some(device) = host
                            .devices()
                            .unwrap()
                            .find(|device| device.name().unwrap() == self.selected_audio_device)
                        {
                            for input_config in device.supported_input_configs().unwrap() {
                                let formatted = format_stream_config(&input_config);
                                ui.selectable_value(
                                    &mut self.selected_input_config,
                                    formatted.clone(),
                                    formatted,
                                );
                            }
                        }
                    });

                egui::ComboBox::from_label("Outputs")
                    .width(350.0)
                    .selected_text(&self.selected_output_config)
                    .show_ui(ui, |ui| {
                        if let Some(device) = host
                            .devices()
                            .unwrap()
                            .find(|device| device.name().unwrap() == self.selected_audio_device)
                        {
                            for input_config in device.supported_output_configs().unwrap() {
                                let formatted = format_stream_config(&input_config);
                                ui.selectable_value(
                                    &mut self.selected_output_config,
                                    formatted.clone(),
                                    formatted,
                                );
                            }
                        }
                    });
            })
        });

        // it always MUST be the latest. See https://docs.rs/egui/latest/egui/containers/panel/struct.CentralPanel.html
        egui::CentralPanel::default().show(ctx, |ui| {
            // if ui.button("Play").clicked() {
            //     self.audio.play();
            // }

            if ui.button("+").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("CLAP bundle/plugin", &["clap"])
                    .pick_file()
                {
                    self.plugins_container.load(&path.display().to_string());
                }
            }

            ui.vertical(|ui| {
                let label = if self.plugins_container.is_empty() {
                    "There's no plugins yet"
                } else {
                    "Loaded plugins:"
                };
                ui.label(label);

                ui.horizontal(|ui| {
                    for (index, plugin) in self.plugins_container.plugins.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            if ui.button("-").clicked() {
                                self.plugins_to_remove.push(index);
                            }
                            ui.vertical(|ui| {
                                ui.label(plugin.name());

                                let mut changed_params = vec![];
                                for param in &plugin.params {
                                    ui.horizontal(|ui| {
                                        ui.add(
                                            Slider::from_get_set(
                                                param.min_value..=param.max_value,
                                                |value| {
                                                    if let Some(value) = value {
                                                        changed_params.push((param.id, value));
                                                    }

                                                    param.value
                                                },
                                            )
                                            .text(&param.name),
                                        );
                                    });
                                }

                                for (param_id, value) in changed_params {
                                    plugin.set_value(param_id, value);
                                }
                            })
                        });
                    }
                });

                self.plugins_to_remove.sort();
                self.plugins_to_remove.reverse();

                for index in &self.plugins_to_remove {
                    self.plugins_container.unload(*index);
                }

                self.plugins_to_remove = vec![];
            })
        });
    }
}

fn format_stream_config(config: &SupportedStreamConfigRange) -> String {
    let sample_rate = if config.min_sample_rate() == config.max_sample_rate() {
        format!("{}", config.min_sample_rate().0)
    } else {
        format!(
            "{} - {}",
            config.min_sample_rate().0,
            config.max_sample_rate().0
        )
    };

    format!("{}, {sample_rate}", config.channels())
}
