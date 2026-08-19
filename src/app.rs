use crate::content::Story;
use crate::game::GameState;
use crate::persistence::{LocalStore, UserSettings};
use eframe::egui::{self, Color32, RichText, Vec2};

const INK: Color32 = Color32::from_rgb(226, 238, 242);
const MUTED: Color32 = Color32::from_rgb(139, 166, 179);
const TEAL: Color32 = Color32::from_rgb(69, 218, 201);
const AMBER: Color32 = Color32::from_rgb(255, 191, 93);
const PANEL: Color32 = Color32::from_rgb(19, 30, 40);
const DEEP: Color32 = Color32::from_rgb(10, 18, 27);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Title,
    Play,
}

pub struct SignalZeroApp {
    story: Story,
    game: GameState,
    settings: UserSettings,
    screen: Screen,
    has_saved_game: bool,
    status: String,
    show_settings: bool,
}

impl SignalZeroApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>, story: Story) -> Self {
        let settings = LocalStore::load_settings().unwrap_or_default();
        let (game, has_saved_game, status) = match LocalStore::load_game() {
            Ok(Some(bundle)) if bundle.game.schema_version == GameState::SCHEMA_VERSION => (
                bundle.game,
                true,
                "A local case file is ready to continue.".to_owned(),
            ),
            Ok(Some(_)) => (
                GameState::new(&story),
                false,
                "An older case file was found and left untouched. Start a new investigation."
                    .to_owned(),
            ),
            Ok(None) => (
                GameState::new(&story),
                false,
                "No local case file found. Begin when ready.".to_owned(),
            ),
            Err(error) => (
                GameState::new(&story),
                false,
                format!("Local storage is unavailable: {error}"),
            ),
        };

        Self {
            story,
            game,
            settings,
            screen: Screen::Title,
            has_saved_game,
            status,
            show_settings: false,
        }
    }

    fn text_size(&self, base: f32) -> f32 {
        base * self.settings.text_scale
    }

    fn apply_style(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = PANEL;
        visuals.panel_fill = DEEP;
        visuals.faint_bg_color = Color32::from_rgb(28, 46, 59);
        visuals.extreme_bg_color = Color32::from_rgb(6, 12, 19);
        visuals.widgets.noninteractive.bg_fill = PANEL;
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(29, 52, 64);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(41, 78, 88);
        visuals.widgets.active.bg_fill = Color32::from_rgb(42, 104, 102);
        visuals.widgets.inactive.fg_stroke.color = INK;
        visuals.widgets.hovered.fg_stroke.color = Color32::WHITE;
        visuals.selection.bg_fill = Color32::from_rgb(44, 124, 121);
        visuals.selection.stroke.color = Color32::WHITE;
        if self.settings.high_contrast {
            visuals.window_fill = Color32::BLACK;
            visuals.panel_fill = Color32::BLACK;
            visuals.faint_bg_color = Color32::from_rgb(34, 34, 34);
            visuals.widgets.inactive.bg_fill = Color32::from_rgb(28, 28, 28);
            visuals.widgets.hovered.bg_fill = Color32::from_rgb(74, 74, 74);
        }
        ctx.set_visuals(visuals);
    }

    fn save(&mut self) {
        match LocalStore::save_game(&self.game, &self.settings) {
            Ok(()) => {
                self.has_saved_game = true;
                self.status = "Case file saved locally.".to_owned();
            }
            Err(error) => self.status = format!("Save failed: {error}"),
        }
    }

    fn save_settings(&mut self) {
        if let Err(error) = LocalStore::save_settings(&self.settings) {
            self.status = format!("Settings could not be saved: {error}");
        }
    }

    fn start_new(&mut self) {
        self.game = GameState::new(&self.story);
        self.status = "New investigation opened. Your first decision is waiting.".to_owned();
        self.screen = Screen::Play;
        self.save();
    }

    fn continue_case(&mut self) {
        self.screen = Screen::Play;
        self.status = "Case file reopened.".to_owned();
    }

    fn choose(&mut self, choice_id: &str) {
        match self.game.choose(&self.story, choice_id) {
            Ok(()) => {
                self.status = self.game.recent_event.clone();
                self.save();
            }
            Err(error) => self.status = error,
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if self.screen != Screen::Play || self.show_settings || self.game.completed {
            return;
        }
        let choices = self.game.current(&self.story).choices.clone();
        let keys = [
            egui::Key::Num1,
            egui::Key::Num2,
            egui::Key::Num3,
            egui::Key::Num4,
            egui::Key::Num5,
            egui::Key::Num6,
            egui::Key::Num7,
            egui::Key::Num8,
            egui::Key::Num9,
        ];
        for (index, choice) in choices.iter().enumerate() {
            if index < keys.len()
                && ctx.input(|input| input.key_pressed(keys[index]))
                && self.game.is_choice_available(choice)
            {
                self.choose(&choice.id);
                break;
            }
        }
        if ctx.input(|input| input.key_pressed(egui::Key::S)) {
            self.save();
        }
    }

    fn title_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(42.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("RUST LABORATORY").size(self.text_size(15.0)).color(TEAL).strong());
                ui.add_space(8.0);
                ui.label(RichText::new(self.story.title.as_str()).size(self.text_size(42.0)).color(INK).strong());
                ui.label(RichText::new(self.story.subtitle.as_str()).size(self.text_size(16.0)).color(MUTED));
                ui.add_space(28.0);
                ui.label(RichText::new("THE CASE FILE").size(self.text_size(12.0)).color(AMBER).strong());
                ui.add_space(6.0);
                ui.label(
                    RichText::new("The annex has been dark for eleven days. A core is still awake.\nFind the record, assemble the evidence, and decide what survives.")
                        .size(self.text_size(17.0))
                        .color(INK),
                );
                ui.add_space(28.0);

                let primary = egui::Button::new(RichText::new("OPEN NEW INVESTIGATION").size(self.text_size(16.0)).strong())
                    .fill(Color32::from_rgb(24, 116, 112))
                    .min_size(Vec2::new(320.0, 46.0));
                if ui.add(primary).clicked() {
                    self.start_new();
                }
                ui.add_space(8.0);
                let continuation = egui::Button::new(RichText::new("CONTINUE LOCAL CASE FILE").size(self.text_size(14.0)))
                    .min_size(Vec2::new(320.0, 40.0));
                if ui.add_enabled(self.has_saved_game, continuation).clicked() {
                    self.continue_case();
                }
                if !self.has_saved_game {
                    ui.label(RichText::new("No saved investigation is available yet.").size(self.text_size(13.0)).color(MUTED));
                }
                ui.add_space(16.0);
                if ui.button(RichText::new("Accessibility & settings").size(self.text_size(14.0)).color(TEAL)).clicked() {
                    self.show_settings = true;
                }
                ui.add_space(36.0);
                ui.label(RichText::new("Offline by default · No account · Local saves only").size(self.text_size(12.0)).color(MUTED));
                ui.label(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))).size(self.text_size(11.0)).color(MUTED));
            });
        });
    }

    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("SIGNAL ZERO")
                        .size(self.text_size(18.0))
                        .color(TEAL)
                        .strong(),
                );
                ui.separator();
                ui.label(
                    RichText::new(self.game.current(&self.story).location.as_str())
                        .size(self.text_size(12.0))
                        .color(MUTED),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(RichText::new("Settings").size(self.text_size(13.0)))
                        .clicked()
                    {
                        self.show_settings = true;
                    }
                    if ui
                        .button(RichText::new("Save [S]").size(self.text_size(13.0)))
                        .clicked()
                    {
                        self.save();
                    }
                    if ui
                        .button(RichText::new("Main menu").size(self.text_size(13.0)))
                        .clicked()
                    {
                        self.screen = Screen::Title;
                    }
                });
            });
            ui.add_space(4.0);
        });
    }

    fn map_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("map_panel")
            .resizable(false)
            .default_width(190.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("FACILITY MAP")
                        .size(self.text_size(13.0))
                        .color(TEAL)
                        .strong(),
                );
                ui.label(
                    RichText::new("Discovered locations only")
                        .size(self.text_size(11.0))
                        .color(MUTED),
                );
                ui.add_space(12.0);
                for node in self.story.nodes.iter().filter(|node| !node.ending) {
                    let discovered = self.game.visited_nodes.contains(&node.id);
                    let active = self.game.current_node == node.id;
                    let color = if active {
                        AMBER
                    } else if discovered {
                        INK
                    } else {
                        Color32::from_rgb(68, 83, 95)
                    };
                    let marker = if active {
                        "●"
                    } else if discovered {
                        "◇"
                    } else {
                        "·"
                    };
                    ui.label(
                        RichText::new(format!("{marker}  {}", node.map_zone))
                            .size(self.text_size(12.5))
                            .color(color),
                    );
                    ui.add_space(5.0);
                }
                ui.add_space(18.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(
                    RichText::new("CASE PROGRESS")
                        .size(self.text_size(12.0))
                        .color(MUTED)
                        .strong(),
                );
                ui.add(
                    egui::ProgressBar::new(self.game.progress(&self.story))
                        .desired_width(154.0)
                        .show_percentage(),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!(
                        "{} locations logged",
                        self.game.visited_nodes.len()
                    ))
                    .size(self.text_size(11.0))
                    .color(MUTED),
                );
            });
    }

    fn evidence_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("evidence_panel")
            .resizable(false)
            .default_width(245.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.label(
                    RichText::new("EVIDENCE LEDGER")
                        .size(self.text_size(13.0))
                        .color(AMBER)
                        .strong(),
                );
                ui.label(
                    RichText::new(format!("{} item(s) secured", self.game.evidence.len()))
                        .size(self.text_size(11.0))
                        .color(MUTED),
                );
                ui.add_space(10.0);
                if self.game.evidence.is_empty() {
                    ui.label(
                        RichText::new("No evidence yet. Inspect the annex carefully.")
                            .size(self.text_size(13.0))
                            .color(MUTED),
                    );
                }
                for evidence_id in self.game.evidence.clone() {
                    if let Some(evidence) = self.story.evidence(&evidence_id) {
                        egui::Frame::group(ui.style())
                            .fill(Color32::from_rgb(24, 43, 54))
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(evidence.title.as_str())
                                        .size(self.text_size(13.0))
                                        .color(INK)
                                        .strong(),
                                );
                                ui.add_space(3.0);
                                ui.label(
                                    RichText::new(evidence.description.as_str())
                                        .size(self.text_size(11.5))
                                        .color(MUTED),
                                );
                            });
                        ui.add_space(7.0);
                    }
                }
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        RichText::new("Your ledger is stored locally.")
                            .size(self.text_size(10.5))
                            .color(MUTED),
                    );
                });
            });
    }

    fn story_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let node = self.game.current(&self.story).clone();
            ui.add_space(12.0);
            ui.label(
                RichText::new(node.location.as_str())
                    .size(self.text_size(12.0))
                    .color(TEAL)
                    .strong(),
            );
            ui.add_space(6.0);
            ui.label(
                RichText::new(node.title.as_str())
                    .size(self.text_size(28.0))
                    .color(INK)
                    .strong(),
            );
            ui.add_space(13.0);
            for paragraph in node.body.split("\n\n") {
                ui.label(
                    RichText::new(paragraph)
                        .size(self.text_size(17.0))
                        .color(INK),
                );
                ui.add_space(10.0);
            }
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(12.0);

            if node.ending {
                ui.label(
                    RichText::new("CASE OUTCOME")
                        .size(self.text_size(12.0))
                        .color(AMBER)
                        .strong(),
                );
                ui.label(
                    RichText::new(self.game.outcome_label().unwrap_or("Recorded"))
                        .size(self.text_size(20.0))
                        .color(TEAL)
                        .strong(),
                );
                ui.add_space(14.0);
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("Open a new investigation").size(self.text_size(15.0)),
                        )
                        .min_size(Vec2::new(250.0, 38.0)),
                    )
                    .clicked()
                {
                    self.start_new();
                }
                return;
            }

            ui.label(
                RichText::new("AVAILABLE ACTIONS")
                    .size(self.text_size(12.0))
                    .color(AMBER)
                    .strong(),
            );
            ui.add_space(7.0);
            for (index, choice) in node.choices.iter().enumerate() {
                let available = self.game.is_choice_available(choice);
                let number = index + 1;
                let label = format!("{number}. {}", choice.label);
                let response = ui.add_enabled(
                    available,
                    egui::Button::new(RichText::new(label).size(self.text_size(15.0)))
                        .min_size(Vec2::new(ui.available_width(), 38.0)),
                );
                if response.clicked() {
                    self.choose(&choice.id);
                    break;
                }
                if self.settings.show_hints {
                    let hint = if available {
                        choice.hint.clone()
                    } else {
                        let missing = self.game.missing_requirements(choice, &self.story);
                        format!("Locked — requires: {}", missing.join(", "))
                    };
                    ui.label(
                        RichText::new(hint)
                            .size(self.text_size(11.5))
                            .color(if available { MUTED } else { AMBER }),
                    );
                }
                ui.add_space(8.0);
            }
            ui.add_space(10.0);
            egui::Frame::group(ui.style())
                .fill(Color32::from_rgb(20, 38, 49))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("SYSTEM LOG")
                            .size(self.text_size(11.0))
                            .color(TEAL)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(self.status.as_str())
                            .size(self.text_size(12.0))
                            .color(MUTED),
                    );
                });
        });
    }

    fn settings_window(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }
        let mut is_open = self.show_settings;
        egui::Window::new("Accessibility & settings")
            .open(&mut is_open)
            .resizable(false)
            .default_width(360.0)
            .show(ctx, |ui| {
                ui.label(RichText::new("READING & DISPLAY").color(TEAL).strong());
                ui.add_space(8.0);
                let text_changed = ui
                    .add(
                        egui::Slider::new(&mut self.settings.text_scale, 0.85..=1.45)
                            .text("Text scale"),
                    )
                    .changed();
                let contrast_changed = ui
                    .checkbox(&mut self.settings.high_contrast, "High contrast")
                    .changed();
                let motion_changed = ui
                    .checkbox(&mut self.settings.reduce_motion, "Reduce motion")
                    .changed();
                let hints_changed = ui
                    .checkbox(&mut self.settings.show_hints, "Show action hints")
                    .changed();
                ui.add_space(10.0);
                ui.label(
                    RichText::new(
                        "Keyboard: use 1–9 to choose an available action; S saves the case.",
                    )
                    .size(self.text_size(12.0))
                    .color(MUTED),
                );
                if text_changed || contrast_changed || motion_changed || hints_changed {
                    self.save_settings();
                }
            });
        self.show_settings = is_open;
    }
}

impl eframe::App for SignalZeroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_style(ctx);
        self.handle_shortcuts(ctx);
        match self.screen {
            Screen::Title => self.title_screen(ctx),
            Screen::Play => {
                self.top_bar(ctx);
                self.map_panel(ctx);
                self.evidence_panel(ctx);
                self.story_panel(ctx);
            }
        }
        self.settings_window(ctx);
    }
}
