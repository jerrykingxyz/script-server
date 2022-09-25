use std::{
    io::{BufWriter, Write},
    str::from_utf8,
};

use tempfile::NamedTempFile;

use super::{uci_option::UciOptionType, uci_section::UciSection};
use crate::utils::Error;
pub struct UciConfig {
    pub name: String,
    pub sections: Vec<UciSection>,
    pub modified: bool,
}

impl UciConfig {
    pub fn new(name: String) -> UciConfig {
        UciConfig {
            name: name,
            sections: Vec::new(),
            modified: false,
        }
    }

    pub fn write_in(&self, file: &mut NamedTempFile) -> Result<(), Error> {
        let mut buf = BufWriter::new(file);

        for sec in self.sections {
            if sec.name == "" {
                buf.write_fmt(format_args!("\nconfig {}\n", sec.sec_type));
            } else {
                buf.write_fmt(format_args!("\nconfig {} '{}'\n", sec.sec_type, sec.name));
            }

            for opt in sec.options {
                match opt.opt_type {
                    UciOptionType::TypeOption => {
                        buf.write_fmt(format_args!("\toption {} '{}'\n", opt.name, opt.values[0]));
                    }
                    UciOptionType::TypeList => {
                        for v in opt.values {
                            buf.write_fmt(format_args!("\tlist {} '{}'\n", opt.name, v));
                        }
                    }
                }
            }
        }

        buf.write_all(b"\n")?;
        Ok(())
    }

    pub fn get_section_name(&self, section: &UciSection) -> String {
        if section.name != "" {
            return section.name;
        }
        format!("{}[{}]", section.sec_type, self._index(section))
    }

    fn _index(&self, section: &UciSection) -> usize {
        let Some((index, _)) = self
            .sections
            .into_iter()
            .enumerate()
            .filter(|(index, sec)| sec.sec_type == section.sec_type)
            .find(|(index, sec)| *sec == *section);
        index
    }

    pub fn get(&self, name: String) -> Result<Option<&UciSection>, Error> {
        if name.starts_with("@") {
            self._get_unnamed(name)
        } else {
            self._get_named(name)
        }
    }

    fn _get_named(&self, name: String) -> Result<Option<&UciSection>, Error> {
        Ok(self.sections.iter().find(|section| section.name == name))
    }

    fn _unmangle_section_name(&self, section_name: String) -> Result<(String, i32), Error> {
        let len = section_name.len();
        let bytes_section_name = section_name.as_bytes();
        if len < 5 {
            return Err(Error::new(
                "implausible section selector: must be at least 5 characters long".to_string(),
            ));
        };

        if bytes_section_name[0] as char != '@' {
            return Err(Error::new(
                "invalid syntax: section selector must start with @ sign".to_string(),
            ));
        };

        let (mut bra, mut ket) = (0, len - 1);

        for (i, r) in bytes_section_name.iter().enumerate() {
            if i != 0 && *r as char == '@' {
                return Err(Error::new(
                    "invalid syntax: multiple @ signs found".to_string(),
                ));
            };
            if bra > 0 && *r as char == '[' {
                return Err(Error::new(
                    "invalid syntax: multiple open brackets found".to_string(),
                ));
            };
            if i != ket && *r as char == ']' {
                return Err(Error::new(
                    "invalid syntax: multiple closed brackets found".to_string(),
                ));
            };
            if *r as char == '[' {
                bra = i;
            };
        }

        if bra == 0 || bra >= ket {
            return Err(Error::new(
                "invalid syntax: section selector must have format '@type[index]'".to_string(),
            ));
        };

        let sec_type = from_utf8(&bytes_section_name[1..bra]).unwrap().to_string();
        let sec_index = match from_utf8(&bytes_section_name[bra + 1..ket])
            .unwrap()
            .parse::<i32>()
        {
            Ok(num) => num,
            Err(err) => {
                return Err(Error::new(format!(
                    "invalid syntax: index must be numeric: {}",
                    err.to_string()
                )))
            }
        };

        Ok((sec_type, sec_index))
    }

    fn _get_unnamed(&self, name: String) -> Result<Option<&UciSection>, Error> {
        let (sec_type, sec_index) = self._unmangle_section_name(name)?;
        let count = self._count(sec_type);
        let index = if sec_index >= 0 {
            sec_index as i32
        } else {
            count as i32 + sec_index
        };

        if index < 0 || index >= count as i32 {
            return Err(Error::new("invalid name: index out of bounds".to_string()));
        };

        Ok(Some(
            *self
                .sections
                .iter()
                .filter(|sec| sec.sec_type == sec_type)
                .collect::<Vec<&UciSection>>()
                .get(index as usize)
                .unwrap(),
        ))
    }

    fn _count(&self, sec_type: String) -> usize {
        self.sections
            .iter()
            .filter(|sec| sec.sec_type == sec_type)
            .collect::<Vec<&UciSection>>()
            .len()
    }

    pub fn add(&self, section: UciSection) -> &UciSection {
        self.sections.push(section);
        &section
    }

    pub fn merge(&self, section: UciSection) -> &UciSection {
        let mut same_name_sec = self.sections.iter().find(|sec| {
            let section_name = self.get_section_name(&section);
            let cfg_section_name = self.get_section_name(sec);
            cfg_section_name == section_name
        });

        if same_name_sec.is_none() {
            same_name_sec = Some(self.add(section));
        }

        same_name_sec.map(|sec| {
            for option in sec.options {
                sec.merge(option)
            }
        });

        same_name_sec.unwrap()
    }

    pub fn del(&mut self, name: String) {
        let idx = self
            .sections
            .iter()
            .position(|sec| sec.name == name)
            .unwrap();
        self.sections.remove(idx);
    }

    pub fn section_name(&self, section: &UciSection) -> String {
        if section.name != "" {
            return section.name;
        };

        format!("@{}[{}]", section.sec_type, self.index(section).unwrap())
    }

    pub fn index(&self, section: &UciSection) -> Option<usize> {
        self.sections
            .iter()
            .filter(|sec| sec.sec_type == section.sec_type)
            .position(|sec| sec == section)
    }
}