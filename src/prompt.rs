struct Prompt {
    prompt_str: String,
    signal_set: bool
}

impl Prompt {
    fn new() -> Prompt {
        Prompt{ prompt_str: String::from("%"), signal_set: false }
    }

    fn set_prompt(&mut self, new_prompt_str: String) {
        *self.prompt_str = new_prompt_str;
    }
}
