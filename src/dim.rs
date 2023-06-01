//! Dimensions

pub enum Unit {
  Mm,
  Cm,
  In
}

impl  Unit {
  fn to_str(&self) -> &'static str {
    match self {
      Self::Mm => "mm",
      Self::Cm => "cm",
      Self::In => "in",
    }
  }
}

pub struct Dimension {
  unit: Unit,
  margin: Option<f32>,
  width: Option<f32>,
  height: Option<f32>
}

impl Dimension {

  /// American Letter.
  pub fn letter() -> Dimension {
    Dimension {
      unit: Unit::In,
      margin: None,
      width: Some(8.5),
      height: Some(11.0),
    }
  }

  /// A4.
  pub fn a4() -> Dimension {
    Dimension {
      unit: Unit::Mm,
      margin: None,
      width: Some(210.0),
      height: Some(297.0),
    }
  }

  /// Computes inner dimension from outer dimension.
  pub fn inner_dim(self, margin: f32) -> Dimension {

    let twice_margin = 2.0 * margin;

    let opt_calc = |x_opt| match x_opt {
      Some(x) => Some(x - twice_margin),
      None => None
    };

    Dimension { 
      unit: self.unit, 
      margin: Some(margin), 
      width: opt_calc(self.width),
      height: opt_calc(self.height)
    }
  }
}

impl ToString for Dimension {
  fn to_string(&self) -> String {
    let opt_prop = |tag, x_opt| match x_opt {
      Some(x) => format!("--{}: {}{}; ", tag, x, self.unit.to_str()),
      None => "".to_string()
    };

    format!(
      "{}{}{}", 
      opt_prop("margin", self.margin),
      opt_prop("width", self.width),
      opt_prop("height", self.height),
    )
  }
}
