use std::io::{self, Write, BufWriter, Stderr};

fn run_test<T>(t: T) -> io::Result<()>
    where T: Fn(BufWriter<Stderr>) -> io::Result<()>
{
    let stderr_buffered = BufWriter::new(io::stderr());
    t(stderr_buffered)
}

#[test]
fn svg_document() {
    let r = run_test(|mut w| {
        init!(&mut w, 500, 500)?;
        comment!(&mut w, "this svg file was created by sugo")?;
        comment!(&mut w, "use at your own peril :^)")?;
        gbegin!(&mut w,
          "fill" => "black",
          "transform" => "rotate(-10 50 100)")?;
        rect!(&mut w,
          "x" => "125px",
          "y" => "125px",
          "width" => "250px",
          "height" => "250px")?;
        txtbegin!(&mut w,
          "x" => "50px",
          "y" => "50px",
          "fill" => "red")?;
        txtwrite!(&mut w,
          "wew {}\n", "lad")?;
        txtend!(&mut w)?;
        gend!(&mut w)?;
        end!(&mut w)
    });
    assert!(!r.is_err(), "error occurred during encoding")
}
