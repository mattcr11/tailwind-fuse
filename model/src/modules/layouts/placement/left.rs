use super::*;

#[derive(Clone, Debug)]
pub struct TailwindLeft {
    kind: UnitValue,
}

impl Display for TailwindLeft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.write_negative(f)?;
        self.kind.write_class(f, "left-")
    }
}

// TODO CONFIRM
impl TailwindInstance for TailwindLeft {
    fn collision_id(&self) -> String {
        "left".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}

impl TailwindLeft {
    /// <https://tailwindcss.com/docs/top-right-bottom-left>
    pub fn parse(
        pattern: &[&str],
        arbitrary: &TailwindArbitrary,
        negative: Negative,
    ) -> Result<Self> {
        let kind = get_kind_px_full_auto_fact("left", pattern, arbitrary, negative)?;
        Ok(Self { kind })
    }
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left#syntax>
    pub fn check_valid(mode: &str) -> bool {
        check_valid_auto(mode)
    }
}