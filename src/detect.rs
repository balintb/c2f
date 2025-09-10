// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

use arboard::{Clipboard, ImageData};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    // image: will output to png
    Image,

    // data
    Json,
    Xml,
    Yaml,
    Toml,
    Csv,
    Sql,

    // languages
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    CSharp,
    Cpp,
    C,
    Shell,
    PowerShell,
    Ruby,
    Php,
    Swift,
    Kotlin,

    // markup, styles
    Html,
    Markdown,
    Latex,
    Css,
    Scss,

    // other config
    Dockerfile,
    GitIgnore,
    Makefile,
    DotEnv,
    Ini,

    // plaintext fallback
    PlainText,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ContentType::Image => "Image (PNG)",
            ContentType::Json => "JSON",
            ContentType::Xml => "XML",
            ContentType::Yaml => "YAML",
            ContentType::Toml => "TOML",
            ContentType::Csv => "CSV",
            ContentType::Sql => "SQL",
            ContentType::Rust => "Rust",
            ContentType::Python => "Python",
            ContentType::JavaScript => "JavaScript",
            ContentType::TypeScript => "TypeScript",
            ContentType::Go => "Go",
            ContentType::Java => "Java",
            ContentType::CSharp => "C#",
            ContentType::Cpp => "C++",
            ContentType::C => "C",
            ContentType::Shell => "Shell script",
            ContentType::PowerShell => "PowerShell",
            ContentType::Ruby => "Ruby",
            ContentType::Php => "PHP",
            ContentType::Swift => "Swift",
            ContentType::Kotlin => "Kotlin",
            ContentType::Html => "HTML",
            ContentType::Markdown => "Markdown",
            ContentType::Latex => "LaTeX",
            ContentType::Css => "CSS",
            ContentType::Scss => "SCSS",
            ContentType::Dockerfile => "Dockerfile",
            ContentType::GitIgnore => "Git ignore",
            ContentType::Makefile => "Makefile",
            ContentType::DotEnv => "Environment file",
            ContentType::Ini => "INI config",
            ContentType::PlainText => "Plain text",
        };
        write!(f, "{name}")
    }
}

impl ContentType {
    pub fn extension(&self) -> &'static str {
        match self {
            ContentType::Image => "png",
            ContentType::Json => "json",
            ContentType::Xml => "xml",
            ContentType::Yaml => "yaml",
            ContentType::Toml => "toml",
            ContentType::Csv => "csv",
            ContentType::Sql => "sql",
            ContentType::Rust => "rs",
            ContentType::Python => "py",
            ContentType::JavaScript => "js",
            ContentType::TypeScript => "ts",
            ContentType::Go => "go",
            ContentType::Java => "java",
            ContentType::CSharp => "cs",
            ContentType::Cpp => "cpp",
            ContentType::C => "c",
            ContentType::Shell => "sh",
            ContentType::PowerShell => "ps1",
            ContentType::Ruby => "rb",
            ContentType::Php => "php",
            ContentType::Swift => "swift",
            ContentType::Kotlin => "kt",
            ContentType::Html => "html",
            ContentType::Markdown => "md",
            ContentType::Latex => "tex",
            ContentType::Css => "css",
            ContentType::Scss => "scss",
            ContentType::Dockerfile => "dockerfile",
            ContentType::GitIgnore => "gitignore",
            ContentType::Makefile => "makefile",
            ContentType::DotEnv => "env",
            ContentType::Ini => "ini",
            ContentType::PlainText => "txt",
        }
    }
}

pub enum ClipboardContent {
    Image(Vec<u8>),
    Text(String),
}

pub fn detect_content(
    clipboard: &mut Clipboard,
) -> Result<(ContentType, ClipboardContent), String> {
    if let Ok(img) = clipboard.get_image() {
        let png_bytes = image_to_png(img)?;
        return Ok((ContentType::Image, ClipboardContent::Image(png_bytes)));
    }

    let text = clipboard
        .get_text()
        .map_err(|e| format!("Error reading clipboard: {e}"))?;

    if text.is_empty() {
        return Err("Clipboard is empty".to_string());
    }

    let content_type = detect_text_type(&text);
    Ok((content_type, ClipboardContent::Text(text)))
}

fn image_to_png(img: ImageData) -> Result<Vec<u8>, String> {
    use image::{ImageBuffer, Rgba};

    let width = img.width as u32;
    let height = img.height as u32;
    let bytes = img.bytes;

    // buffer for rgba
    let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, bytes.to_vec())
        .ok_or_else(|| "Failed to create image buffer".to_string())?;

    // png encode
    let mut png_bytes = Vec::new();
    img_buffer
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Failed to encode PNG: {e}"))?;

    Ok(png_bytes)
}

pub fn detect_text_type(text: &str) -> ContentType {
    let trimmed = text.trim();

    // strict parsers first
    if detectors::data::is_json(trimmed) {
        return ContentType::Json;
    }
    if detectors::data::is_xml(trimmed) {
        return ContentType::Xml;
    }

    // config files (before yaml)
    if detectors::special::is_dockerfile(trimmed) {
        return ContentType::Dockerfile;
    }
    if detectors::special::is_gitignore(trimmed) {
        return ContentType::GitIgnore;
    }
    if detectors::special::is_makefile(trimmed) {
        return ContentType::Makefile;
    }

    // yaml after makefile
    if detectors::data::is_yaml(trimmed) {
        return ContentType::Yaml;
    }
    if detectors::data::is_toml(trimmed) {
        return ContentType::Toml;
    }
    if detectors::data::is_csv(trimmed) {
        return ContentType::Csv;
    }
    if detectors::data::is_sql(trimmed) {
        return ContentType::Sql;
    }

    if detectors::special::is_dotenv(trimmed) {
        return ContentType::DotEnv;
    }
    if detectors::special::is_ini(trimmed) {
        return ContentType::Ini;
    }

    // markup
    if detectors::markup::is_html(trimmed) {
        return ContentType::Html;
    }
    if detectors::markup::is_markdown(trimmed) {
        return ContentType::Markdown;
    }
    if detectors::markup::is_latex(trimmed) {
        return ContentType::Latex;
    }

    // languages
    if detectors::lang::is_rust(trimmed) {
        return ContentType::Rust;
    }
    if detectors::lang::is_python(trimmed) {
        return ContentType::Python;
    }
    if detectors::lang::is_typescript(trimmed) {
        return ContentType::TypeScript;
    }
    if detectors::lang::is_javascript(trimmed) {
        return ContentType::JavaScript;
    }
    if detectors::lang::is_go(trimmed) {
        return ContentType::Go;
    }
    if detectors::lang::is_java(trimmed) {
        return ContentType::Java;
    }
    if detectors::lang::is_csharp(trimmed) {
        return ContentType::CSharp;
    }
    if detectors::lang::is_cpp(trimmed) {
        return ContentType::Cpp;
    }
    if detectors::lang::is_c(trimmed) {
        return ContentType::C;
    }
    if detectors::lang::is_shell(trimmed) {
        return ContentType::Shell;
    }
    if detectors::lang::is_powershell(trimmed) {
        return ContentType::PowerShell;
    }
    if detectors::lang::is_ruby(trimmed) {
        return ContentType::Ruby;
    }
    if detectors::lang::is_php(trimmed) {
        return ContentType::Php;
    }
    if detectors::lang::is_swift(trimmed) {
        return ContentType::Swift;
    }
    if detectors::lang::is_kotlin(trimmed) {
        return ContentType::Kotlin;
    }

    if detectors::markup::is_scss(trimmed) {
        return ContentType::Scss;
    }
    if detectors::markup::is_css(trimmed) {
        return ContentType::Css;
    }

    ContentType::PlainText
}

pub mod detectors {
    pub mod data {
        pub fn is_json(text: &str) -> bool {
            (text.starts_with('{') && text.ends_with('}'))
                || (text.starts_with('[') && text.ends_with(']'))
        }

        pub fn is_xml(text: &str) -> bool {
            if text.starts_with("<?xml") {
                return true;
            }

            // html-like content
            let lower = text.to_lowercase();
            if lower.contains("<div")
                || lower.contains("<span")
                || lower.contains("<p>")
                || lower.contains("<body")
                || lower.contains("<html")
                || lower.contains("<!doctype")
                || lower.contains("<h1")
                || lower.contains("<h2")
                || lower.contains("<a ")
                || lower.contains("<img")
            {
                return false; // not html, not xml
            }

            // generic xml
            text.starts_with('<') && text.contains("</")
        }

        pub fn is_yaml(text: &str) -> bool {
            if text.starts_with("---") {
                return true;
            }

            let has_yaml_list = text.lines().any(|l| {
                let trimmed = l.trim();
                trimmed.starts_with("- ") || trimmed.starts_with("* ")
            });

            let has_key_value = text.lines().any(|l| {
                let trimmed = l.trim();
                trimmed.contains(": ") && !trimmed.starts_with("//") && !trimmed.starts_with("#")
            });

            has_key_value || (text.contains(":\n") && has_yaml_list)
        }

        pub fn is_toml(text: &str) -> bool {
            text.contains("[") && text.contains("]") && text.contains(" = ")
        }

        pub fn is_csv(text: &str) -> bool {
            let lines: Vec<&str> = text.lines().take(3).collect();
            lines.len() > 1 && lines.iter().all(|l| l.contains(','))
        }

        pub fn is_sql(text: &str) -> bool {
            let upper = text.to_uppercase();
            upper.contains("SELECT ")
                || upper.contains("INSERT ")
                || upper.contains("UPDATE ")
                || upper.contains("DELETE ")
                || upper.contains("CREATE TABLE")
                || upper.contains("ALTER TABLE")
        }
    }

    pub mod lang {
        pub fn is_rust(text: &str) -> bool {
            text.contains("fn ")
                || text.contains("impl ")
                || text.contains("use ")
                || text.contains("struct ")
                || text.contains("enum ")
                || text.contains("trait ")
                || text.contains("let mut ")
                || text.contains("match ")
        }

        pub fn is_python(text: &str) -> bool {
            text.starts_with("#!/usr/bin/env python")
                || text.starts_with("#!/usr/bin/python")
                || text.contains("import ")
                || text.contains("from ")
                || text.contains("def ")
                || text.contains("class ")
                || text.contains("if __name__")
                || text.contains("print(")
        }

        pub fn is_typescript(text: &str) -> bool {
            text.contains("interface ")
                || text.contains("type ")
                || text.contains(": string")
                || text.contains(": number")
                || text.contains(": boolean")
                || text.contains("enum ")
        }

        pub fn is_javascript(text: &str) -> bool {
            text.contains("const ")
                || text.contains("let ")
                || text.contains("var ")
                || text.contains("function ")
                || text.contains("=>")
                || text.contains("console.log(")
        }

        pub fn is_go(text: &str) -> bool {
            text.contains("package ")
                || text.contains("func ")
                || text.contains("import (")
                || text.contains("var ")
                || text.contains("type ") && text.contains(" struct")
        }

        pub fn is_java(text: &str) -> bool {
            text.contains("public class ")
                || text.contains("private ")
                || text.contains("public static void main")
                || text.contains("import java.")
        }

        pub fn is_csharp(text: &str) -> bool {
            text.contains("using System")
                || text.contains("namespace ")
                || text.contains("public class ")
                || text.contains("static void Main")
        }

        pub fn is_cpp(text: &str) -> bool {
            text.contains("#include <")
                || text.contains("std::")
                || text.contains("cout <<")
                || text.contains("namespace ")
                || text.contains("class ") && text.contains("::")
        }

        pub fn is_c(text: &str) -> bool {
            text.contains("#include <stdio.h>")
                || text.contains("#include <stdlib.h>")
                || text.contains("int main(")
                || text.contains("void ")
        }

        pub fn is_shell(text: &str) -> bool {
            text.starts_with("#!/bin/bash")
                || text.starts_with("#!/bin/sh")
                || text.starts_with("#!/usr/bin/env bash")
                || text.contains("echo ")
                || text.contains("if [")
                || text.contains("for ")
        }

        pub fn is_powershell(text: &str) -> bool {
            text.contains("$PSVersionTable")
                || text.contains("Get-")
                || text.contains("Set-")
                || text.contains("Write-Host")
                || text.starts_with("param(")
        }

        pub fn is_ruby(text: &str) -> bool {
            text.starts_with("#!/usr/bin/env ruby")
                || text.contains("puts ")
                || text.contains("require ")
                || text.contains("def ")
                || text.contains("class ") && text.contains("end")
        }

        pub fn is_php(text: &str) -> bool {
            text.starts_with("<?php")
                || text.contains("<?=")
                || text.contains("echo ")
                || text.contains("function ") && text.contains("$")
        }

        pub fn is_swift(text: &str) -> bool {
            text.contains("import Foundation")
                || text.contains("import UIKit")
                || text.contains("func ")
                || text.contains("var ") && text.contains(": ")
        }

        pub fn is_kotlin(text: &str) -> bool {
            text.contains("fun ")
                || text.contains("val ")
                || text.contains("var ") && text.contains(": ")
                || text.contains("import kotlin.")
        }
    }

    pub mod markup {
        pub fn is_html(text: &str) -> bool {
            text.contains("<!DOCTYPE")
                || text.contains("<html")
                || text.contains("<body")
                || text.contains("<div")
                || text.contains("<span")
                || text.contains("<p>")
                || text.contains("<h1")
                || text.contains("<h2")
                || text.contains("<a ")
                || text.contains("<img")
                || text.contains("</div>")
                || text.contains("</span>")
                || text.contains("</body>")
                || text.contains("</html>")
        }

        pub fn is_markdown(text: &str) -> bool {
            text.starts_with("# ")
                || text.starts_with("## ")
                || text.contains("\n# ")
                || text.contains("\n## ")
                || text.contains("```")
                || (text.contains("[") && text.contains("]("))
                || text
                    .lines()
                    .any(|l| l.starts_with("- ") || l.starts_with("* "))
        }

        pub fn is_latex(text: &str) -> bool {
            text.contains("\\documentclass")
                || text.contains("\\begin{")
                || text.contains("\\section{")
                || text.contains("\\usepackage{")
        }

        pub fn is_css(text: &str) -> bool {
            text.contains("{")
                && text.contains("}")
                && (text.contains("color:")
                    || text.contains("font-")
                    || text.contains("margin:")
                    || text.contains("padding:"))
        }

        pub fn is_scss(text: &str) -> bool {
            text.contains("$") && text.contains(":") && text.contains(";")
                || text.contains("@mixin")
                || text.contains("@include")
        }
    }

    pub mod special {
        pub fn is_dockerfile(text: &str) -> bool {
            text.starts_with("FROM ")
                || text.contains("\nFROM ")
                || text.contains("RUN ")
                || text.contains("CMD ")
                || text.contains("EXPOSE ")
                || text.contains("WORKDIR ")
        }

        pub fn is_gitignore(text: &str) -> bool {
            text.lines()
                .any(|l| l.starts_with("*.") || l.starts_with("/") || l == "node_modules")
        }

        pub fn is_makefile(text: &str) -> bool {
            text.contains(":\n\t")
                || text.contains(".PHONY:")
                || (text
                    .lines()
                    .any(|l| l.ends_with(':') && !l.starts_with('#'))
                    && text.lines().any(|l| l.starts_with('\t')))
        }

        pub fn is_dotenv(text: &str) -> bool {
            let has_assignment = text.lines().any(|l| l.contains('=') && !l.starts_with('#'));
            let all_valid = text
                .lines()
                .filter(|l| !l.is_empty())
                .all(|l| l.starts_with('#') || l.contains('='));

            has_assignment && all_valid
        }

        pub fn is_ini(text: &str) -> bool {
            text.contains("[")
                && text.contains("]")
                && text.lines().any(|l| l.contains(" = ") || l.contains("="))
        }
    }
}

#[cfg(test)]
#[path = "detect_tests.rs"]
mod tests;
