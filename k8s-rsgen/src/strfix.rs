use regex::{Captures, Regex};
use std::borrow::Cow;

pub fn dot_under<S>(sin: S) -> String
where
    S: Into<Cow<'static, str>>,
{
    sin.into().to_mut().replace(".", "_")
}

pub fn nodot<S>(sin: S) -> String
where
    S: Into<Cow<'static, str>>,
{
    sin.into().to_mut().replace(".", "")
}


pub fn camel_snake<S>(sin: S) -> String
    where
    S: Into<Cow<'static, str>>,
{
    let s = dot_under(sin);
    let re = Regex::new(r"[[:upper:]]+").unwrap();
    let res = re.replace_all(&s, |caps: &Captures| {
        format!("_{}", &caps[0].to_lowercase())
    });
    res.into_owned()
}

pub fn fix_name<S>(sin: S) -> String
where
    S: Into<Cow<'static, str>>,
{
    camel_snake(sin)
}
