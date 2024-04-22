use crate::time_limit::TimeLimit;

#[derive(Clone, Debug)]
pub(crate) struct FrameTime {
    pub(crate) frame: u32,
    pub(crate) time: f32,
}

impl std::ops::Sub for &FrameTime {
    type Output = FrameTime;

    fn sub(self, other: Self) -> Self::Output {
        FrameTime {
            frame: self.frame - other.frame,
            time: self.time - other.time,
        }
    }
}

impl FrameTime {
    pub(crate) fn has_timedout(&self, time_limit: &TimeLimit) -> bool {
        match time_limit {
            TimeLimit::Frames(f) => self.frame > *f,
            TimeLimit::Duration(d) => self.time > d.as_secs_f32(),
        }
    }
}
