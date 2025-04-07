use crate::data::quiz_data;
use crate::models::quiz::Quiz;
use egui;

pub struct MyApp {
    quiz: Option<Quiz>,
    current_question: usize,
    selected_answer: Option<usize>,
    show_feedback: bool,
    score: usize,
    state: AppState,
}

#[derive(PartialEq)]
enum AppState {
    MainMenu,
    Playing,
    Results,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            quiz: None,
            current_question: 0,
            selected_answer: None,
            show_feedback: false,
            score: 0,
            state: AppState::MainMenu,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up custom fonts and styles if needed
        setup_custom_styles(&cc.egui_ctx);

        Self::default()
    }

    fn start_quiz(&mut self) {
        // Load quiz data
        let mut quiz = quiz_data::create_sample_quiz();

        // Shuffle answers using our platform-independent random implementation
        quiz.shuffle_answers();

        self.quiz = Some(quiz);
        self.current_question = 0;
        self.selected_answer = None;
        self.show_feedback = false;
        self.score = 0;
        self.state = AppState::Playing;
    }

    fn next_question(&mut self) {
        if let Some(quiz) = &self.quiz {
            if self.current_question < quiz.total_questions() - 1 {
                self.current_question += 1;
                self.selected_answer = None;
                self.show_feedback = false;
            } else {
                self.state = AppState::Results;
            }
        }
    }

    fn render_main_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading("Bonauer Quiz");
            ui.add_space(20.0);

            if ui.button("Start Quiz").clicked() {
                self.start_quiz();
            }

            ui.add_space(20.0);
            if ui.button("Exit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn render_quiz(&mut self, ui: &mut egui::Ui) {
        if let Some(quiz) = &self.quiz {
            if self.current_question < quiz.questions.len() {
                let question = &quiz.questions[self.current_question];

                ui.vertical(|ui| {
                    // Question header
                    ui.heading(format!(
                        "Question {} of {}",
                        self.current_question + 1,
                        quiz.questions.len()
                    ));
                    ui.separator();

                    // Question text
                    ui.label(&question.question_text);
                    ui.add_space(20.0);

                    // Answers
                    for (idx, answer) in question.answers.iter().enumerate() {
                        let button_text = format!("{}", answer.text);
                        let is_selected = self.selected_answer == Some(idx);

                        let mut button = egui::Button::new(button_text);

                        if is_selected {
                            button = button.fill(egui::Color32::LIGHT_BLUE);
                        }

                        if self.show_feedback {
                            if answer.correct {
                                button = button.fill(egui::Color32::LIGHT_GREEN);
                            } else if is_selected {
                                button = button.fill(egui::Color32::LIGHT_RED);
                            }
                        }

                        if ui.add(button).clicked() && !self.show_feedback {
                            self.selected_answer = Some(idx);
                        }
                    }

                    // Explanation if available and showing feedback
                    if self.show_feedback {
                        if let Some(explanation) = &question.explanation {
                            ui.add_space(10.0);
                            ui.label(explanation);
                        }
                    }

                    ui.add_space(20.0);

                    // Submit button
                    if !self.show_feedback {
                        if ui.button("Submit").clicked() && self.selected_answer.is_some() {
                            self.show_feedback = true;

                            // Check if answer is correct
                            if let Some(selected) = self.selected_answer {
                                if question.answers[selected].correct {
                                    self.score += 1;
                                }
                            }
                        }
                    } else {
                        if ui.button("Next Question").clicked() {
                            self.next_question();
                        }
                    }
                });
            }
        }
    }

    fn render_results(&mut self, ui: &mut egui::Ui) {
        if let Some(quiz) = &self.quiz {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("Quiz Results");
                ui.add_space(20.0);

                let score_text = format!("Score: {} out of {}", self.score, quiz.total_questions());
                ui.label(score_text);

                let percentage = (self.score as f32 / quiz.total_questions() as f32) * 100.0;
                let percentage_text = format!("{}%", percentage as i32);
                ui.label(percentage_text);

                ui.add_space(30.0);

                if ui.button("Try Again").clicked() {
                    self.start_quiz();
                }

                ui.add_space(10.0);

                if ui.button("Back to Main Menu").clicked() {
                    self.state = AppState::MainMenu;
                }
            });
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.state {
            AppState::MainMenu => self.render_main_menu(ui),
            AppState::Playing => self.render_quiz(ui),
            AppState::Results => self.render_results(ui),
        });
    }
}

fn setup_custom_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // Increase text size for better readability
    style
        .text_styles
        .get_mut(&egui::TextStyle::Body)
        .unwrap()
        .size = 16.0;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Button)
        .unwrap()
        .size = 16.0;
    style
        .text_styles
        .get_mut(&egui::TextStyle::Heading)
        .unwrap()
        .size = 24.0;

    ctx.set_style(style);
}
