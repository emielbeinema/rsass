use super::{FileContext, SassItem, parse_scss_file};
use selectors::Selector;
use std::ascii::AsciiExt;
use std::io::{self, Write};
use valueexpression::Value;
use variablescope::{Scope, ScopeImpl};

/// Selected target format.
/// Only formats that are variants of this type are supported by rsass.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputStyle {
    Normal, // TODO What should be the name of this format?
    Compressed,
}

impl OutputStyle {
    /// Write a slice of sass items in this format.
    /// The `file_context` is needed if there are `@import` statements
    /// in the sass file.
    pub fn write_root(&self,
                      items: &[SassItem],
                      file_context: FileContext)
                      -> Result<Vec<u8>, String> {
        let mut globals = ScopeImpl::new();
        let mut result = Vec::new();
        let mut separate = false;
        for item in items {
            self.handle_root_item(item,
                                  &mut globals,
                                  &mut separate,
                                  &file_context,
                                  &mut result);
        }
        if result != b"" && result[result.len() - 1] != b'\n' {
            write!(result, "\n").unwrap();
        }
        if result.is_ascii() {
            Ok(result)
        } else {
            let mut r2 = vec![];
            if self.is_compressed() {
                // Byte order mark U+FEFF as utf-8.
                r2.extend_from_slice(b"\xEF\xBB\xBF");
            } else {
                r2.extend_from_slice(b"@charset \"UTF-8\";\n");
            }
            r2.extend(result);
            Ok(r2)
        }
    }
    fn handle_root_item(&self,
                        item: &SassItem,
                        globals: &mut Scope,
                        separate: &mut bool,
                        file_context: &FileContext,
                        result: &mut Write) {
        match item {
            &SassItem::Rule(ref s, ref b) => {
                if *separate {
                    self.do_indent(result, 0).unwrap();
                } else {
                    *separate = true;
                }
                self.write_rule(s, b, result, globals, None, file_context, 0)
                    .unwrap();
            }
            &SassItem::VariableDeclaration {
                 ref name,
                 ref val,
                 ref default,
                 ref global,
             } => {
                if *default {
                    globals.define_default(name, val, *global);
                } else {
                    globals.define(name, val, *global);
                }
            }
            &SassItem::MixinDeclaration { ref name, ref args, ref body } => {
                globals.define_mixin(name, args, body)
            }
            &SassItem::FunctionDeclaration { ref name, ref func } => {
                globals.define_function(name, func.clone());
            }
            &SassItem::MixinCall { ref name, ref args } => {
                if let Some((m_args, m_body)) = globals.get_mixin(name) {
                    let mut scope = m_args.eval(globals, args);
                    for item in m_body {
                        self.handle_root_item(&item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result);
                    }
                } else {
                    panic!(format!("Unknown mixin {}({:?})", name, args))
                }
            }
            &SassItem::Import(ref name) => {
                let name = globals.evaluate(name);
                if let Value::Literal(ref x, _) = name {
                    if let Some((sub_context, file)) =
                        file_context.find_file(x.as_ref()) {
                        for item in parse_scss_file(&file).unwrap() {
                            self.handle_root_item(&item,
                                                  globals,
                                                  separate,
                                                  &sub_context,
                                                  result);
                        }
                    } else {
                        writeln!(result, "@import url({});", x).unwrap();
                    }
                } else {
                    writeln!(result, "@import {};", name).unwrap();
                }
            }
            &SassItem::Comment(ref c) => {
                if !self.is_compressed() {
                    if *separate {
                        self.do_indent(result, 0).unwrap();
                    } else {
                        *separate = true;
                    }
                    write!(result, "/*{}*/", c).unwrap();
                }
            }
            &SassItem::Property(..) => {
                panic!("Global property not allowed");
            }
            &SassItem::NamespaceRule(..) => {
                panic!("Global namespaced property not allowed");
            }
            &SassItem::AtRule(ref query, ref body) => {
                if *separate {
                    self.do_indent(result, 0).unwrap();
                } else {
                    *separate = true;
                }
                write!(result, "@{}{{", query).unwrap();
                self.do_indent(result, 0).unwrap();
                let mut direct = vec![];
                self.handle_body(&mut direct,
                                 result,
                                 &mut ScopeImpl::sub(globals),
                                 &[Selector::root()],
                                 body,
                                 file_context,
                                 2)
                    .unwrap();
                assert!(direct.is_empty(),
                        "Direct output in @-rule {:?}",
                        query);
                write!(result, "}}").unwrap();
            }
            &SassItem::IfStatement(ref cond, ref do_if, ref do_else) => {
                if globals.evaluate(cond).is_true() {
                    for item in do_if {
                        self.handle_root_item(item,
                                              globals,
                                              separate,
                                              file_context,
                                              result);
                    }
                } else {
                    for item in do_else {
                        self.handle_root_item(item,
                                              globals,
                                              separate,
                                              file_context,
                                              result);
                    }
                }
            }
            &SassItem::Each(ref name, ref values, ref body) => {
                let values = match globals.evaluate(values) {
                    Value::List(v, _) => v,
                    v => vec![v],
                };
                for value in values {
                    let mut scope = ScopeImpl::sub(globals);
                    scope.define(name, &value, false);
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result);
                    }
                }
            }
            &SassItem::For {
                 ref name,
                 ref from,
                 ref to,
                 inclusive,
                 ref body,
             } => {
                let from = globals.evaluate(from).integer_value().unwrap();
                let to = globals.evaluate(to).integer_value().unwrap();
                let to = if inclusive { to + 1 } else { to };
                for value in from..to {
                    let mut scope = ScopeImpl::sub(globals);
                    scope.define(name, &Value::scalar(value), false);
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result);
                    }
                }
            }
            &SassItem::While(ref cond, ref body) => {
                let mut scope = ScopeImpl::sub(globals);
                while scope.evaluate(cond).is_true() {
                    for item in body {
                        self.handle_root_item(item,
                                              &mut scope,
                                              separate,
                                              file_context,
                                              result);
                    }
                }
            }
            &SassItem::Return(_) => {
                panic!("Return not allowed in global context");
            }
            &SassItem::None => (),
        }
    }
    fn write_rule(&self,
                  selectors: &[Selector],
                  body: &[SassItem],
                  out: &mut Write,
                  scope: &mut Scope,
                  parent: Option<&[Selector]>,
                  file_context: &FileContext,
                  indent: usize)
                  -> io::Result<()> {
        let selectors = if let Some(parent) = parent {
            let mut result = Vec::new();
            for p in parent {
                for s in selectors {
                    result.push(p.join(s));
                }
            }
            result
        } else {
            selectors.into()
        };
        let mut direct = Vec::new();
        let mut sub = Vec::new();
        self.handle_body(&mut direct,
                         &mut sub,
                         &mut ScopeImpl::sub(scope),
                         &selectors,
                         body,
                         file_context,
                         indent)?;
        if !direct.is_empty() {
            self.do_indent_no_lf(out, indent)?;
            write!(out,
                   "{}{}{{",
                   self.join_selectors(&selectors),
                   self.opt_space())?;
            if self.is_compressed() && direct.last() == Some(&b';') {
                direct.pop();
            }
            out.write_all(&direct)?;
            self.do_indent(out, indent)?;
            write!(out, "}}")?;
            self.do_indent(out, 0)?;
        }
        out.write_all(&sub)?;
        Ok(())
    }

    fn join_selectors(&self, selectors: &[Selector]) -> String {
        selectors
            .iter()
            .map(|s| if self.is_compressed() {
                     format!("{:#}", s)
                 } else {
                     format!("{}", s)
                 })
            .collect::<Vec<_>>()
            .join(if self.is_compressed() { "," } else { ", " })
    }

    fn handle_body(&self,
                   direct: &mut Vec<u8>,
                   sub: &mut Write,
                   scope: &mut Scope,
                   selectors: &[Selector],
                   body: &[SassItem],
                   file_context: &FileContext,
                   indent: usize)
                   -> io::Result<()> {
        for b in body {
            match b {
                &SassItem::Comment(ref c) => {
                    if !self.is_compressed() {
                        self.do_indent(direct, indent + 2)?;
                        write!(direct, "/*{}*/", c)?;
                    }
                }
                &SassItem::Property(ref name, ref value, ref important) => {
                    self.write_property(direct,
                                        name,
                                        &scope.evaluate(value),
                                        *important,
                                        indent + 2)?;
                }
                &SassItem::NamespaceRule(ref name, ref value, ref body) => {
                    self.write_nsrule(direct,
                                      scope,
                                      name,
                                      value,
                                      body,
                                      indent + 2)?;
                }
                &SassItem::Rule(ref s, ref b) => {
                    self.write_rule(s,
                                    b,
                                    sub,
                                    scope,
                                    Some(selectors),
                                    file_context,
                                    indent)?;
                }
                &SassItem::VariableDeclaration {
                     ref name,
                     ref val,
                     default,
                     global,
                 } => {
                    if default {
                        scope.define_default(name, val, global);
                    } else {
                        scope.define(name, val, global);
                    }
                }
                &SassItem::MixinDeclaration {
                     ref name,
                     ref args,
                     ref body,
                 } => {
                    scope.define_mixin(name, args, body);
                }
                &SassItem::FunctionDeclaration { ref name, ref func } => {
                    scope.define_function(name, func.clone());
                }
                &SassItem::MixinCall { ref name, ref args } => {
                    if let Some((m_args, m_body)) = scope.get_mixin(name) {
                        let mut argscope = m_args.eval(scope, args);
                        self.handle_body(direct,
                                         sub,
                                         &mut argscope,
                                         selectors,
                                         &m_body,
                                         file_context,
                                         indent)?;
                    } else {
                        writeln!(direct,
                                 "/* Unknown mixin {}({:?}) */",
                                 name,
                                 args)
                                .unwrap();
                    }
                }
                &SassItem::Import(ref name) => {
                    let name = scope.evaluate(name);
                    if let Value::Literal(ref x, _) = name {
                        let (sub_context, file) = file_context.file(x.as_ref());
                        let items = parse_scss_file(&file).unwrap();
                        self.handle_body(direct,
                                         sub,
                                         scope,
                                         selectors,
                                         &items,
                                         &sub_context,
                                         0)
                            .unwrap()
                    } else {
                        writeln!(direct, "@import {};", name).unwrap();
                    }
                }
                &SassItem::AtRule(ref query, ref body) => {
                    let mut s1 = vec![];
                    let mut s2 = vec![];
                    self.handle_body(&mut s1,
                                     &mut s2,
                                     &mut ScopeImpl::sub(scope),
                                     selectors,
                                     body,
                                     file_context,
                                     2)
                        .unwrap();

                    write!(sub, "@{}{{", query).unwrap();
                    if !s1.is_empty() {
                        self.do_indent(sub, 2)?;
                        write!(sub,
                               "{}{}{{",
                               self.join_selectors(&selectors),
                               self.opt_space())?;
                        if self.is_compressed() && s1.last() == Some(&b';') {
                            s1.pop();
                        }
                        sub.write_all(&s1)?;
                        self.do_indent(sub, 2)?;
                        write!(sub, "}}")?;
                    }
                    if !s2.is_empty() {
                        self.do_indent(sub, 0)?;
                        sub.write_all(&s2).unwrap();
                    }
                    write!(sub, "}}").unwrap();
                }
                &SassItem::IfStatement(ref cond, ref do_if, ref do_else) => {
                    if scope.evaluate(cond).is_true() {
                        self.handle_body(direct,
                                         sub,
                                         &mut ScopeImpl::sub(scope),
                                         selectors,
                                         do_if,
                                         file_context,
                                         0)
                            .unwrap();
                    } else {
                        self.handle_body(direct,
                                         sub,
                                         &mut ScopeImpl::sub(scope),
                                         selectors,
                                         do_else,
                                         file_context,
                                         0)
                            .unwrap();
                    }
                }
                &SassItem::Each(ref name, ref values, ref body) => {
                    let values = match scope.evaluate(values) {
                        Value::List(v, _) => v,
                        v => vec![v],
                    };
                    for value in values {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name, &value, false);
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)
                            .unwrap();
                    }
                }
                &SassItem::For {
                     ref name,
                     ref from,
                     ref to,
                     inclusive,
                     ref body,
                 } => {
                    let from = scope.evaluate(from).integer_value().unwrap();
                    let to = scope.evaluate(to).integer_value().unwrap();
                    let to = if inclusive { to + 1 } else { to };
                    for value in from..to {
                        let mut scope = ScopeImpl::sub(scope);
                        scope.define(name, &Value::scalar(value), false);
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)
                            .unwrap();
                    }
                }
                &SassItem::While(ref cond, ref body) => {
                    while scope.evaluate(cond).is_true() {
                        let mut scope = ScopeImpl::sub(scope);
                        self.handle_body(direct,
                                         sub,
                                         &mut scope,
                                         selectors,
                                         body,
                                         file_context,
                                         0)
                            .unwrap();
                    }
                }
                &SassItem::Return(_) => {
                    panic!("Return not allowed in global context");
                }
                &SassItem::None => (),
            }
        }
        Ok(())
    }

    fn write_nsrule(&self,
                    out: &mut Write,
                    scope: &mut Scope,
                    name: &str,
                    value: &Value,
                    body: &[SassItem],
                    indent: usize)
                    -> io::Result<()> {
        self.write_property(out, name, &scope.evaluate(value), false, indent)?;
        for item in body {
            match item {
                &SassItem::Property(ref name2, ref value, ref important) => {
                    self.write_property(out,
                                        &format!("{}-{}", name, name2),
                                        &scope.evaluate(value),
                                        *important,
                                        indent)?;
                }
                &SassItem::NamespaceRule(ref name2, ref value, ref body) => {
                    self.write_nsrule(out,
                                      scope,
                                      &format!("{}-{}", name, name2),
                                      value,
                                      body,
                                      indent)?;
                }
                &SassItem::None => (),
                x => panic!("Item {:?} not supported in scoped rule", x),
            }
        }
        Ok(())
    }
    fn write_property(&self,
                      out: &mut Write,
                      name: &str,
                      value: &Value,
                      important: bool,
                      indent: usize)
                      -> io::Result<()> {
        if !value.is_null() {
            self.do_indent(out, indent)?;
            if self.is_compressed() {
                let important = if important { "!important" } else { "" };
                write!(out, "{}:{:#}{};", name, value, important)?;
            } else {
                let important = if important { " !important" } else { "" };
                write!(out, "{}: {}{};", name, value, important)?;
            }
        }
        Ok(())
    }

    fn do_indent(&self, out: &mut Write, steps: usize) -> io::Result<()> {
        if !self.is_compressed() {
            write!(out, "\n")?;
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }
    fn do_indent_no_lf(&self, out: &mut Write, steps: usize) -> io::Result<()> {
        if !self.is_compressed() {
            for _i in 0..steps {
                write!(out, " ")?;
            }
        }
        Ok(())
    }

    fn opt_space(&self) -> &'static str {
        if self.is_compressed() { "" } else { " " }
    }

    fn is_compressed(&self) -> bool {
        self == &OutputStyle::Compressed
    }
}
