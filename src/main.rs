use std::{
    fs,
    io::{self, BufRead, BufReader, Stdout, Write, stdout},
    os::unix::process::CommandExt,
    path::Path,
    process::{Command, Stdio},
};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, read},
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

mod config;

fn main() -> std::io::Result<()> {
    let mut ctx = Context::new()?;

    loop {
        match read()? {
            Event::Key(key) => {
                ctx.key(key);
            }
            _ => {}
        }
        if ctx.exit {
            break;
        }
        ctx.render()?;
    }

    Ok(())
}

struct Context {
    selected: u16,
    offset: u16,
    apps: Vec<App>,
    filter: String,
    filtered: Vec<App>,
    out: Stdout,
    _w: u16,
    h: u16,
    exit: bool,
}

impl Context {
    fn new() -> io::Result<Self> {
        let mut apps = apps(config::APPS_DIRECTORIES);

        apps.retain(|app| !config::IGNORED_APPS.contains(&&*app.name));

        for (name, exec) in config::SCRIPTS {
            apps.push(App {
                name: name.to_string(),
                name_lower: name.to_ascii_lowercase(),
                exec: exec.to_string(),
            });
        }

        apps.sort_by(|a, b| a.name.cmp(&b.name));

        let (w, h) = terminal::size()?;
        let mut ctx = Context {
            selected: 0,
            offset: 0,
            apps: apps.clone(),
            filter: String::new(),
            filtered: apps.clone(),
            out: stdout(),
            _w: w,
            h,
            exit: false,
        };

        execute!(
            ctx.out,
            EnterAlternateScreen,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            terminal::DisableLineWrap,
        )?;
        terminal::enable_raw_mode()?;

        Ok(ctx)
    }
    fn render(&mut self) -> io::Result<()> {
        execute!(self.out, Clear(ClearType::All))?;

        for i in 0..self.h {
            let idx = i + self.offset;
            let Some(app) = self.filtered.get(idx as usize) else {
                break;
            };
            execute!(
                self.out,
                cursor::MoveTo(1, i),
                SetForegroundColor(if idx == self.selected {
                    config::ACTIVE
                } else {
                    config::INACTIVE
                }),
                Print(&app.name),
                ResetColor
            )?;
        }

        self.out.flush()
    }
    fn key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.exit = true;
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                    self.offset = self.offset.min(self.selected);
                }
            }
            KeyCode::Down => {
                if self.selected < self.filtered.len() as u16 - 1 {
                    self.selected += 1;
                    self.offset = self.offset.max(self.selected.saturating_sub(self.h - 1));
                }
            }
            KeyCode::Backspace => {
                self.filter.pop();
                self.update_filtered();
                self.selected = 0;
                self.offset = 0;
            }
            KeyCode::Enter => {
                run_exec_line(&self.filtered[self.selected as usize].exec);
                self.exit = true;
            }
            code => {
                if let Some(ch) = code.as_char() {
                    self.filter.push(ch.to_ascii_lowercase());
                    self.update_filtered();
                    self.selected = 0;
                    self.offset = 0;
                }
            }
        }
    }
    fn update_filtered(&mut self) {
        self.filtered = self.apps.clone();
        self.filtered
            .retain(|app| app.name_lower.contains(&self.filter));
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        execute!(
            self.out,
            LeaveAlternateScreen,
            cursor::Show,
            terminal::EnableLineWrap
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn run_exec_line(exec_line: &str) {
    let cleaned = exec_line
        .split_whitespace()
        .filter(|token| !token.starts_with('%'))
        .collect::<Vec<_>>();

    if cleaned.is_empty() {
        return;
    }

    let command = cleaned[0];
    let args = &cleaned[1..];

    let mut cmd = Command::new(command);
    cmd.args(args);

    unsafe {
        cmd.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }

    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
}

fn apps(paths: &[impl AsRef<Path>]) -> Vec<App> {
    let mut apps = Vec::new();

    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "desktop") {
                    if let Ok(file) = fs::File::open(&path) {
                        let reader = BufReader::new(file);
                        let mut name = None;
                        let mut exec = None;
                        let mut in_desktop_entry = false;

                        for line in reader.lines().flatten() {
                            if line.trim() == "[Desktop Entry]" {
                                in_desktop_entry = true;
                            } else if in_desktop_entry {
                                if line.starts_with('[') {
                                    break;
                                } else if line.starts_with("Name=") {
                                    name = Some(line["Name=".len()..].to_string());
                                } else if line.starts_with("Exec=") {
                                    exec = Some(line["Exec=".len()..].to_string());
                                }
                            }
                        }

                        if let (Some(name), Some(exec)) = (name, exec) {
                            apps.push(App {
                                name: name.clone(),
                                name_lower: name.to_ascii_lowercase(),
                                exec,
                            });
                        }
                    }
                }
            }
        }
    }

    apps
}

#[derive(Debug, Clone)]
struct App {
    name: String,
    name_lower: String,
    exec: String,
}
