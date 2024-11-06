#[macro_export]
macro_rules! get_or_err {
    ($this:ident, $key:expr, $err:expr, $msg:expr) => {{
        $this
            .get_model()
            .get_model()
            .get($key)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} definition in conf file",
                    $msg
                )))
            })?
            .get($key)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} section in conf file",
                    $msg
                )))
            })?
    }};
}

#[macro_export]
macro_rules! get_or_err_with_context {
    ($this:ident, $key:expr, $ctx:expr, $err:expr, $msg:expr) => {{
        $this
            .get_model()
            .get_model()
            .get($key)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} definition in conf file",
                    $msg
                )))
            })?
            .get($ctx)
            .ok_or_else(|| {
                $crate::error::Error::from($err(format!(
                    "Missing {} section in conf file",
                    $msg
                )))
            })?
    }};
}

#[macro_export]
macro_rules! register_g_function {
    ($enforcer:ident, $fname:ident, $ast:ident) => {{
        let rm = Arc::clone(&$enforcer.rm);
        let count = $ast.value.matches('_').count();

        match count {
            2 => {
                $enforcer.engine.register_fn(
                    $fname,
                    move |arg1: ImmutableString, arg2: ImmutableString| {
                        rm.read().has_link(&arg1, &arg2, None)
                    },
                );
            }
            3 => {
                $enforcer.engine.register_fn(
                    $fname,
                    move |arg1: ImmutableString,
                          arg2: ImmutableString,
                          arg3: ImmutableString| {
                        rm.read().has_link(&arg1, &arg2, Some(&arg3))
                    },
                );
            }
            _ => {
                $enforcer.engine.register_fn(
                    $fname,
                    move |args: Vec<ImmutableString>| {
                        if args.len() == count {
                            let mut iter = args.iter();
                            let arg1 = iter.next().unwrap();
                            let arg2 = iter.next().unwrap();
                            let domain = if count > 3 {
                                Some(iter.map(|s| s.to_string()).collect::<Vec<_>>().join("_"))
                            } else {
                                iter.next().map(|s| s.as_str())
                            };
                            rm.read().has_link(arg1, arg2, domain.as_deref())
                        } else {
                            false
                        }
                    },
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! push_index_if_explain {
    ($this:ident) => {{
        #[cfg(feature = "explain")]
        if $this.cap > 1 {
            $this.expl.push($this.idx);
        }
    }};
}
