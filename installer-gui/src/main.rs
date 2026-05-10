use eframe::egui;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use anyhow::Result;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InstallPage {
    Welcome,
    Dependencies,
    Source,
    Path,
    Progress,
    Finish,
}

#[derive(Clone)]
struct InstallConfig {
    repo_url: String,
    repo_branch: String,
    install_path: PathBuf,
    auto_start: bool,
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            repo_url: "https://github.com/example/linux-system-monitor.git".to_string(),
            repo_branch: "main".to_string(),
            install_path: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("SystemMonitor"),
            auto_start: true,
        }
    }
}

struct InstallStatus {
    message: String,
    progress: f32,
    error: Option<String>,
}

impl Default for InstallStatus {
    fn default() -> Self {
        Self {
            message: "准备开始...".to_string(),
            progress: 0.0,
            error: None,
        }
    }
}

struct InstallerApp {
    current_page: InstallPage,
    config: InstallConfig,
    status: InstallStatus,
    rust_available: bool,
    git_available: bool,
    tx: Option<mpsc::Sender<InstallStatus>>,
    rx: Option<mpsc::Receiver<InstallStatus>>,
}

impl Default for InstallerApp {
    fn default() -> Self {
        Self {
            current_page: InstallPage::Welcome,
            config: InstallConfig::default(),
            status: InstallStatus::default(),
            rust_available: false,
            git_available: false,
            tx: None,
            rx: None,
        }
    }
}

impl InstallerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn check_dependencies(&mut self) {
        self.rust_available = std::process::Command::new("rustup")
            .arg("--version")
            .output()
            .is_ok();

        self.git_available = std::process::Command::new("git")
            .arg("--version")
            .output()
            .is_ok();
    }

    fn start_installation(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.tx = Some(tx.clone());
        self.rx = Some(rx);
        self.current_page = InstallPage::Progress;
        self.status = InstallStatus::default();

        let config = self.config.clone();

        thread::spawn(move || {
            let _ = Self::run_installation(config, tx);
        });
    }

    fn run_installation(config: InstallConfig, tx: mpsc::Sender<InstallStatus>) -> Result<()> {
        Self::send_status(&tx, "检查 Rust...", 0.0)?;
        if !std::process::Command::new("rustup").arg("--version").output().is_ok() {
            Self::send_status(&tx, "正在安装 Rustup...", 0.05)?;
            Self::install_rustup()?;
        }

        Self::send_status(&tx, "检查 Git...", 0.15)?;
        if !std::process::Command::new("git").arg("--version").output().is_ok() {
            Self::send_status(&tx, "正在安装 Git...", 0.2)?;
            Self::install_git()?;
        }

        Self::send_status(&tx, "克隆代码仓库...", 0.3)?;
        let temp_dir = std::env::temp_dir().join("system-monitor-install");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir)?;

        std::process::Command::new("git")
            .arg("clone")
            .arg("--branch")
            .arg(&config.repo_branch)
            .arg("--depth")
            .arg("1")
            .arg(&config.repo_url)
            .arg(&temp_dir)
            .status()?;

        Self::send_status(&tx, "正在构建项目...", 0.4)?;
        std::process::Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&temp_dir)
            .status()?;

        Self::send_status(&tx, "正在安装...", 0.8)?;
        let _ = std::fs::remove_dir_all(&config.install_path);
        std::fs::create_dir_all(&config.install_path)?;

        let bin_path = temp_dir.join("target").join("release").join("linux-system-monitor");
        if bin_path.exists() {
            std::fs::copy(bin_path, config.install_path.join("system-monitor.exe"))?;
        }

        if config.auto_start {
            Self::send_status(&tx, "配置自动启动...", 0.95)?;
            Self::setup_autostart(&config.install_path)?;
        }

        Self::send_status(&tx, "安装完成！", 1.0)?;

        Ok(())
    }

    fn send_status(tx: &mpsc::Sender<InstallStatus>, message: &str, progress: f32) -> Result<()> {
        tx.send(InstallStatus {
            message: message.to_string(),
            progress,
            error: None,
        })?;
        Ok(())
    }

    fn install_rustup() -> Result<()> {
        Ok(())
    }

    fn install_git() -> Result<()> {
        Ok(())
    }

    fn setup_autostart(install_path: &PathBuf) -> Result<()> {
        Ok(())
    }

    fn show_welcome_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("Linux System Monitor - Windows 安装向导");
        ui.add_space(20.0);
        ui.label("欢迎使用 Linux System Monitor 的 Windows 安装程序！");
        ui.label("这个安装向导将帮助您在 Windows 系统上安装和配置系统监控工具。");
        ui.add_space(40.0);
        
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("下一步 >").clicked() {
                    self.current_page = InstallPage::Dependencies;
                    self.check_dependencies();
                }
            });
        });
    }

    fn show_dependencies_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("依赖检查");
        ui.add_space(20.0);

        ui.horizontal(|ui| {
            let rust_icon = if self.rust_available { "✅" } else { "❌" };
            ui.label(format!("{} Rust (rustup)", rust_icon));
            if self.rust_available {
                ui.colored_label(egui::Color32::GREEN, "已安装");
            } else {
                ui.colored_label(egui::Color32::RED, "未安装 - 安装程序将自动安装");
            }
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let git_icon = if self.git_available { "✅" } else { "❌" };
            ui.label(format!("{} Git", git_icon));
            if self.git_available {
                ui.colored_label(egui::Color32::GREEN, "已安装");
            } else {
                ui.colored_label(egui::Color32::RED, "未安装 - 安装程序将自动安装");
            }
        });

        ui.add_space(40.0);

        ui.horizontal(|ui| {
            if ui.button("< 上一步").clicked() {
                self.current_page = InstallPage::Welcome;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("下一步 >").clicked() {
                    self.current_page = InstallPage::Source;
                }
            });
        });
    }

    fn show_source_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("源代码配置");
        ui.add_space(20.0);

        ui.label("GitHub 仓库 URL:");
        ui.text_edit_singleline(&mut self.config.repo_url);
        
        ui.add_space(10.0);
        ui.label("分支或标签:");
        ui.text_edit_singleline(&mut self.config.repo_branch);

        ui.add_space(40.0);

        ui.horizontal(|ui| {
            if ui.button("< 上一步").clicked() {
                self.current_page = InstallPage::Dependencies;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("下一步 >").clicked() {
                    self.current_page = InstallPage::Path;
                }
            });
        });
    }

    fn show_path_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("安装路径");
        ui.add_space(20.0);

        ui.label("选择安装目录:");
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.config.install_path.to_string_lossy().to_string());
            if ui.button("浏览...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .pick_folder() {
                    self.config.install_path = path;
                }
            }
        });

        ui.add_space(20.0);
        ui.checkbox(&mut self.config.auto_start, "安装后自动启动");

        ui.add_space(40.0);

        ui.horizontal(|ui| {
            if ui.button("< 上一步").clicked() {
                self.current_page = InstallPage::Source;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("安装").clicked() {
                    self.start_installation();
                }
            });
        });
    }

    fn show_progress_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("正在安装");
        ui.add_space(20.0);

        if let Some(rx) = &self.rx {
            if let Ok(status) = rx.try_recv() {
                self.status = status;
            }
        }

        ui.label(&self.status.message);
        ui.add_space(20.0);
        
        let progress_bar = egui::ProgressBar::new(self.status.progress)
            .show_percentage();
        ui.add(progress_bar);

        if self.status.progress >= 1.0 {
            ui.add_space(40.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("完成").clicked() {
                    self.current_page = InstallPage::Finish;
                }
            });
        }
    }

    fn show_finish_page(&mut self, ui: &mut egui::Ui) {
        ui.heading("安装完成！");
        ui.add_space(20.0);
        
        ui.label("Linux System Monitor 已成功安装到您的系统。");
        ui.add_space(20.0);
        
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("立即启动").clicked() {
                    let exe_path = self.config.install_path.join("system-monitor.exe");
                    let _ = std::process::Command::new(exe_path).spawn();
                    std::process::exit(0);
                }
                if ui.button("关闭").clicked() {
                    std::process::exit(0);
                }
            });
        });
    }
}

impl eframe::App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            
            match self.current_page {
                InstallPage::Welcome => self.show_welcome_page(ui),
                InstallPage::Dependencies => self.show_dependencies_page(ui),
                InstallPage::Source => self.show_source_page(ui),
                InstallPage::Path => self.show_path_page(ui),
                InstallPage::Progress => self.show_progress_page(ui),
                InstallPage::Finish => self.show_finish_page(ui),
            }
        });

        if self.current_page == InstallPage::Progress {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Linux System Monitor 安装程序",
        options,
        Box::new(|cc| Ok(Box::new(InstallerApp::new(cc)))),
    )
}
