//! Unified application launcher for the Lunola Creative Suite.
//!
//! Provides a launcher screen that can open any of the three apps (wordcraft, deckcraft,
//! gridcraft). The launcher is accessible from the Backstage view and shows all available
//! apps with their icons and descriptions.

use egui::{RichText, Sense, Ui};

use crate::lunola::SuiteApp;
use crate::theme;
use crate::SheetApp;

/// The app IDs that can be launched.
pub use crate::lunola::SuiteApp as AppId;

/// Returns the display name for an app.
pub fn app_name(app: SuiteApp) -> &'static str {
    app.display_name()
}

/// Returns the description for an app.
pub fn app_description(app: SuiteApp) -> &'static str {
    app.description()
}

/// Attempt to launch the given app. In a desktop context this would spawn the process;
/// in the web context it shows a message. Returns true if the launch was initiated.
pub fn launch_app(_app: SuiteApp) -> bool {
    false
}

/// Draws the launcher UI: a grid of app tiles that can be clicked to launch.
pub fn show_launcher(_app: &mut SheetApp, ui: &mut Ui) {
    let t = crate::theme::Tokens::get(ui.ctx());
    ui.label(RichText::new("Lunola Creative Suite").font(theme::ui_bold(22.0)).color(t.text));
    ui.add_space(8.0);
    ui.label(RichText::new("Choose an app to open").font(theme::ui_font(13.0)).color(t.text_dim));
    ui.add_space(20.0);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(16.0, 16.0);
        for suite_app in SuiteApp::ALL {
            app_tile(ui, suite_app, &t);
        }
    });
}

fn app_tile(ui: &mut Ui, suite_app: SuiteApp, t: &crate::theme::Tokens) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(180.0, 160.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, suite_app.display_name()));

    let p = ui.painter();
    let hovered = resp.hovered();
    let bg = if hovered { t.hover } else { t.window };
    p.rect_filled(rect, 8.0, bg);
    p.rect_stroke(
        rect,
        8.0,
        egui::Stroke::new(if hovered { 2.0 } else { 1.0 }, if hovered { t.accent } else { t.ribbon_border }),
        egui::StrokeKind::Inside,
    );

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.center().x, rect.top() + 40.0),
        egui::vec2(48.0, 48.0),
    );
    // Draw a simple colored circle as a placeholder icon
    p.circle_filled(icon_rect.center(), 20.0, t.accent);
    let label = match suite_app {
        SuiteApp::Wordcraft => "W",
        SuiteApp::Deckcraft => "D",
        SuiteApp::Gridcraft => "G",
    };
    let galley = p.layout(label.to_string(), theme::ui_font(20.0), t.text, 40.0);
    p.galley(
        egui::pos2(icon_rect.center().x - galley.size().x / 2.0, icon_rect.center().y - galley.size().y / 2.0),
        galley,
        t.text,
    );

    let name_galley = p.layout(
        suite_app.display_name().to_string(),
        theme::ui_font(14.0),
        t.text,
        rect.width() - 16.0,
    );
    p.galley(
        egui::pos2(rect.center().x - name_galley.size().x / 2.0, rect.top() + 100.0),
        name_galley,
        t.text,
    );

    let desc_galley = p.layout(
        suite_app.description().to_string(),
        theme::ui_font(11.0),
        t.text_dim,
        rect.width() - 16.0,
    );
    p.galley(
        egui::pos2(rect.center().x - desc_galley.size().x / 2.0, rect.top() + 122.0),
        desc_galley,
        t.text_dim,
    );

    if resp.clicked() {
        launch_app(suite_app);
    }
}
