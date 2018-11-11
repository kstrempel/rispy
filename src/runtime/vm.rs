use runtime::tokens::Token;
use runtime::environment::Machine;


impl Machine {
    pub fn eval_parser(&mut self, tree: &Vec<Token>) -> Token {
        let mut iter = tree.into_iter();
        let mut result = Token::None;
        self.push_environment();
        while let Some(token) = iter.next() {
            match token {
                Token::Atom(symbol) => {
                    result = self.match_atom(symbol, &mut iter);
                }
                Token::Block(subs) => {
                    result = self.eval_parser(subs);
                }
                _ => {
                    println!("Panic at {:?}", tree);
                    result = Token::None;
                }
            }
        }

        self.pop_environment();
        result
    }

    pub fn get_name_and_result(name: Option<&Token>, value: Option<&Token>) -> Result<(String, Token), String> {
        let token_name = name.expect("error in define - name");
        let token_value = value.expect("error in define - value");

        let name = token_name.atom_2_string().unwrap();

        Ok((name, token_value.clone()))
    }
}
