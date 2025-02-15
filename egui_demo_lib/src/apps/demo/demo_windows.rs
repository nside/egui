use egui::{CtxRef, ScrollArea, Ui, Window};
use std::collections::BTreeSet;

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
struct Demos {
    #[cfg_attr(feature = "persistence", serde(skip))]
    demos: Vec<Box<dyn super::Demo>>,

    open: BTreeSet<String>,
}
impl Default for Demos {
    fn default() -> Self {
        let demos: Vec<Box<dyn super::Demo>> = vec![
            Box::new(super::dancing_strings::DancingStrings::default()),
            Box::new(super::drag_and_drop::DragAndDropDemo::default()),
            Box::new(super::zoom_rotate::ZoomRotate::default()),
            Box::new(super::font_book::FontBook::default()),
            Box::new(super::DemoWindow::default()),
            Box::new(super::painting::Painting::default()),
            Box::new(super::plot_demo::PlotDemo::default()),
            Box::new(super::scrolling::Scrolling::default()),
            Box::new(super::sliders::Sliders::default()),
            Box::new(super::widget_gallery::WidgetGallery::default()),
            Box::new(super::window_options::WindowOptions::default()),
            Box::new(super::tests::WindowResizeTest::default()),
            // Tests:
            Box::new(super::tests::CursorTest::default()),
            Box::new(super::tests::IdTest::default()),
            Box::new(super::tests::InputTest::default()),
            Box::new(super::layout_test::LayoutTest::default()),
            Box::new(super::tests::ManualLayoutTest::default()),
            Box::new(super::tests::TableTest::default()),
        ];

        use crate::apps::demo::Demo;
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::WidgetGallery::default()
                .name()
                .to_owned(),
        );

        Self { demos, open }
    }
}
impl Demos {
    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            ui.checkbox(&mut is_open, demo.name());
            set_open(open, demo.name(), is_open);
        }
    }

    pub fn show(&mut self, ctx: &CtxRef) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[derive(Default)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct DemoWindows {
    open_windows: OpenWindows,
    demos: Demos,
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    /// `sidebar_ui` can be used to optionally show some things in the sidebar
    pub fn ui(&mut self, ctx: &CtxRef) {
        egui::SidePanel::left("side_panel", 190.0).show(ctx, |ui| {
            ui.heading("✒ egui demos");

            ui.separator();

            ScrollArea::auto_sized().show(ui, |ui| {
                use egui::special_emojis::{GITHUB, OS_APPLE, OS_LINUX, OS_WINDOWS};

                ui.label("egui is an immediate mode GUI library written in Rust.");
                ui.hyperlink_to(
                    format!("{} egui home page", GITHUB),
                    "https://github.com/emilk/egui",
                );

                ui.label(format!(
                    "egui can be run on the web, or natively on {}{}{}",
                    OS_APPLE, OS_LINUX, OS_WINDOWS,
                ));

                ui.separator();

                ui.heading("Windows:");
                self.demos.checkboxes(ui);

                ui.separator();

                ui.label("egui:");
                self.open_windows.checkboxes(ui);

                ui.separator();

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory().reset_areas();
                }
            });
        });

        egui::TopPanel::top("menu_bar").show(ctx, |ui| {
            show_menu_bar(ui);
        });

        {
            let mut fill = ctx.style().visuals.extreme_bg_color;
            if !cfg!(target_arch = "wasm32") {
                // Native: WrapApp uses a transparent window, so let's show that off:
                // NOTE: the OS compositor assumes "normal" blending, so we need to hack it:
                let [r, g, b, _] = fill.to_array();
                fill = egui::Color32::from_rgba_premultiplied(r, g, b, 180);
            }
            let frame = egui::Frame::none().fill(fill);
            egui::CentralPanel::default().frame(frame).show(ctx, |_| {});
        }

        self.windows(ctx);
    }

    /// Show the open windows.
    fn windows(&mut self, ctx: &CtxRef) {
        let Self {
            open_windows,
            demos,
            ..
        } = self;

        Window::new("🔧 Settings")
            .open(&mut open_windows.settings)
            .scroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        Window::new("🔍 Inspection")
            .open(&mut open_windows.inspection)
            .scroll(true)
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

        Window::new("📝 Memory")
            .open(&mut open_windows.memory)
            .resizable(false)
            .show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        demos.show(ctx);
    }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct OpenWindows {
    // egui stuff:
    settings: bool,
    inspection: bool,
    memory: bool,
}

impl Default for OpenWindows {
    fn default() -> Self {
        OpenWindows::none()
    }
}

impl OpenWindows {
    fn none() -> Self {
        Self {
            settings: false,
            inspection: false,
            memory: false,
        }
    }

    fn checkboxes(&mut self, ui: &mut Ui) {
        let Self {
            settings,
            inspection,
            memory,
        } = self;

        ui.checkbox(settings, "🔧 Settings");
        ui.checkbox(inspection, "🔍 Inspection");
        ui.checkbox(memory, "📝 Memory");
    }
}

fn show_menu_bar(ui: &mut Ui) {
    use egui::*;

    menu::bar(ui, |ui| {
        menu::menu(ui, "File", |ui| {
            if ui.button("Organize windows").clicked() {
                ui.ctx().memory().reset_areas();
            }
            if ui
                .button("Clear egui memory")
                .on_hover_text("Forget scroll, collapsing headers etc")
                .clicked()
            {
                *ui.ctx().memory() = Default::default();
            }
        });
    });
}
