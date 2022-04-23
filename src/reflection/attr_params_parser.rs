pub struct AttrParamsParser<'s> {
    line: &'s [u8],

    key_start: usize,
    key_end: usize,
    value_start: usize,
    value_end: usize,
}

impl<'s> AttrParamsParser<'s> {
    pub fn new(line: &'s [u8]) -> Self {
        Self {
            line,
            key_start: 0,
            key_end: 0,
            value_start: 0,
            value_end: 0,
        }
    }

    pub fn find_key_start(&mut self) -> Option<()> {
        for i in self.key_start..self.line.len() {
            if self.line[i] == '(' as u8 {
                continue;
            }

            if self.line[i] == '*' as u8 {
                continue;
            }

            if self.line[i] == ';' as u8 {
                continue;
            }

            if self.line[i] == 32 {
                continue;
            }

            self.key_start = i;
            return Some(());
        }

        None
    }

    fn fine_key_end(&mut self) -> Option<()> {
        for i in self.key_start..self.line.len() {
            let b = self.line[i];

            if b == '=' as u8 || b == 32 {
                self.key_end = i;
                return Some(());
            }
        }

        None
    }

    fn fine_value_start(&mut self) -> Option<()> {
        for i in self.key_end..self.line.len() {
            let b = self.line[i];

            if b == '=' as u8 {
                continue;
            }

            if b == '"' as u8 {
                self.value_start = i + 1;
                return Some(());
            }
        }

        None
    }

    fn fine_value_end(&mut self) -> Option<()> {
        for i in self.value_start..self.line.len() {
            let b = self.line[i];

            if b == '"' as u8 {
                self.value_end = i;
                return Some(());
            }
        }

        None
    }
}

impl<'s> Iterator for AttrParamsParser<'s> {
    type Item = (&'s str, &'s str);

    fn next(&mut self) -> Option<Self::Item> {
        self.key_start = self.value_end + 1;

        self.find_key_start()?;

        self.fine_key_end()?;
        self.fine_value_start()?;
        self.fine_value_end()?;

        let key = &self.line[self.key_start..self.key_end];

        let key = std::str::from_utf8(key).unwrap();

        let value = &self.line[self.value_start..self.value_end];

        let value = std::str::from_utf8(value).unwrap();

        Some((key, value))
    }
}
