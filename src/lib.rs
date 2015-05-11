use std::io::{ Read, Result, Write };

pub type Printer = Box<Fn(u64, u64) -> ()>;

pub struct Progress<R> {
  inner: R,
  printer: Box<Printer>,
  total: u64,
  current: u64,
  lastupdate: u64,
  interval: u64
}

impl <R: Read> Progress<R> {
  pub fn new(r: R, total: u64, printer: Printer) -> Progress<R> {
    Progress {
      inner: r,
      printer: Box::new(printer),
      total: total,
      current: 0,
      lastupdate: 0,
      interval: 0
    }
  }
}

impl <W: Write> Progress<W> {
  pub fn new(w: W, total: u64, printer: Printer) -> Progress<W> {
    Progress {
      inner: w,
      printer: Box::new(printer),
      total: total,
      current: 0,
      lastupdate: 0,
      interval: 0
    }
  }
}

impl<R: Read> Read for Progress<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    let amt = try!(self.inner.read(buf));
    self.current += amt as u64;
    if self.current - self.lastupdate > self.interval {
      match self.printer {
        ref p => p(self.current, self.total)
      };
      self.lastupdate = self.current;
    }
    return Ok(amt)
  }
}

impl<W: Write> Write for Progress<W> {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    let amt = try!(self.inner.write(buf));
    self.current += amt as u64;
    if self.current - self.lastupdate > self.interval {
      match self.printer {
        ref p => p(self.current, self.total)
      };
      self.lastupdate = self.current;
    }
    return Ok(amt)
  }

  fn flush(&mut self) -> Result<()> {
    self.inner.flush()
  }
}


#[test]
fn it_works() {
}
