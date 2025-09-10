// Copyright (c) 2025 balintb - https://github.com/balintb/c2f
// Licensed under the MIT License

#[cfg(test)]
mod tests {
    use crate::detect::detectors::*;

    mod data_tests {
        use super::*;

        #[test]
        fn test_json_detection() {
            assert!(data::is_json(r#"{"key": "value"}"#));
            assert!(data::is_json(r#"[1, 2, 3]"#));
            assert!(data::is_json(r#"{"nested": {"key": "value"}}"#));
            assert!(!data::is_json("plain text"));
            assert!(!data::is_json("{incomplete"));
        }

        #[test]
        fn test_xml_detection() {
            assert!(data::is_xml("<?xml version=\"1.0\"?>"));
            assert!(data::is_xml("<root><child></child></root>"));
            assert!(data::is_xml("<tag>content</tag>"));
            assert!(!data::is_xml("plain text"));
            assert!(!data::is_xml("<incomplete"));
        }

        #[test]
        fn test_yaml_detection() {
            assert!(data::is_yaml("---\nkey: value"));
            assert!(data::is_yaml("name: test\nage: 30"));
            assert!(data::is_yaml("items:\n  - one\n  - two"));
            assert!(!data::is_yaml("plain text"));
            assert!(!data::is_yaml("no colon here"));
        }

        #[test]
        fn test_toml_detection() {
            assert!(data::is_toml("[section]\nkey = \"value\""));
            assert!(data::is_toml("[package]\nname = \"test\""));
            assert!(!data::is_toml("plain text"));
            assert!(!data::is_toml("key = value")); // missing section
        }

        #[test]
        fn test_csv_detection() {
            assert!(data::is_csv("name,age\nJohn,30\nJane,25"));
            assert!(data::is_csv("a,b,c\n1,2,3"));
            assert!(!data::is_csv("single line"));
            assert!(!data::is_csv("no,commas\nhere"));
        }

        #[test]
        fn test_sql_detection() {
            assert!(data::is_sql("SELECT * FROM users"));
            assert!(data::is_sql("INSERT INTO table VALUES (1, 2)"));
            assert!(data::is_sql("UPDATE users SET name = 'John'"));
            assert!(data::is_sql("DELETE FROM logs"));
            assert!(data::is_sql("CREATE TABLE test (id INT)"));
            assert!(!data::is_sql("plain text"));
        }
    }

    mod lang_tests {
        use super::*;

        #[test]
        fn test_rust_detection() {
            assert!(lang::is_rust("fn main() { println!(\"Hello\"); }"));
            assert!(lang::is_rust("struct Point { x: i32 }"));
            assert!(lang::is_rust("use std::io;\nlet mut x = 5;"));
            assert!(lang::is_rust("impl Display for Point"));
            assert!(lang::is_rust("enum Color { Red, Green }"));
            assert!(!lang::is_rust("plain text"));
        }

        #[test]
        fn test_python_detection() {
            assert!(lang::is_python("import os\nprint(\"hello\")"));
            assert!(lang::is_python("def function():\n    pass"));
            assert!(lang::is_python("#!/usr/bin/env python\nprint('test')"));
            assert!(lang::is_python("if __name__ == '__main__':\n    main()"));
            assert!(lang::is_python("from datetime import datetime"));
            assert!(lang::is_python("class MyClass:"));
            assert!(!lang::is_python("plain text"));
        }

        #[test]
        fn test_javascript_detection() {
            assert!(lang::is_javascript("const x = 5;"));
            assert!(lang::is_javascript("function test() { return; }"));
            assert!(lang::is_javascript("console.log('hello');"));
            assert!(lang::is_javascript("const fn = () => {}"));
            assert!(lang::is_javascript("let arr = [1, 2, 3]"));
            assert!(lang::is_javascript("var obj = {}"));
            assert!(!lang::is_javascript("plain text"));
        }

        #[test]
        fn test_typescript_detection() {
            assert!(lang::is_typescript("interface User { name: string }"));
            assert!(lang::is_typescript("type ID = number;"));
            assert!(lang::is_typescript("const x: boolean = true;"));
            assert!(lang::is_typescript("enum Color { Red, Green }"));
            assert!(lang::is_typescript(
                "function test(): string { return ''; }"
            ));
            assert!(!lang::is_typescript("plain text"));
        }

        #[test]
        fn test_go_detection() {
            assert!(lang::is_go("package main\nfunc main() {}"));
            assert!(lang::is_go("import (\n    \"fmt\"\n)"));
            assert!(lang::is_go("func test() error { return nil }"));
            assert!(lang::is_go("type Person struct { Name string }"));
            assert!(!lang::is_go("plain text"));
        }

        #[test]
        fn test_java_detection() {
            assert!(lang::is_java("public class Main {}"));
            assert!(lang::is_java("import java.util.*;"));
            assert!(lang::is_java("public static void main(String[] args) {}"));
            assert!(lang::is_java("private int count;"));
            assert!(!lang::is_java("plain text"));
        }

        #[test]
        fn test_csharp_detection() {
            assert!(lang::is_csharp("using System;"));
            assert!(lang::is_csharp("namespace MyApp { }"));
            assert!(lang::is_csharp("public class Program { }"));
            assert!(lang::is_csharp("static void Main() { }"));
            assert!(!lang::is_csharp("plain text"));
        }

        #[test]
        fn test_cpp_detection() {
            assert!(lang::is_cpp("#include <iostream>"));
            assert!(lang::is_cpp("std::cout << \"Hello\";"));
            assert!(lang::is_cpp("namespace ns { }"));
            assert!(lang::is_cpp("class MyClass::method()"));
            assert!(!lang::is_cpp("plain text"));
        }

        #[test]
        fn test_c_detection() {
            assert!(lang::is_c("#include <stdio.h>"));
            assert!(lang::is_c("#include <stdlib.h>"));
            assert!(lang::is_c("int main() { return 0; }"));
            assert!(lang::is_c("void function() { }"));
            assert!(!lang::is_c("plain text"));
        }

        #[test]
        fn test_shell_detection() {
            assert!(lang::is_shell("#!/bin/bash\necho hello"));
            assert!(lang::is_shell("for i in *; do echo $i; done"));
            assert!(lang::is_shell("if [ -f file ]; then echo exists; fi"));
            assert!(lang::is_shell("echo 'test'"));
            assert!(!lang::is_shell("plain text"));
        }

        #[test]
        fn test_powershell_detection() {
            assert!(lang::is_powershell("$PSVersionTable"));
            assert!(lang::is_powershell("Get-Process"));
            assert!(lang::is_powershell("Set-Location"));
            assert!(lang::is_powershell("Write-Host 'Hello'"));
            assert!(lang::is_powershell("param($Name)"));
            assert!(!lang::is_powershell("plain text"));
        }

        #[test]
        fn test_ruby_detection() {
            assert!(lang::is_ruby("#!/usr/bin/env ruby"));
            assert!(lang::is_ruby("puts 'Hello'"));
            assert!(lang::is_ruby("require 'json'"));
            assert!(lang::is_ruby("def method\nend"));
            assert!(lang::is_ruby("class MyClass\nend"));
            assert!(!lang::is_ruby("plain text"));
        }

        #[test]
        fn test_php_detection() {
            assert!(lang::is_php("<?php echo 'Hello'; ?>"));
            assert!(lang::is_php("<?= $variable ?>"));
            assert!(lang::is_php("function test() { echo $x; }"));
            assert!(!lang::is_php("plain text"));
        }

        #[test]
        fn test_swift_detection() {
            assert!(lang::is_swift("import Foundation"));
            assert!(lang::is_swift("import UIKit"));
            assert!(lang::is_swift("func test() -> String"));
            assert!(lang::is_swift("var name: String = \"test\""));
            assert!(!lang::is_swift("plain text"));
        }

        #[test]
        fn test_kotlin_detection() {
            assert!(lang::is_kotlin("fun main() { }"));
            assert!(lang::is_kotlin("val name = \"test\""));
            assert!(lang::is_kotlin("var count: Int = 0"));
            assert!(lang::is_kotlin("import kotlin.math.*"));
            assert!(!lang::is_kotlin("plain text"));
        }
    }

    mod markup_tests {
        use super::*;

        #[test]
        fn test_html_detection() {
            assert!(markup::is_html("<!DOCTYPE html><html></html>"));
            assert!(markup::is_html("<div>content</div>"));
            assert!(markup::is_html("<html><body><p>Text</p></body></html>"));
            assert!(markup::is_html("<span>test</span>"));
            assert!(!markup::is_html("plain text"));
        }

        #[test]
        fn test_markdown_detection() {
            assert!(markup::is_markdown("# Header"));
            assert!(markup::is_markdown("## Subheader"));
            assert!(markup::is_markdown("# Title\n## Subtitle"));
            assert!(markup::is_markdown("- list item\n* another"));
            assert!(markup::is_markdown("[link](url)"));
            assert!(markup::is_markdown("```code```"));
            assert!(!markup::is_markdown("plain text"));
        }

        #[test]
        fn test_latex_detection() {
            assert!(markup::is_latex("\\documentclass{article}"));
            assert!(markup::is_latex("\\begin{document}"));
            assert!(markup::is_latex("\\section{Introduction}"));
            assert!(markup::is_latex("\\usepackage{amsmath}"));
            assert!(!markup::is_latex("plain text"));
        }

        #[test]
        fn test_css_detection() {
            assert!(markup::is_css("body { color: red; }"));
            assert!(markup::is_css(".class { margin: 10px; }"));
            assert!(markup::is_css("#id { padding: 5px; }"));
            assert!(markup::is_css("div { font-size: 14px; }"));
            assert!(!markup::is_css("plain text"));
        }

        #[test]
        fn test_scss_detection() {
            assert!(markup::is_scss("$color: red;"));
            assert!(markup::is_scss("@mixin button { }"));
            assert!(markup::is_scss("@include mixin;"));
            assert!(!markup::is_scss("plain text"));
        }
    }

    mod special_tests {
        use super::*;

        #[test]
        fn test_dockerfile_detection() {
            assert!(special::is_dockerfile("FROM ubuntu\nRUN apt update"));
            assert!(special::is_dockerfile("FROM node:14\nWORKDIR /app"));
            assert!(special::is_dockerfile("EXPOSE 3000"));
            assert!(special::is_dockerfile("CMD [\"node\", \"app.js\"]"));
            assert!(!special::is_dockerfile("plain text"));
        }

        #[test]
        fn test_gitignore_detection() {
            assert!(special::is_gitignore("*.log\nnode_modules\n/dist"));
            assert!(special::is_gitignore("node_modules"));
            assert!(special::is_gitignore("*.pyc\n__pycache__/"));
            assert!(special::is_gitignore("/build"));
            assert!(!special::is_gitignore("plain text"));
        }

        #[test]
        fn test_makefile_detection() {
            assert!(special::is_makefile("target:\n\tcommand"));
            assert!(special::is_makefile(".PHONY: clean"));
            assert!(!special::is_makefile("plain text"));
            assert!(!special::is_makefile("# Markdown Title"));
        }

        #[test]
        fn test_dotenv_detection() {
            assert!(special::is_dotenv("KEY=value\nANOTHER=123"));
            assert!(special::is_dotenv("# Comment\nKEY=value"));
            assert!(special::is_dotenv("DATABASE_URL=postgres://localhost"));
            assert!(!special::is_dotenv("# Markdown Title"));
            assert!(!special::is_dotenv("plain text without equals"));
        }

        #[test]
        fn test_ini_detection() {
            assert!(special::is_ini("[section]\nkey = value"));
            assert!(special::is_ini("[database]\nhost=localhost"));
            assert!(!special::is_ini("plain text"));
        }
    }

    #[test]
    fn test_detect_text_type_priority() {
        use crate::detect::{detect_text_type, ContentType};

        // json should be detected before js
        assert_eq!(detect_text_type(r#"{"key": "value"}"#), ContentType::Json);

        // dockerfile should be detected before Shell
        assert_eq!(
            detect_text_type("FROM ubuntu\nRUN echo test"),
            ContentType::Dockerfile
        );

        assert_eq!(
            detect_text_type("just some plain text"),
            ContentType::PlainText
        );
    }

    #[test]
    fn test_real_world_samples() {
        use crate::detect::{detect_text_type, ContentType};

        // markdown samples
        assert_eq!(
            detect_text_type("# Markdown Title\n\nSome content"),
            ContentType::Markdown
        );
        assert_eq!(detect_text_type("# Markdown Title"), ContentType::Markdown);
        assert_eq!(
            detect_text_type("## Subtitle\n- List item"),
            ContentType::Markdown
        );

        // python samples
        assert_eq!(
            detect_text_type("print('Hello World')"),
            ContentType::Python
        );
        assert_eq!(
            detect_text_type("import sys\nsys.exit(0)"),
            ContentType::Python
        );

        // html samples
        assert_eq!(
            detect_text_type("<!DOCTYPE html>\n<html><body></body></html>"),
            ContentType::Html
        );
        assert_eq!(detect_text_type("<div>Content</div>"), ContentType::Html);

        // sql samples
        assert_eq!(
            detect_text_type("SELECT * FROM users WHERE id = 1"),
            ContentType::Sql
        );
        assert_eq!(
            detect_text_type("CREATE TABLE test (id INT)"),
            ContentType::Sql
        );

        // shell samples
        assert_eq!(
            detect_text_type("#!/bin/bash\necho 'test'"),
            ContentType::Shell
        );
        assert_eq!(
            detect_text_type("echo 'hello'\nfor i in *; do echo $i; done"),
            ContentType::Shell
        );

        // rust samples (that's me!)
        assert_eq!(
            detect_text_type("fn main() {\n    println!(\"Hello\");\n}"),
            ContentType::Rust
        );

        // yaml samples
        assert_eq!(
            detect_text_type("---\nname: test\nvalue: 123"),
            ContentType::Yaml
        );
        assert_eq!(
            detect_text_type("key: value\nanother: test"),
            ContentType::Yaml
        );

        // csv samples
        assert_eq!(
            detect_text_type("name,age,city\nJohn,30,NYC\nJane,25,LA"),
            ContentType::Csv
        );

        // .env samples
        assert_eq!(
            detect_text_type("DATABASE_URL=postgres://localhost\nAPI_KEY=secret"),
            ContentType::DotEnv
        );

        // dockerfile samples
        assert_eq!(
            detect_text_type("FROM node:14\nRUN npm install"),
            ContentType::Dockerfile
        );

        // makefile samples
        assert_eq!(
            detect_text_type("build:\n\tgo build -o app"),
            ContentType::Makefile
        );
        assert_eq!(
            detect_text_type(".PHONY: clean\nclean:\n\trm -rf dist"),
            ContentType::Makefile
        );
    }

    #[test]
    fn test_edge_cases() {
        use crate::detect::{detect_text_type, ContentType};

        // empty strings should not crash
        assert_eq!(detect_text_type(""), ContentType::PlainText);
        assert_eq!(detect_text_type("   "), ContentType::PlainText);

        // single character markdown should still be detected
        assert_eq!(detect_text_type("# H"), ContentType::Markdown);

        // comments that look like markdown but in .env file
        assert_eq!(
            detect_text_type("# This is a comment\nKEY=value"),
            ContentType::DotEnv
        );
    }
}
