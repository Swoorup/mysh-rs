pub struct Prompt {
    prompt_str: String,
}

impl Prompt {
    pub fn new(prompt_str: String) -> Prompt {
        Prompt {
            prompt_str: prompt_str,
        }
    }

    pub fn set_prompt(&mut self, new_prompt_str: String) {
        self.prompt_str = new_prompt_str;
    }

    pub fn get_prompt(&self) -> &String {
        &self.prompt_str
    }

    // just for fun, this is unnecessary
    pub fn move_prompt(&mut self) -> String {
        ::std::mem::replace(&mut self.prompt_str, String::new())
    }
}

#[test]
fn test_set_prompt() {
    let prompt = Prompt::new(String::from("sytherax% "));
    assert!(prompt.get_prompt().as_str() == "sytherax% ");

    prompt.set_prompt(String::from("swoorup% "));
    assert!(prompt.get_prompt().as_str() == "swoorup% ");
}

#[test]
fn test_get_prompt() {
    let prompt = Prompt::new(String::from("sytherax% "));
    assert!(prompt.get_prompt().as_str() == "sytherax% ");
    assert!(prompt.get_prompt().as_str() == "sytherax% ");
}

#[test]
fn test_move_prompt() {
    let mut prompt = Prompt::new(String::from("sytherax% "));

    assert!(prompt.move_prompt().as_str() == "sytherax% ");
    assert!(prompt.move_prompt().as_str() != "sytherax% ");
    assert!(prompt.move_prompt().as_str() == "");
}
