use std::{cmp::max, borrow::Cow};


#[derive(Default)]
struct WriterState {
    indentation_level: usize,
    open_parenthesis: usize,
    open_braces: usize,
    open_brackets: usize,
    prefix: Option<Cow<'static, str>>,
}

/// Used to generate pretty indented code
#[derive(Default)]
pub struct PrettyWriter {
    state: WriterState,
    output: String,
}


impl PrettyWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Combines this writer with another one.
    /// 
    /// open brackets are inherited, the current string is indented with additional indentation
    /// to match this writers indentation level.
    /// If the indentation level is non-zero, it's added to this writer's.
    /// Any prefixes are ignored.
    pub fn extend(&mut self, other : PrettyWriter){

        for line in other.output.lines() {
            self.write_line(line);
        }
        self.state.open_parenthesis += other.state.open_parenthesis;
        self.state.open_braces += other.state.open_braces;
        self.state.open_brackets += other.state.open_brackets;
        self.state.indentation_level += other.state.indentation_level;
    }

    /// Inserts a newline character inline
    pub fn newline(&mut self) -> &mut Self {
        self.output.push('\n');
        self
    }

    /// Writes indentation and prefix without a newline
    fn write_indented_prefix(&mut self){
        (0..self.state.indentation_level).into_iter()
            .for_each(|_| self.output.push('\t'));
        
        if let Some(prefix) = &self.state.prefix {
            self.output.push_str(prefix);
        }
    }

    /// Writes a line at the current indentation level and append a newline at the end
    pub fn write_line(&mut self, line: &str) -> &mut Self {
        self.write_indented_prefix();
        self.output.push_str(line);
        self.output.push('\n');
        self
    }

    /// Writes without adding newline but keeps the indentation
    pub fn write_no_newline(&mut self, line: &str) -> &mut Self {
        self.write_indented_prefix();
        self.output.push_str(line);
        self
    }
    

    /// Writes a postfixed (after main text, before newline) line at the current indentation level and append a newline at the end
    pub fn write_postfixed_line(&mut self, line: &str, postfix: &str) -> &mut Self {
        self.write_indented_prefix();
        self.output.push_str(line);
        self.output.push_str(postfix);
        self.output.push('\n');
        self
    }
    
    /// Writes a line without adding indentation or a newline
    pub fn write_inline(&mut self, line: &str) -> &mut Self {
        self.output.push_str(line);
        self
    }

    /// Writes indentation only, useful if you need to follow this by `write_inline`
    pub fn write_indentation(&mut self) -> &mut Self {
        self.write_indented_prefix();
        self
    }
    

    /// Sets a prefix to be appended before every line written
    pub fn set_prefix(&mut self, prefix: Cow<'static, str>) -> &mut Self {
        self.state.prefix = Some(prefix);
        self
    }

    /// Clears the set prefix 
    pub fn clear_prefix(&mut self) -> &mut Self{
        self.state.prefix = None;
        self
    }

    /// Increases intendation level permamently, does not write to the output yet
    pub fn indent(&mut self) -> &mut Self{
        self.state.indentation_level += 1;
        self
    }

    /// Decrases intendation level permamently, does not write to the output yet
    pub fn dedent(&mut self) -> &mut Self{
        self.state.indentation_level = self.state.indentation_level.checked_sub(1).expect("No indentation to dedent");
        self
    }

    /// Opens parenthesised section
    pub fn open_paren(&mut self) -> &mut Self{
        self.write_line("(");
        self.indent();
        self.state.open_parenthesis += 1;
        self
    } 

    /// Closes parenthesised section
    pub fn close_paren(&mut self) -> &mut Self{
        self.dedent();
        self.write_line(")");
        self.state.open_parenthesis = self.state.open_parenthesis.checked_sub(1).expect("No parenthesis to close");
        self
    } 

    /// Opens braceed section
    pub fn open_brace(&mut self) -> &mut Self{
        self.write_line("{");
        self.indent();
        self.state.open_braces += 1;
        self
    } 

    /// Closes braced section
    pub fn close_brace(&mut self) -> &mut Self{
        self.dedent();
        self.write_line("}");
        self.state.open_braces = self.state.open_braces.checked_sub(1).expect("No brace to close");
        self
    } 

    /// Opens bracketed section
    pub fn open_bracket(&mut self) -> &mut Self{
        self.write_line("[");
        self.indent();
        self.state.open_brackets += 1;
        self
    } 

    /// Closes bracketed section
    pub fn close_bracket(&mut self) -> &mut Self{
        self.dedent();
        self.write_line("]");
        self.state.open_brackets = self.state.open_brackets.checked_sub(1).expect("No bracket to close");
        self
    } 

    /// Consumes self and produces the output string, panics if there is unclosed parenthesis/brackets etc.
    pub fn finish(self) -> String {
        if self.state.open_braces > 0 {
            panic!("{} unclosed braces",self.state.open_braces)
        } else if self.state.open_brackets > 0 {
            panic!("{} unclosed brackets",self.state.open_brackets)
        } else if self.state.open_parenthesis > 0 {
            panic!("{} unclosed parenthesis",self.state.open_parenthesis)
        };

        self.output
    }

}

#[cfg(test)]
mod test {
    use crate::PrettyWriter;

    #[test]
    fn test_indentation(){

        let mut writer = PrettyWriter::new();

        writer
            .write_line("a")
            .indent()
                .write_line("a")
                .indent()
                    .write_line("a")
                    .dedent()
                .write_line("a")
                .dedent()
            .write_line("a");

        assert_eq!(writer.finish(),"a\n\ta\n\t\ta\n\ta\na\n");
    }

    #[test]
    fn test_parenthesis(){

        let mut writer = PrettyWriter::new();

        writer
            .write_line("a")
            .open_paren()
            .open_paren()
            .write_line("a")
            .close_paren()
            .close_paren()
            .write_line("a");

        assert_eq!(writer.finish(),"a\n(\n\t(\n\t\ta\n\t)\n)\na\n");
    }
}