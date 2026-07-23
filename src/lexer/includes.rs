use crate::{
    error::lexer_error::LexerError,
    lexer::tokens::{Token, TokenType},
};

use super::lexer::Lexer;

use std::path::PathBuf;

impl Lexer {
    pub(crate) fn process_includes(
        &mut self,
        current_file: &PathBuf,
        tokens: &mut Vec<Token>,
    ) -> Result<(), LexerError> {
        if self
            .processed_files
            .contains(&self.get_filename(current_file))
        {
            *tokens = Vec::new();
            return Ok(());
        }

        if self
            .processing_files
            .contains(&self.get_filename(current_file))
        {
            return Err(LexerError::CircularInclude(self.get_filename(current_file)));
        }

        self.processing_files
            .insert(self.get_filename(current_file));

        let new_tokens: Vec<_> = tokens
            .iter()
            .filter(|t| match &t.token_type {
                TokenType::Include(_) => true,
                _ => false,
            })
            .map(|t| match &t.token_type {
                TokenType::Include(s) => s.clone(),
                _ => unreachable!(),
            })
            .map(|p| self.scan_file(current_file, p[1..(p.len() - 1)].to_string()))
            .collect();

        let mut complete_tokens = Vec::new();
        for t in new_tokens.iter() {
            let mut tokens = t.clone()?;
            complete_tokens.append(&mut tokens);
        }

        let mut original_tokens: Vec<Token> = tokens
            .iter()
            .filter(|t| match &t.token_type {
                TokenType::Include(_) => false,
                _ => true,
            })
            .cloned()
            .collect();

        complete_tokens.append(&mut original_tokens);
        *tokens = complete_tokens;

        self.processing_files
            .remove(&self.get_filename(current_file));

        self.processed_files.insert(self.get_filename(current_file));

        Ok(())
    }
}
