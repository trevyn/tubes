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
			.with_input_attribute(0, PinArgs { shape: PinShape::Triangle, ..Default::default() }, |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
			})
			.with_static_attribute(1, |ui| ui.label("Can't Connect to Me"))
			.with_output_attribute(
				2,
				PinArgs { shape: PinShape::TriangleFilled, ..Default::default() },
				|ui| ui.label(RichText::new("Output").color(Color32::WHITE)),
			),
		NodeConstructor::new(1, NodeArgs { outline: Some(Color32::WHITE), ..Default::default() })
			.with_origin([225.0, 150.0].into())
			.with_title(|ui| ui.label(RichText::new("Example Node B").size(20.0).color(Color32::WHITE)))
			.with_static_attribute(3, |ui| ui.label("Can't Connect to Me"))
			.with_output_attribute(4, Default::default(), |ui| {
				ui.label(RichText::new("Output").color(Color32::WHITE))
			})
			.with_input_attribute(5, Default::default(), |ui| {
				ui.label(RichText::new("Input").color(Color32::WHITE))
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

impl App for MyApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Turbonodes");
			example_graph(&mut self.ctx, &mut self.links, ui);
		});

		// Resize the native window to be just the size we need it to be:
		frame.set_window_size(ctx.used_size());
	}
}

fn main() {
	eframe::run_native(
		"Turbonodes",
		eframe::NativeOptions::default(),
		Box::new(|_cc| Box::new(MyApp::default())),
	);
}
