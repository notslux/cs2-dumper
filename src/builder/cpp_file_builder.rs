use super::FileBuilder;

use std::io::{Result, Write};

/// A structure representing a builder for C++ header files.
/// The builder implements the `FileBuilder` trait.
#[derive(Clone, Debug, PartialEq)]
pub struct CppFileBuilder;

impl FileBuilder for CppFileBuilder {
    fn extension(&mut self) -> &str {
        "hpp"
    }

    fn write_top_level(&mut self, output: &mut dyn Write) -> Result<()> {
        write!(output, "#pragma once\n\n#include <cstddef>\n\n")
    }

    fn write_namespace(
        &mut self,
        output: &mut dyn Write,
        name: &str,
        comment: Option<&str>,
    ) -> Result<()> {
        let comment = comment.map_or(String::new(), |c| format!(" // {}", c));

        write!(output, "namespace {} {{{}\n", name, comment)
    }

    fn write_variable(
        &mut self,
        output: &mut dyn Write,
        name: &str,
        value: usize,
        comment: Option<&str>,
        indentation: Option<usize>,
    ) -> Result<()> {
        let indentation = " ".repeat(indentation.unwrap_or(4));

        let comment = comment.map_or(String::new(), |c| format!(" // {}", c));

        write!(
            output,
            "{}constexpr std::ptrdiff_t {} = {:#X};{}\n",
            indentation, name, value, comment
        )
    }

    fn write_closure(&mut self, output: &mut dyn Write, eof: bool) -> Result<()> {
        write!(output, "{}", if eof { "}" } else { "}\n\n" })
    }
}
