use eframe::egui::{Color32, RichText};
use eframe::{egui, App};
use egui_nodes::{Context, LinkArgs, NodeArgs, NodeConstructor, PinArgs, PinShape};

#[derive(Default)]
struct MyApp {
	ctx: Context,
	links: Vec<(usize, usize)>,
}

pub fn example_graph(ctx: &mut Context, links: &mut Vec<(usize, usize)>, ui: &mut egui::Ui) {
	// add nodes with attributes
	let nodes = vec![
		NodeConstructor::new(0, NodeArgs { outline: Some(Color32::WHITE), ..Default::default() })
			.with_origin([50.0, 150.0].into())
			.with_title(|ui| ui.label(RichText::new("Example Node A").size(20.0).color(Color32::WHITE)))
			.with_input_attribute(0, Default::default(), |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
			})
			.with_static_attribute(1, |ui| ui.label("Can't Connect to Me"))
			.with_output_attribute(2, Default::default(), |ui| {
				ui.label(RichText::new("Output").color(Color32::WHITE))
			}),
		NodeConstructor::new(1, NodeArgs { outline: Some(Color32::WHITE), ..Default::default() })
			.with_origin([225.0, 150.0].into())
			.with_title(|ui| ui.label(RichText::new("Example Node B").size(20.0).color(Color32::WHITE)))
			.with_static_attribute(3, |ui| ui.label("Can't Connect to Me"))
			.with_input_attribute(5, Default::default(), |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
			})
			.with_output_attribute(4, Default::default(), |ui| {
				ui.label(RichText::new("Output").color(Color32::WHITE))
			}),
		NodeConstructor::new(2, NodeArgs { outline: Some(Color32::WHITE), ..Default::default() })
			.with_origin([325.0, 150.0].into())
			.with_title(|ui| ui.label(RichText::new("Example Node C").size(20.0).color(Color32::WHITE)))
			.with_static_attribute(6, |ui| ui.label("Can't Connect to Me"))
			.with_input_attribute(8, Default::default(), |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
			})
			.with_output_attribute(7, Default::default(), |ui| {
				ui.label(RichText::new("Output").color(Color32::WHITE))
			}),
		NodeConstructor::new(3, NodeArgs { outline: Some(Color32::WHITE), ..Default::default() })
			.with_origin([425.0, 150.0].into())
			.with_title(|ui| ui.label(RichText::new("Example Node D").size(20.0).color(Color32::WHITE)))
			.with_static_attribute(9, |ui| ui.label("Can't Connect to Me"))
			.with_input_attribute(11, Default::default(), |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
			})
			.with_output_attribute(10, Default::default(), |ui| {
				ui.label(RichText::new("Output").color(Color32::WHITE))
			}),
	];

	ctx.show(
		nodes,
		links.iter().enumerate().map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
		ui,
	);

	// remove destroyed links
	if let Some(idx) = ctx.link_destroyed() {
		links.remove(idx);
	}

	// add created links
	if let Some((start, end, _)) = ctx.link_created() {
		links.push((start, end))
	}
}

impl MyApp {
	fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		// Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
		// Restore app state using cc.storage (requires the "persistence" feature).
		// Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
		// for e.g. egui::PaintCallback.
		Self::default()
	}
}

impl App for MyApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Tubes");
			example_graph(&mut self.ctx, &mut self.links, ui);
		});

		// Resize the native window to be just the size we need it to be:
		frame.set_window_size(ctx.used_size());
	}
}

fn main() {
	eframe::run_native(
		"Tubes",
		eframe::NativeOptions::default(),
		Box::new(|cc| Box::new(MyApp::new(cc))),
	);
}
