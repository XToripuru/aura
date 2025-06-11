use crossterm::style::Color;

pub const APPS_DIRECTORIES: &[&str] = &[
    "/usr/share/applications",
    "~/.wine/drive_c/users/$USER/AppData/Roaming/",
];

pub const IGNORED_APPS: &[&str] = &[
    "Code - OSS - URL Handler",
    "Pinentry",
    "Qt V4L2 test Utility",
    "Xwayland",
    "Electron 34",
    "Electron 32",
    "Avahi Zeroconf Browser",
    "Avahi VNC Server Browser",
    "Avahi SSH Server Browser",
    "OpenStreetMap",
    "Google Maps",
    "Zenity",
    "wheelmap.org",
    "CMake",
    "Run Software",
    "User folders update",
    "Update",
    "Uninstall",
    // These below don't work as is but they could work using scripts.
    "nvtop",
    "Htop",
    "Neovim",
    "Alacritty",
];

pub const SCRIPTS: &[(&str, &str)] = &[("Shutdown", "poweroff")];

pub const ACTIVE: Color = Color::White;
pub const INACTIVE: Color = Color::DarkGrey;
