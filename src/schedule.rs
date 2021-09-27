use crate::user::User;
use anyhow::Result;
use rand::Rng;

pub trait Schedule {
    fn schedule(&self, members: &Vec<User>) -> Result<Vec<User>>;
}

pub struct TimeSliceSchedule {}

impl TimeSliceSchedule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Schedule for TimeSliceSchedule {
    fn schedule(&self, members: &Vec<User>) -> Result<Vec<User>> {
        Ok(members.clone())
    }
}

pub struct CallAllSchedule {}

impl CallAllSchedule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Schedule for CallAllSchedule {
    fn schedule(&self, members: &Vec<User>) -> Result<Vec<User>> {
        Ok(members.clone())
    }
}

pub struct RandomSchedule {}

impl RandomSchedule {
    pub fn new() -> Self {
        Self {}
    }
}

impl Schedule for RandomSchedule {
    fn schedule(&self, members: &Vec<User>) -> Result<Vec<User>> {
        let mut rng = rand::thread_rng();
        let u = &members[rng.gen_range(0..members.len()) as usize];
        Ok(vec![u.clone()])
    }
}
