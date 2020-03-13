pub enum Shell {
    Bash,
    Sh,
    Zsh,
    Fish,
    Cmd,
    PowerShell,
}

impl Shell {
    pub fn export(self, id: String, secret: String, token: String, ps: String) {
        match self {
            Shell::Bash | Shell::Sh | Shell::Zsh => {
                println!("export {}={}", "AWS_ACCESS_KEY", id);
                println!("export {}={}", "AWS_SECRET_KEY", secret);
                println!("export {}={}", "AWS_SESSION_TOKEN", token);
                println!("export {}='{}'", "PS1", ps);
            }
            Shell::Fish => {
                println!("set -x {} \"{}\"", "AWS_ACCESS_KEY", id);
                println!("set -x {} \"{}\"", "AWS_SECRET_KEY", secret);
                println!("set -x {} \"{}\"", "AWS_SESSION_TOKEN", token);
                println!("set -x {} \"{}\"", "PS1", ps);
            }
            Shell::Cmd => {
                println!("set {}={}", "AWS_ACCESS_KEY", id);
                println!("set {}={}", "AWS_SECRET_KEY", secret);
                println!("set {}={}", "AWS_SESSION_TOKEN", token);
            }
            Shell::PowerShell => {
                println!(
                    "Set-Variable -Name \"{}\" -Value \"{}\"",
                    "AWS_ACCESS_KEY", id
                );
                println!(
                    "Set-Variable -Name \"{}\" -Value \"{}\"",
                    "AWS_SECRET_KEY", secret
                );
                println!(
                    "Set-Variable -Name \"{}\" -Value \"{}\"",
                    "AWS_SESSION_TOKEN", token
                );
            }
        }
    }
}

impl Default for Shell {
    fn default() -> Self {
        Shell::Bash
    }
}

impl<'a> From<&'a str> for Shell {
    fn from(s: &'a str) -> Self {
        match s {
            "/bin/bash" | "/usr/bin/bash" | "/usr/local/bin/bash" => Shell::Bash,
            "/bin/zsh" | "/usr/bin/zsh" | "/usr/local/bin/zsh" => Shell::Zsh,
            "/bin/sh" => Shell::Sh,
            "/bin/fish" | "/usr/local/bin/fish" => Shell::Fish,
            // to_lowercase ?
            s if s.ends_with("cmd.exe") => Shell::Cmd,
            s if s.ends_with("Powershell.exe") => Shell::PowerShell,
            _ => Default::default(),
        }
    }
}
