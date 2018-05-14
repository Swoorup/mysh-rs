use nix::sys::signal::*;
use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref PROMPT: Mutex<String> = Mutex::new("λ ".to_string());
    static ref OLD_SIGACTION: Mutex<Option<SigAction>> = Mutex::new(None);
}

pub fn set_prompt(prompt_str: &str) {
    *PROMPT.lock().unwrap() = prompt_str.to_string();
}

pub fn get_prompt() -> MutexGuard<'static, String> {
    PROMPT.lock().unwrap()
}

pub fn set_shell_signal_handlers() {
    let actions = SigAction::new(SigHandler::SigIgn, SaFlags::empty(), SigSet::empty());
    unsafe {
        sigaction(Signal::SIGTSTP, &actions).unwrap();
        sigaction(Signal::SIGQUIT, &actions).unwrap();
        *OLD_SIGACTION.lock().unwrap() = Some(sigaction(Signal::SIGINT, &actions).unwrap())
    }
}

pub fn disable_shell_signal_handlers() {
    if let Some(actions) = &*OLD_SIGACTION.lock().unwrap() {
        unsafe {
            sigaction(Signal::SIGINT, &actions).unwrap();
        }
    }
}

#[test]
fn test_prompt() {
    set_prompt("λ ");
    assert!(&(*get_prompt()) == "λ ");

    set_prompt("% ");
    assert!(&(*get_prompt()) == "% ");
}
