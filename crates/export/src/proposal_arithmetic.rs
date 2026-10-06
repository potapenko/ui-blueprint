//! Outward rounding bounds for proposal anchor arithmetic, not UI tolerances.
//! Inputs are exact stored f64 values. Exact operations do not widen the interval.
#[derive(Clone, Copy)]
pub(super) struct Interval {
    lower: f64,
    upper: f64,
}
impl Interval {
    pub(super) fn exact(value: f64) -> Self {
        Self {
            lower: value,
            upper: value,
        }
    }
    pub(super) fn half(value: f64) -> Self {
        let half = value * 0.5;
        // Halving is exact except at the subnormal boundary. Preserve that
        // operation's direction of rounding instead of assigning an epsilon.
        Self {
            lower: if half * 2.0 > value {
                half.next_down()
            } else {
                half
            },
            upper: if half * 2.0 < value {
                half.next_up()
            } else {
                half
            },
        }
    }
    fn add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            lower: sum_bounds(self.lower, rhs.lower)?.lower,
            upper: sum_bounds(self.upper, rhs.upper)?.upper,
        })
    }
    fn subtract(self, rhs: Self) -> Option<Self> {
        self.add(Self {
            lower: -rhs.upper,
            upper: -rhs.lower,
        })
    }
    fn intersect(self, rhs: Self) -> Option<Self> {
        let lower = self.lower.max(rhs.lower);
        let upper = self.upper.min(rhs.upper);
        (lower <= upper).then_some(Self { lower, upper })
    }
    fn absolute(self) -> Self {
        if self.lower >= 0.0 {
            self
        } else if self.upper <= 0.0 {
            Self {
                lower: -self.upper,
                upper: -self.lower,
            }
        } else {
            Self {
                lower: 0.0,
                upper: self.upper.max(-self.lower),
            }
        }
    }
    pub(super) fn contains(self, value: f64) -> bool {
        value >= self.lower && value <= self.upper
    }
}

/// TwoSum recovers the rounding residual of one finite addition. Widen only
/// toward the exact result, by its adjacent representable endpoint. This also
/// bounds subtraction, with no scale-dependent or fixed measurement allowance.
fn sum_bounds(a: f64, b: f64) -> Option<Interval> {
    let sum = a + b;
    if !sum.is_finite() {
        return None;
    }
    let recovered_b = sum - a;
    let residual = (a - (sum - recovered_b)) + (b - recovered_b);
    let lower = if residual < 0.0 { sum.next_down() } else { sum };
    let upper = if residual > 0.0 { sum.next_up() } else { sum };
    (lower.is_finite() && upper.is_finite()).then_some(Interval { lower, upper })
}

/// Keep both algebraically equivalent enclosures. Relative arithmetic preserves
/// sub-ULP extents at a shared large base; endpoint arithmetic preserves small
/// distances after a large base cancels its own offset. Intersecting the finite
/// enclosures avoids promoting either path's lost precision into accepted values.
/// If one path overflows, retain the other finite enclosure.
pub(super) fn distance(a: (f64, Interval), b: (f64, Interval)) -> Option<Interval> {
    let relative = Interval::exact(b.0)
        .subtract(Interval::exact(a.0))
        .and_then(|base| b.1.subtract(a.1).and_then(|offset| base.add(offset)));
    let endpoints = Interval::exact(b.0).add(b.1).and_then(|right| {
        Interval::exact(a.0)
            .add(a.1)
            .and_then(|left| right.subtract(left))
    });
    let difference = match (relative, endpoints) {
        (Some(relative), Some(endpoints)) => relative.intersect(endpoints),
        (Some(finite), None) | (None, Some(finite)) => Some(finite),
        (None, None) => None,
    }?;
    Some(difference.absolute())
}
