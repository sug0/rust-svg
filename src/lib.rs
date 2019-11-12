use std::io::{self, ErrorKind, Write};

// meta stuff
#[macro_export]
macro_rules! init {
    ($w:expr, $width:expr, $height:expr) => {(|| {
        $w.write_all(b"<?xml version=\"1.0\" encoding=\"utf-8\"?>\n")?;
        write!($w,
r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
     width="{}" height="{}" x="0" y="0">
"#,
        $width, $height)
    })()}
}

#[macro_export]
macro_rules! end {
    ($w:expr) => { $w.write_all(b"</svg>\n") }
}

#[macro_export]
macro_rules! comment {
    ($w:expr, $comment:expr) => { write!($w, "<!-- {} -->\n", $comment) }
}

// types of tags
#[macro_export]
macro_rules! tag {
    ($w:expr, $tag:expr $(, $k:expr => $v:expr)*) => { $crate::tag_helper($w, true, &[$tag $(, $k, $v)*]) }
}

#[macro_export]
macro_rules! block {
    ($w:expr, $tag:expr $(, $k:expr => $v:expr)*) => { $crate::tag_helper($w, false, &[$tag $(, $k, $v)*]) }
}

// groups
#[macro_export]
macro_rules! gbegin {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::block!($w, "g" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! gend {
    ($w:expr) => { $crate::block!($w, "/g") }
}

// text
#[macro_export]
macro_rules! txtbegin {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::block!($w, "text" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! txtwrite {
    ($w:expr, $($arg:tt)*) => { write!($w, $($arg)*) }
}

#[macro_export]
macro_rules! txtend {
    ($w:expr) => { $crate::block!($w, "/text") }
}

// shapes
#[macro_export]
macro_rules! rect {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "rect" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! circle {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "circle" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! ellipse {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "ellipse" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! line {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "line" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! polyline {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "polyline" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! polygon {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "polygon" $(, $k => $v)*) }
}

#[macro_export]
macro_rules! path {
    ($w:expr $(, $k:expr => $v:expr)*) => { $crate::tag!($w, "path" $(, $k => $v)*) }
}

#[inline(always)]
pub fn tag_helper<W: Write>(w: &mut W, endtag: bool, attrs: &[&str]) -> io::Result<()> {
    if attrs.len()%2 == 0 {
        return Err(io::Error::new(ErrorKind::Other, "attrs are even"))
    }

    write!(w, "<{}", attrs[0])?;

    for i in (1..attrs.len()).step_by(2) {
        let (k, v) = (attrs[i], attrs[i+1]);
        write!(w, " {}=\"{}\"", k, v)?;
    }

    write!(w, "{}\n", if endtag {
        "/>"
    } else {
        ">"
    })
}

#[cfg(test)]
mod tests;
