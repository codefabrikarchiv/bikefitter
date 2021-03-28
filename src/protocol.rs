pub mod protocol {
  #[derive(Default, Copy, Debug, Clone)]
  pub struct Datagram {
      pub x: i32,
      pub y: i32,
      pub action: i32,
  }

  #[derive(Default)]
  pub struct Port {
    pub name: String,
    pub index: i32,
  }
}
