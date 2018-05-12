use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
    static ref PROMPT: Mutex<String> = Mutex::new("λ ".to_string());
}

pub fn set_prompt(prompt_str: &str) {
    *PROMPT.lock().unwrap() = prompt_str.to_string();
}

pub fn get_prompt() -> MutexGuard<'static, String> {
    PROMPT.lock().unwrap()
}

#[test]
fn test_prompt(){
    set_prompt("λ ");
    assert!(&(*get_prompt()) == "λ ");

    set_prompt("% ");
    assert!(&(*get_prompt()) == "% ");
}