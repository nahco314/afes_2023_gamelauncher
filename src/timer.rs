use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;
use std::string::ToString;
use std::thread::sleep;
use std::time::{Duration, Instant};

const POPUP_EVERY_N_SEC: u64 = 5;

static BAT_CONTENT: &str = r#"
chcp 65001 >nul
@powershell -Command "Add-Type -AssemblyName System.Windows.Forms;$result = [System.Windows.Forms.MessageBox]::Show(\"メッセージ文1`nメッセージ文2`nメッセージ文3\", 'タイトル', 'YesNoCancel', 'Asterisk');exit $result;"
@echo 戻り値は %ERRORLEVEL% です
pause
"#;

pub fn inf_popup() {
    let mut start = Instant::now();

    loop {
        sleep(Duration::from_secs(1));

        if start.elapsed() >= Duration::from_secs(POPUP_EVERY_N_SEC) {
            let mut f = File::create(Path::new("./b.bat")).unwrap();
            write!(f, "{}", BAT_CONTENT).unwrap();

            f.seek(SeekFrom::Start(0)).unwrap();

            let mut game_cmd = Command::new("./b.bat");
            game_cmd.current_dir("./");
            game_cmd.spawn().expect("failed to show dialog");

            start = Instant::now();
        }
    }
}
